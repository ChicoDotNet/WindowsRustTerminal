$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '9a9d44c60dcb35888b7f34e35bf2d136372b8828'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI DECFRA source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_sgr.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_decfra.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_decfra.h'))
{
    throw 'CSI DECFRA include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
        _dispatch->SetGraphicsRendition(parameters);
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
        _dispatch->SetGraphicsRendition(parameters);
        _ClearLastChar();
        return true;
    }

    constexpr size_t decfraCapacity = 32;
    int32_t decfraInput[decfraCapacity]{};
    size_t decfraInputCount = 0;
    parameters.for_each([&](const auto value) {
        THROW_HR_IF(E_UNEXPECTED, decfraInputCount >= decfraCapacity);
        decfraInput[decfraInputCount++] = static_cast<int32_t>(value);
    });

    int32_t decfraOutput[decfraCapacity]{};
    size_t decfraOutputCount = 0;
    uint32_t decfraMatched = 0;
    const auto decfraStatus = terminal_parser_ffi_output_csi_decfra_values(
        static_cast<uint64_t>(id),
        decfraInput,
        decfraInputCount,
        decfraOutput,
        decfraCapacity,
        &decfraOutputCount,
        &decfraMatched);
    THROW_HR_IF(E_UNEXPECTED, decfraStatus != TERMINAL_PARSER_FFI_OK);

    if (decfraMatched != 0)
    {
        THROW_HR_IF(E_UNEXPECTED, decfraOutputCount != decfraInputCount);
        for (size_t index = 0; index < decfraOutputCount; ++index)
        {
            THROW_HR_IF(E_UNEXPECTED, decfraOutput[index] != decfraInput[index]);
        }

        _dispatch->FillRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2), parameters.at(3).value_or(0), parameters.at(4).value_or(0));
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI DECFRA dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyDecfra = @"
    case CsiActionCodes::DECFRA_FillRectangularArea:
        _dispatch->FillRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2), parameters.at(3).value_or(0), parameters.at(4).value_or(0));
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyDecfra))
{
    throw 'Legacy CSI DECFRA case marker mismatch.'
}
$text = $text.Replace($legacyDecfra, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI DECFRA source diff.'
}

if ($text.Contains('case CsiActionCodes::DECFRA_FillRectangularArea:'))
{
    throw 'Legacy CSI DECFRA case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_decfra_values'))
{
    throw 'CSI DECFRA Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI DECFRA Rust ownership candidate.'
