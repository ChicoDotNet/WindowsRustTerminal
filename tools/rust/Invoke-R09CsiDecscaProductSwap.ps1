$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = 'c8094df19cd296143330a934debe9ae3ee554344'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI DECSCA source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_pop_sgr.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_decsca.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_decsca.h'))
{
    throw 'CSI DECSCA include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (popSgrPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (popSgrPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_NONE)
    {
        _ClearLastChar();
        return true;
    }

    constexpr size_t decscaCapacity = 32;
    int32_t decscaInput[decscaCapacity]{};
    size_t decscaInputCount = 0;
    parameters.for_each([&](const auto value) {
        THROW_HR_IF(E_UNEXPECTED, decscaInputCount >= decscaCapacity);
        decscaInput[decscaInputCount++] = static_cast<int32_t>(value);
    });

    int32_t decscaOutput[decscaCapacity]{};
    size_t decscaOutputCount = 0;
    const auto decscaStatus = terminal_parser_ffi_output_csi_decsca_values(
        static_cast<uint64_t>(id),
        decscaInput,
        decscaInputCount,
        decscaOutput,
        decscaCapacity,
        &decscaOutputCount);
    THROW_HR_IF(E_UNEXPECTED, decscaStatus != TERMINAL_PARSER_FFI_OK);

    if (decscaOutputCount != 0)
    {
        THROW_HR_IF(E_UNEXPECTED, decscaOutputCount != decscaInputCount);
        for (size_t index = 0; index < decscaOutputCount; ++index)
        {
            THROW_HR_IF(E_UNEXPECTED, decscaOutput[index] != decscaInput[index]);
        }

        _dispatch->SetCharacterProtectionAttribute(parameters);
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI DECSCA dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyDecsca = @"
    case CsiActionCodes::DECSCA_SetCharacterProtectionAttribute:
        _dispatch->SetCharacterProtectionAttribute(parameters);
        break;

"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyDecsca))
{
    throw 'Legacy CSI DECSCA case marker mismatch.'
}
$text = $text.Replace($legacyDecsca, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI DECSCA source diff.'
}

if ($text.Contains('case CsiActionCodes::DECSCA_SetCharacterProtectionAttribute:'))
{
    throw 'Legacy CSI DECSCA case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_decsca_values'))
{
    throw 'CSI DECSCA Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI DECSCA Rust ownership candidate.'
