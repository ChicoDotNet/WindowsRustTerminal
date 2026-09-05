$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '55a3e9d04dc538af5f641927372c2b530075d936'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI request mode source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_cursor_style.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_request_mode.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_request_mode.h'))
{
    throw 'CSI request mode include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (cursorStylePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_STYLE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (cursorStylePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_CURSOR_STYLE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_request_mode_result requestModePlan{};
    const auto requestModeStatus = terminal_parser_ffi_output_csi_request_mode_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &requestModePlan);
    THROW_HR_IF(E_UNEXPECTED, requestModeStatus != TERMINAL_PARSER_FFI_OK);

    switch (requestModePlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_MODE_REQUEST_MODE:
        if (requestModePlan.private_mode != 0)
        {
            _dispatch->RequestMode(static_cast<DispatchTypes::DECPrivateMode>(requestModePlan.mode));
        }
        else
        {
            _dispatch->RequestMode(static_cast<DispatchTypes::ANSIStandardMode>(requestModePlan.mode));
        }
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_MODE_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (requestModePlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_REQUEST_MODE_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI request mode dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyRequestMode = @"
    case CsiActionCodes::DECRQM_RequestMode:
        _dispatch->RequestMode(DispatchTypes::ANSIStandardMode(parameters.at(0)));
        break;
    case CsiActionCodes::DECRQM_PrivateRequestMode:
        _dispatch->RequestMode(DispatchTypes::DECPrivateMode(parameters.at(0)));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyRequestMode))
{
    throw 'Legacy CSI request mode cases marker mismatch.'
}
$text = $text.Replace($legacyRequestMode, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI request mode source diff.'
}

if ($text.Contains('case CsiActionCodes::DECRQM_RequestMode:') -or $text.Contains('case CsiActionCodes::DECRQM_PrivateRequestMode:'))
{
    throw 'Legacy CSI request mode cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_request_mode_plan'))
{
    throw 'CSI request mode Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI request mode Rust ownership candidate.'
