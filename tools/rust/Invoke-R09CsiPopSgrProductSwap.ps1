$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'b9299ea83abdabb895d6a6b0700b4e92d60ce895'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI pop SGR source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_window_manipulation.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_pop_sgr.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_pop_sgr.h'))
{
    throw 'CSI pop SGR include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (windowManipulationPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_WINDOW_MANIPULATION_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (windowManipulationPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_WINDOW_MANIPULATION_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_pop_sgr_result popSgrPlan{};
    const auto popSgrStatus = terminal_parser_ffi_output_csi_pop_sgr_plan(
        static_cast<uint64_t>(id),
        &popSgrPlan);
    THROW_HR_IF(E_UNEXPECTED, popSgrStatus != TERMINAL_PARSER_FFI_OK);

    switch (popSgrPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_POP:
        _dispatch->PopGraphicsRendition();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (popSgrPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI pop SGR dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyPopSgr = @"
    case CsiActionCodes::XT_PopSgr:
    case CsiActionCodes::XT_PopSgrAlias:
        _dispatch->PopGraphicsRendition();
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyPopSgr))
{
    throw 'Legacy CSI pop SGR case marker mismatch.'
}
$text = $text.Replace($legacyPopSgr, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI pop SGR source diff.'
}

if ($text.Contains('case CsiActionCodes::XT_PopSgr:') -or $text.Contains('case CsiActionCodes::XT_PopSgrAlias:'))
{
    throw 'Legacy CSI pop SGR cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_pop_sgr_plan'))
{
    throw 'CSI pop SGR Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI pop SGR Rust ownership candidate.'
