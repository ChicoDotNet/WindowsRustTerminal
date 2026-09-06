$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'e13c6d0e7216281783d34f2dacbd0536462ef80c'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI rectangular erase source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_column.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_rect_erase.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_rect_erase.h'))
{
    throw 'CSI rectangular erase include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (columnPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (columnPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_NONE)
    {
        _ClearLastChar();
        return true;
    }

    constexpr size_t rectEraseCapacity = 32;
    int32_t rectEraseInput[rectEraseCapacity]{};
    size_t rectEraseInputCount = 0;
    parameters.for_each([&](const auto value) {
        THROW_HR_IF(E_UNEXPECTED, rectEraseInputCount >= rectEraseCapacity);
        rectEraseInput[rectEraseInputCount++] = static_cast<int32_t>(value);
    });

    int32_t rectEraseOutput[rectEraseCapacity]{};
    size_t rectEraseOutputCount = 0;
    uint32_t rectEraseKind = TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE_NONE;
    const auto rectEraseStatus = terminal_parser_ffi_output_csi_rect_erase_values(
        static_cast<uint64_t>(id),
        rectEraseInput,
        rectEraseInputCount,
        rectEraseOutput,
        rectEraseCapacity,
        &rectEraseOutputCount,
        &rectEraseKind);
    THROW_HR_IF(E_UNEXPECTED, rectEraseStatus != TERMINAL_PARSER_FFI_OK);

    if (rectEraseKind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE_NONE)
    {
        THROW_HR_IF(E_UNEXPECTED, rectEraseOutputCount != rectEraseInputCount);
        for (size_t index = 0; index < rectEraseOutputCount; ++index)
        {
            THROW_HR_IF(E_UNEXPECTED, rectEraseOutput[index] != rectEraseInput[index]);
        }

        switch (rectEraseKind)
        {
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE:
            _dispatch->EraseRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0));
            break;
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_SELECTIVE_ERASE:
            _dispatch->SelectiveEraseRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0));
            break;
        default:
            THROW_HR(E_UNEXPECTED);
        }

        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI rectangular erase dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyRectErase = @"
    case CsiActionCodes::DECERA_EraseRectangularArea:
        _dispatch->EraseRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0));
        break;
    case CsiActionCodes::DECSERA_SelectiveEraseRectangularArea:
        _dispatch->SelectiveEraseRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyRectErase))
{
    throw 'Legacy CSI rectangular erase case marker mismatch.'
}
$text = $text.Replace($legacyRectErase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI rectangular erase source diff.'
}

if ($text.Contains('case CsiActionCodes::DECERA_EraseRectangularArea:') -or $text.Contains('case CsiActionCodes::DECSERA_SelectiveEraseRectangularArea:'))
{
    throw 'Legacy CSI rectangular erase cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_rect_erase_values'))
{
    throw 'CSI rectangular erase Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI rectangular erase Rust ownership candidate.'
