$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '5d147d1f161502fada0ea63e234ef5d4fe40d777'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI SGR source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_push_sgr.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_sgr.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_sgr.h'))
{
    throw 'CSI SGR include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
        _dispatch->PushGraphicsRendition(parameters);
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
        _dispatch->PushGraphicsRendition(parameters);
        _ClearLastChar();
        return true;
    }

    constexpr size_t sgrCapacity = 32;
    int32_t sgrInput[sgrCapacity]{};
    size_t sgrInputCount = 0;
    parameters.for_each([&](const auto value) {
        THROW_HR_IF(E_UNEXPECTED, sgrInputCount >= sgrCapacity);
        sgrInput[sgrInputCount++] = static_cast<int32_t>(value);
    });

    int32_t sgrOutput[sgrCapacity]{};
    size_t sgrOutputCount = 0;
    uint32_t sgrMatched = 0;
    const auto sgrStatus = terminal_parser_ffi_output_csi_sgr_values(
        static_cast<uint64_t>(id),
        sgrInput,
        sgrInputCount,
        sgrOutput,
        sgrCapacity,
        &sgrOutputCount,
        &sgrMatched);
    THROW_HR_IF(E_UNEXPECTED, sgrStatus != TERMINAL_PARSER_FFI_OK);

    if (sgrMatched != 0)
    {
        THROW_HR_IF(E_UNEXPECTED, sgrOutputCount != sgrInputCount);
        for (size_t index = 0; index < sgrOutputCount; ++index)
        {
            THROW_HR_IF(E_UNEXPECTED, sgrOutput[index] != sgrInput[index]);
        }

        _dispatch->SetGraphicsRendition(parameters);
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI SGR dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacySgr = @"
    case CsiActionCodes::SGR_SetGraphicsRendition:
        _dispatch->SetGraphicsRendition(parameters);
        break;

"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacySgr))
{
    throw 'Legacy CSI SGR case marker mismatch.'
}
$text = $text.Replace($legacySgr, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI SGR source diff.'
}

if ($text.Contains('case CsiActionCodes::SGR_SetGraphicsRendition:'))
{
    throw 'Legacy CSI SGR case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_sgr_values'))
{
    throw 'CSI SGR Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI SGR Rust ownership candidate.'
