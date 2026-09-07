$ErrorActionPreference = 'Stop'

$source = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$expectedBlob = '978810c679574d4c205a5de701d2bf3660a8e3d3'

$actualBlob = (git hash-object -- $source).Trim()
if ($actualBlob -ne $expectedBlob)
{
    throw "CSI user preference charset source blob drifted: expected $expectedBlob, got $actualBlob"
}

$text = [IO.File]::ReadAllText($source)
if ($text -notmatch "`r`n" -or $text -match "(?<!`r)`n")
{
    throw 'OutputStateMachineEngine.cpp is not canonical CRLF; refusing mechanical rewrite.'
}

$includeOld = '#include "terminal_parser_ffi_output_csi_rect_copy.h"' + "`r`n"
$includeNew = $includeOld + '#include "terminal_parser_ffi_output_csi_user_preference_charset.h"' + "`r`n"
if (-not $text.Contains($includeOld) -or $text.Contains('terminal_parser_ffi_output_csi_user_preference_charset.h'))
{
    throw 'CSI user preference charset include marker mismatch.'
}
$text = $text.Replace($includeOld, $includeNew)

$dispatchMarker = @"
        _dispatch->CopyRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0), parameters.at(4), parameters.at(5), parameters.at(6), parameters.at(7));
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

$dispatchReplacement = @"
        _dispatch->CopyRectangularArea(parameters.at(0), parameters.at(1), parameters.at(2).value_or(0), parameters.at(3).value_or(0), parameters.at(4), parameters.at(5), parameters.at(6), parameters.at(7));
        _ClearLastChar();
        return true;
    }

    terminal_parser_ffi_output_csi_user_preference_charset_result userPreferenceCharsetPlan{};
    const auto userPreferenceCharsetStatus = terminal_parser_ffi_output_csi_user_preference_charset_plan(
        static_cast<uint64_t>(id),
        &userPreferenceCharsetPlan);
    THROW_HR_IF(E_UNEXPECTED, userPreferenceCharsetStatus != TERMINAL_PARSER_FFI_OK);

    switch (userPreferenceCharsetPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_REQUEST:
        _dispatch->RequestUserPreferenceCharset();
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE:
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }

    if (userPreferenceCharsetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE)
    {
        _ClearLastChar();
        return true;
    }

    switch (id)
"@ -replace "`n", "`r`n"

if (-not $text.Contains($dispatchMarker))
{
    throw 'CSI user preference charset dispatch insertion marker mismatch.'
}
$text = $text.Replace($dispatchMarker, $dispatchReplacement)

$legacyCase = @"
    case CsiActionCodes::DECRQUPSS_RequestUserPreferenceSupplementalSet:
        _dispatch->RequestUserPreferenceCharset();
        break;
"@ -replace "`n", "`r`n"

if (-not $text.Contains($legacyCase))
{
    throw 'Legacy CSI user preference charset case marker mismatch.'
}
$text = $text.Replace($legacyCase, '')

[IO.File]::WriteAllText($source, $text, [Text.UTF8Encoding]::new($false))

if ((git diff --numstat -- $source) -notmatch '^\d+\s+\d+\s+src/terminal/parser/OutputStateMachineEngine.cpp$')
{
    throw 'Unexpected CSI user preference charset source diff.'
}

if ($text.Contains('case CsiActionCodes::DECRQUPSS_RequestUserPreferenceSupplementalSet:'))
{
    throw 'Legacy CSI user preference charset case remains after candidate rewrite.'
}

if (-not $text.Contains('terminal_parser_ffi_output_csi_user_preference_charset_plan'))
{
    throw 'CSI user preference charset Rust ownership seam is missing from candidate.'
}

Write-Host 'Prepared CRLF-safe CSI user preference charset Rust ownership candidate.'
