$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '7b8921540aa63156c9dcae4bcc853eb8ab919acf'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI margins source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_cursor.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_margins.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_margins.h'))
{
    throw 'CSI margins include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$old = @"
    switch (id)
    {
    case CsiActionCodes::DECSTBM_SetTopBottomMargins:
        _dispatch->SetTopBottomScrollingMargins(parameters.at(0).value_or(0), parameters.at(1).value_or(0));
        break;
    case CsiActionCodes::DECSLRM_SetLeftRightMargins:
        // Note that this can also be ANSISYSSC, depending on the state of DECLRMM.
        _dispatch->SetLeftRightScrollingMargins(parameters.at(0).value_or(0), parameters.at(1).value_or(0));
        break;
"@ -replace "`n", "`r`n"

$new = @"
    terminal_parser_ffi_output_csi_margins_result marginsPlan{};
    const auto marginsStatus = terminal_parser_ffi_output_csi_margins_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        static_cast<int32_t>(parameters.at(1).value_or(0)),
        &marginsPlan);
    THROW_HR_IF(E_UNEXPECTED, marginsStatus != TERMINAL_PARSER_FFI_OK);

    switch (marginsPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_MARGINS_TOP_BOTTOM:
        _dispatch->SetTopBottomScrollingMargins(marginsPlan.first, marginsPlan.second);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_MARGINS_LEFT_RIGHT:
        _dispatch->SetLeftRightScrollingMargins(marginsPlan.first, marginsPlan.second);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_MARGINS_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (marginsPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_MARGINS_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
    {
"@ -replace "`n", "`r`n"

if (-not $text.Contains($old))
{
    throw 'CSI margins dispatch block marker mismatch.'
}
$text = $text.Replace($old, $new)

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI margins source diff.'
}

$diff = git diff -- $source
$addedLegacy = $diff -split "`n" | Where-Object { $_ -match '^\+' -and $_ -match 'CsiActionCodes::(DECSTBM_SetTopBottomMargins|DECSLRM_SetLeftRightMargins)' }
if ($addedLegacy)
{
    throw 'Legacy CSI margins cases were reintroduced in the candidate.'
}

if ($text.Contains('case CsiActionCodes::DECSTBM_SetTopBottomMargins:') -or $text.Contains('case CsiActionCodes::DECSLRM_SetLeftRightMargins:'))
{
    throw 'Legacy CSI margins cases remain after candidate rewrite.'
}

Write-Host 'Prepared CRLF-safe CSI margins Rust ownership candidate.'