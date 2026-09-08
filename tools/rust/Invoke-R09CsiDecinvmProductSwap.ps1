$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '78fbbb9e998c27a2c5b6641d2b3f8e83c4081f6b'

$actualBlob = (git rev-parse "HEAD:$source").Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI DECINVM source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source) -replace "`r`n", "`n"

$includeOld = '#include "terminal_parser_ffi_output_csi_decsace.h"' + "`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_decinvm.h"' + "`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_decinvm.h'))
{
    throw 'CSI DECINVM include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (decsacePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

$dispatchReplacement = @"
    if (decsacePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_decinvm_result decinvmPlan{};
    const auto decinvmStatus = terminal_parser_ffi_output_csi_decinvm_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &decinvmPlan);
    THROW_HR_IF(E_UNEXPECTED, decinvmStatus != TERMINAL_PARSER_FFI_OK);

    switch (decinvmPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_INVOKE_MACRO:
        _dispatch->InvokeMacro(decinvmPlan.macro_id);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (decinvmPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_DECINVM_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`r`n", "`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI DECINVM dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCase = @"
    case CsiActionCodes::DECINVM_InvokeMacro:
        _dispatch->InvokeMacro(parameters.at(0).value_or(0));
        break;
"@ -replace "`r`n", "`n"

if (-not $text.Contains($legacyCase))
{
    throw 'Legacy CSI DECINVM case marker mismatch.'
}
$text = $text.Replace($legacyCase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI DECINVM source diff.'
}

if ($text.Contains('case CsiActionCodes::DECINVM_InvokeMacro:'))
{
    throw 'Legacy CSI DECINVM case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_decinvm_plan'))
{
    throw 'CSI DECINVM Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CSI DECINVM Rust ownership candidate.'
