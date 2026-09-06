$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '6b1d5a4500f45c2f6d933caa93e8c5b21c190ea1'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI push SGR source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_decsca.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_push_sgr.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_push_sgr.h'))
{
    throw 'CSI push SGR include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
        _dispatch->SetCharacterProtectionAttribute(parameters);
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
        _dispatch->SetCharacterProtectionAttribute(parameters);
        _ClearLastChar();
        return true;
    }

    constexpr size_t pushSgrCapacity = 32;
    int32_t pushSgrInput[pushSgrCapacity]{};
    size_t pushSgrInputCount = 0;
    parameters.for_each([&](const auto value) {
        THROW_HR_IF(E_UNEXPECTED, pushSgrInputCount >= pushSgrCapacity);
        pushSgrInput[pushSgrInputCount++] = static_cast<int32_t>(value);
    });

    int32_t pushSgrOutput[pushSgrCapacity]{};
    size_t pushSgrOutputCount = 0;
    uint32_t pushSgrMatched = 0;
    const auto pushSgrStatus = terminal_parser_ffi_output_csi_push_sgr_values(
        static_cast<uint64_t>(id),
        pushSgrInput,
        pushSgrInputCount,
        pushSgrOutput,
        pushSgrCapacity,
        &pushSgrOutputCount,
        &pushSgrMatched);
    THROW_HR_IF(E_UNEXPECTED, pushSgrStatus != TERMINAL_PARSER_FFI_OK);

    if (pushSgrMatched != 0)
    {
        THROW_HR_IF(E_UNEXPECTED, pushSgrOutputCount != pushSgrInputCount);
        for (size_t index = 0; index < pushSgrOutputCount; ++index)
        {
            THROW_HR_IF(E_UNEXPECTED, pushSgrOutput[index] != pushSgrInput[index]);
        }

        _dispatch->PushGraphicsRendition(parameters);
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI push SGR dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyPushSgr = @"
    case CsiActionCodes::XT_PushSgr:
    case CsiActionCodes::XT_PushSgrAlias:
        _dispatch->PushGraphicsRendition(parameters);
        break;

"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyPushSgr))
{
    throw 'Legacy CSI push SGR case marker mismatch.'
}
$text = $text.Replace($legacyPushSgr, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI push SGR source diff.'
}

if ($text.Contains('case CsiActionCodes::XT_PushSgr:') -or $text.Contains('case CsiActionCodes::XT_PushSgrAlias:'))
{
    throw 'Legacy CSI push SGR cases remain after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_push_sgr_values'))
{
    throw 'CSI push SGR Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI push SGR Rust ownership candidate.'
