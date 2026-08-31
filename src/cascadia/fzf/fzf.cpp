#include "pch.h"
#include "fzf.h"
#include "terminal_app_ffi.h"

#undef CharLower
#undef CharUpper

using namespace fzf::matcher;

namespace
{
    static std::vector<UChar32> utf16ToUtf32(std::wstring_view text)
    {
        const UChar* data = reinterpret_cast<const UChar*>(text.data());
        const int32_t dataLen = static_cast<int32_t>(text.size());
        const int32_t cpCount = u_countChar32(data, dataLen);

        std::vector<UChar32> out(cpCount);

        UErrorCode status = U_ZERO_ERROR;
        u_strToUTF32(out.data(), static_cast<int32_t>(out.size()), nullptr, data, dataLen, &status);
        THROW_HR_IF(E_UNEXPECTED, status > U_ZERO_ERROR);

        return out;
    }

    static void foldStringUtf32(std::vector<UChar32>& str)
    {
        for (auto& cp : str)
        {
            cp = u_foldCase(cp, U_FOLD_CASE_DEFAULT);
        }
    }

    static std::wstring serializePattern(const Pattern& pattern)
    {
        std::wstring result;
        for (const auto& term : pattern.terms)
        {
            if (!result.empty())
            {
                result.push_back(L' ');
            }

            for (auto codePoint : term)
            {
                if (codePoint <= 0xffff)
                {
                    result.push_back(static_cast<wchar_t>(codePoint));
                }
                else
                {
                    codePoint -= 0x10000;
                    result.push_back(static_cast<wchar_t>(0xd800 + (codePoint >> 10)));
                    result.push_back(static_cast<wchar_t>(0xdc00 + (codePoint & 0x3ff)));
                }
            }
        }
        return result;
    }
}

Pattern fzf::matcher::ParsePattern(const std::wstring_view patternStr)
{
    Pattern patObj;
    size_t pos = 0;

    while (true)
    {
        const auto beg = patternStr.find_first_not_of(L' ', pos);
        if (beg == std::wstring_view::npos)
        {
            break;
        }

        const auto end = std::min(patternStr.size(), patternStr.find_first_of(L' ', beg));
        const auto word = patternStr.substr(beg, end - beg);
        auto codePoints = utf16ToUtf32(word);
        foldStringUtf32(codePoints);
        patObj.terms.push_back(std::move(codePoints));
        pos = end;
    }

    return patObj;
}

std::optional<MatchResult> fzf::matcher::Match(std::wstring_view text, const Pattern& pattern)
{
    if (pattern.terms.empty())
    {
        return MatchResult{};
    }

    static_assert(sizeof(wchar_t) == sizeof(uint16_t));

    const auto serializedPattern = serializePattern(pattern);
    terminal_app_ffi_fzf_pattern* rawPattern = nullptr;
    auto status = terminal_app_ffi_fzf_pattern_create_utf16(
        reinterpret_cast<const uint16_t*>(serializedPattern.data()),
        serializedPattern.size(),
        &rawPattern);
    THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_APP_FFI_OK || rawPattern == nullptr);

    const std::unique_ptr<terminal_app_ffi_fzf_pattern, decltype(&terminal_app_ffi_fzf_pattern_destroy)> rustPattern{
        rawPattern,
        terminal_app_ffi_fzf_pattern_destroy
    };

    int32_t score = 0;
    uint8_t matched = 0;
    size_t requiredRuns = 0;
    status = terminal_app_ffi_fzf_match_utf16(
        rustPattern.get(),
        reinterpret_cast<const uint16_t*>(text.data()),
        text.size(),
        &score,
        &matched,
        nullptr,
        0,
        &requiredRuns);

    THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_APP_FFI_OK && status != TERMINAL_APP_FFI_BUFFER_TOO_SMALL);
    if (!matched)
    {
        return std::nullopt;
    }

    std::vector<terminal_app_ffi_fzf_run> ffiRuns(requiredRuns);
    if (requiredRuns != 0)
    {
        size_t writtenRuns = 0;
        status = terminal_app_ffi_fzf_match_utf16(
            rustPattern.get(),
            reinterpret_cast<const uint16_t*>(text.data()),
            text.size(),
            &score,
            &matched,
            ffiRuns.data(),
            ffiRuns.size(),
            &writtenRuns);
        THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_APP_FFI_OK || !matched || writtenRuns != ffiRuns.size());
    }

    MatchResult result;
    result.Score = score;
    result.Runs.reserve(ffiRuns.size());
    for (const auto& run : ffiRuns)
    {
        result.Runs.push_back({ run.start, run.end });
    }
    return result;
}
