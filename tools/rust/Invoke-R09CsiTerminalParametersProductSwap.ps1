$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '9e97dd6bd04bc42a7874dc8ffb764d467a27dc82'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI terminal parameters source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_tab.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_terminal_parameters.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_terminal_parameters.h'))
{
    throw 'CSI terminal parameters include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (tabPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (tabPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_TAB_NONE)
    {
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_terminal_parameters_result terminalParametersPlan{};
    const auto terminalParametersStatus = terminal_parser_ffi_output_csi_terminal_parameters_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &terminalParametersPlan);
    THROW_HR_IF(E_UNEXPECTED, terminalParametersStatus != TERMINAL_PARSER_FFI_OK);

    switch (terminalParametersPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_TERMINAL_PARAMETERS_REQUEST:
        _dispatch->RequestTerminalParameters(terminalParametersPlan.parameter);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_TERMINAL_PARAMETERS_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (terminalParametersPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_TERMINAL_PARAMETERS_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI terminal parameters dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyTerminalParameters = @"
    case CsiActionCodes::DECREQTPARM_RequestTerminalParameters:
        _dispatch->RequestTerminalParameters(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyTerminalParameters))
{
    throw 'Legacy CSI terminal parameters case marker mismatch.'
}
$text = $text.Replace($legacyTerminalParameters, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI terminal parameters source diff.'
}

if ($text.Contains('case CsiActionCodes::DECREQTPARM_RequestTerminalParameters:'))
{
    throw 'Legacy CSI terminal parameters case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_terminal_parameters_plan'))
{
    throw 'CSI terminal parameters Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI terminal parameters Rust ownership candidate.'
