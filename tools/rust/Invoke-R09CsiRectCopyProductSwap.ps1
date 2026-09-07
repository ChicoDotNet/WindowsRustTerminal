$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '13723c5efa9de2bc821d780d6806ddea84f5913f'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI rectangular copy source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_rect_erase.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_rect_copy.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_rect_copy.h'))
{
    throw 'CSI rectangular copy include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
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

$dispatchReplacement = @"
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

    constexpr size_t rectCopyCapacity = 32;
    int32_t rectCopyInput[rectCopyCapacity]{};
    size_t rectCopyInputCount = 0;
    parameters.for_each([&](const auto value) {
        THROW_HR_IF(E_UNEXPECTED, rectCopyInputCount >= rectCopyCapacity);
        rectCopyInput[rectCopyInputCount++] = static_cast<int32_t>(value);
    });

    int32_t rectCopyOutput[rectCopyCapacity]{};
    size_t rectCopyOutputCount = 0;
    uint32_t rectCopyMatched = 0;
    const auto rectCopyStatus = terminal_parser_ffi_output_csi_rect_copy_values(
        static_cast<uint64_t>(id),
        rectCopyInput,
        rectCopyInputCount,
        rectCopyOutput,
        rectCopyCapacity,
        &rectCopyOutputCount,
        &rectCopyMatched);
    THROW_HR_IF(E_UNEXPECTED, rectCopyStatus != TERMINAL_PARSER_FFI_OK);

    if (rectCopyMatched != 0)
    {
        THROW_HR_IF(E_UNEXPECTED, rectCopyOutputCount != rectCopyInputCount);
        for (size_t index = 0; index < rectCopyOutputCount; ++index)
        {
            THROW_HR_IF(E_UNEXPECTED, rectCopyOutput[index] != rectCopyInput[index]);
        }

        _dispatch->CopyRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0), parameters.at(4), parameters.at(5), parameters.at(6), parameters.at(7));
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI rectangular copy dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyRectCopy = @"
    case CsiActionCodes::DECCRA_CopyRectangularArea:
        _dispatch->CopyRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0), parameters.at(4), parameters.at(5), parameters.at(6), parameters.at(7));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyRectCopy))
{
    throw 'Legacy CSI rectangular copy case marker mismatch.'
}
$text = $text.Replace($legacyRectCopy, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI rectangular copy source diff.'
}

if ($text.Contains('case CsiActionCodes::DECCRA_CopyRectangularArea:'))
{
    throw 'Legacy CSI rectangular copy case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_rect_copy_values'))
{
    throw 'CSI rectangular copy Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI rectangular copy Rust ownership candidate.'
