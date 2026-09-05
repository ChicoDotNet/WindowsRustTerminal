$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '2fd7ac5b303367d07713f6ad989268f79e5aba9c'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI erase source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_mode.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_erase.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_erase.h'))
{
    throw 'CSI erase include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
    if (modePlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
    if (modePlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    constexpr size_t erasePlanCapacity = 32;
    int32_t eraseParameters[erasePlanCapacity]{};
    size_t eraseParameterCount = 0;
    parameters.for_each([&](const auto eraseType) {
        THROW_HR_IF(E_UNEXPECTED, eraseParameterCount >= erasePlanCapacity);
        eraseParameters[eraseParameterCount++] = static_cast<int32_t>(eraseType);
    });

    terminal_parser_ffi_output_csi_erase_result erasePlans[erasePlanCapacity]{};
    size_t erasePlanCount = 0;
    const auto eraseStatus = terminal_parser_ffi_output_csi_erase_plans(
        static_cast<uint64_t>(id),
        eraseParameters,
        eraseParameterCount,
        erasePlans,
        erasePlanCapacity,
        &erasePlanCount);
    THROW_HR_IF(E_UNEXPECTED, eraseStatus != TERMINAL_PARSER_FFI_OK);

    for (size_t index = 0; index < erasePlanCount; ++index)
    {
        const auto& erasePlan = erasePlans[index];
        const auto eraseType = static_cast<DispatchTypes::EraseType>(erasePlan.value);

        switch (erasePlan.kind)
        {
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_DISPLAY:
            _dispatch->EraseInDisplay(eraseType);
            break;
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_SELECTIVE_DISPLAY:
            _dispatch->SelectiveEraseInDisplay(eraseType);
            break;
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_LINE:
            _dispatch->EraseInLine(eraseType);
            break;
        case TERMINAL_PARSER_FFI_OUTPUT_CSI_ERASE_SELECTIVE_LINE:
            _dispatch->SelectiveEraseInLine(eraseType);
            break;
        default:
            THROW_HR(E_UNEXPECTED);
        }
    }

    if (erasePlanCount != 0)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI erase dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyErase = @"
    case CsiActionCodes::ED_EraseDisplay:
        parameters.for_each([&](const auto eraseType) {
            _dispatch->EraseInDisplay(eraseType);
        });
        break;
    case CsiActionCodes::DECSED_SelectiveEraseDisplay:
        parameters.for_each([&](const auto eraseType) {
            _dispatch->SelectiveEraseInDisplay(eraseType);
        });
        break;
    case CsiActionCodes::EL_EraseLine:
        parameters.for_each([&](const auto eraseType) {
            _dispatch->EraseInLine(eraseType);
        });
        break;
    case CsiActionCodes::DECSEL_SelectiveEraseLine:
        parameters.for_each([&](const auto eraseType) {
            _dispatch->SelectiveEraseInLine(eraseType);
        });
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyErase))
{
    throw 'Legacy CSI erase cases marker mismatch.'
}
$text = $text.Replace($legacyErase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI erase source diff.'
}

$legacyCases = @(
    'case CsiActionCodes::ED_EraseDisplay:',
    'case CsiActionCodes::DECSED_SelectiveEraseDisplay:',
    'case CsiActionCodes::EL_EraseLine:',
    'case CsiActionCodes::DECSEL_SelectiveEraseLine:'
)
foreach ($legacyCase in $legacyCases)
{
    if ($text.Contains($legacyCase))
    {
        throw "Legacy CSI erase case remains after candidate rewrite: $legacyCase"
    }
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_erase_plans'))
{
    throw 'CSI erase Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI erase Rust ownership candidate.'
