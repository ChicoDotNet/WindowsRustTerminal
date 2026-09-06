$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '1dc9347eeabe3f5fc4589d5236c219aa1a9f9d48'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI column source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_decfra.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_column.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_column.h'))
{
    throw 'CSI column include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
        _dispatch->FillRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2), parameters.at(3).value_or(0), parameters.at(4).value_or(0));
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
        _dispatch->FillRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2), parameters.at(3).value_or(0), parameters.at(4).value_or(0));
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_column_plan_result columnPlan{};
    const auto columnStatus = terminal_parser_ffi_output_csi_column_plan(
        static_cast<uint64_t>(id),
        static_cast<int32_t>(parameters.at(0).value_or(0)),
        &columnPlan);
    THROW_HR_IF(E_UNEXPECTED, columnStatus != TERMINAL_PARSER_FFI_OK);

    switch (columnPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_INSERT:
        _dispatch->InsertColumn(columnPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_DELETE:
        _dispatch->DeleteColumn(columnPlan.count);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (columnPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI column dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyColumns = @"
    case CsiActionCodes::DECIC_InsertColumn:
        _dispatch->InsertColumn(parameters.at(0));
        break;
    case CsiActionCodes::DECDC_DeleteColumn:
        _dispatch->DeleteColumn(parameters.at(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyColumns))
{
    throw 'Legacy CSI column case marker mismatch.'
}
$text = $text.Replace($legacyColumns, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI column source diff.'
}

if ($text.Contains('case CsiActionCodes::DECIC_InsertColumn:') -or $text.Contains('case CsiActionCodes::DECDC_DeleteColumn:'))
{
    throw 'Legacy CSI column cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_column_plan'))
{
    throw 'CSI column Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI column Rust ownership candidate.'
