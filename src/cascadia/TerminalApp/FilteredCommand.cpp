// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "pch.h"
#include "CommandPalette.h"
#include "fzf/fzf.h"
#include "terminal_app_ffi.h"

#include "FilteredCommand.g.cpp"

using namespace winrt;
using namespace winrt::TerminalApp;
using namespace winrt::Windows::UI::Core;
using namespace winrt::Windows::UI::Xaml;
using namespace winrt::Windows::System;
using namespace winrt::Windows::Foundation;
using namespace winrt::Windows::Foundation::Collections;
using namespace winrt::Microsoft::Terminal::Settings::Model;

namespace winrt::TerminalApp::implementation
{
    namespace
    {
        std::wstring _serializePattern(const fzf::matcher::Pattern& pattern)
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

        std::shared_ptr<terminal_app_ffi_fzf_pattern> _createRustPattern(const fzf::matcher::Pattern& pattern)
        {
            if (pattern.terms.empty())
            {
                return {};
            }

            static_assert(sizeof(wchar_t) == sizeof(uint16_t));
            const auto serialized = _serializePattern(pattern);
            terminal_app_ffi_fzf_pattern* rawPattern = nullptr;
            const auto status = terminal_app_ffi_fzf_pattern_create_utf16(
                reinterpret_cast<const uint16_t*>(serialized.data()),
                serialized.size(),
                &rawPattern);
            THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_APP_FFI_OK || rawPattern == nullptr);

            return { rawPattern, [](terminal_app_ffi_fzf_pattern* value) noexcept {
                        if (value)
                        {
                            terminal_app_ffi_fzf_pattern_destroy(value);
                        }
                    } };
        }
    }

    // This class is a wrapper of IPaletteItem, that is used as an item of a filterable list in CommandPalette.
    // It manages a highlighted text that is computed by matching search filter characters to item name
    FilteredCommand::FilteredCommand(const winrt::TerminalApp::IPaletteItem& item) :
        _Item{ item }, _Weight{ 0 }
    {
        // Recompute the highlighted name if the item name changes
        // Our Item will not change, so we don't need to update the revoker if it does.
        _itemChangedRevoker = _Item.as<winrt::Windows::UI::Xaml::Data::INotifyPropertyChanged>().PropertyChanged(winrt::auto_revoke, [=](auto& /*sender*/, auto& e) {
            const auto property{ e.PropertyName() };
            if (property == L"Name")
            {
                _update();
            }
            else if (property == L"Subtitle")
            {
                _update();
                PropertyChanged.raise(*this, winrt::Windows::UI::Xaml::Data::PropertyChangedEventArgs{ L"HasSubtitle" });
            }
        });
    }

    void FilteredCommand::UpdateFilter(std::shared_ptr<fzf::matcher::Pattern> pattern)
    {
        // If the filter was not changed we want to prevent the re-computation of matching
        // that might result in triggering a notification event
        if (pattern != _pattern)
        {
            _pattern = pattern;
            _rustPattern = pattern ? _createRustPattern(*pattern) : nullptr;
            _update();
        }
    }

    bool FilteredCommand::HasSubtitle()
    {
        return !_Item.Subtitle().empty();
    }

    static std::tuple<std::vector<winrt::TerminalApp::HighlightedRun>, int32_t> _matchedSegmentsAndWeight(const std::shared_ptr<terminal_app_ffi_fzf_pattern>& pattern, const winrt::hstring& haystack)
    {
        std::vector<winrt::TerminalApp::HighlightedRun> segments;
        int32_t weight = 0;

        if (pattern)
        {
            static_assert(sizeof(wchar_t) == sizeof(uint16_t));
            uint8_t matched = 0;
            size_t requiredRuns = 0;
            auto status = terminal_app_ffi_fzf_match_utf16(
                pattern.get(),
                reinterpret_cast<const uint16_t*>(haystack.c_str()),
                haystack.size(),
                &weight,
                &matched,
                nullptr,
                0,
                &requiredRuns);

            THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_APP_FFI_OK && status != TERMINAL_APP_FFI_BUFFER_TOO_SMALL);
            if (!matched)
            {
                return { {}, 0 };
            }

            if (requiredRuns != 0)
            {
                std::vector<terminal_app_ffi_fzf_run> runs(requiredRuns);
                size_t writtenRuns = 0;
                status = terminal_app_ffi_fzf_match_utf16(
                    pattern.get(),
                    reinterpret_cast<const uint16_t*>(haystack.c_str()),
                    haystack.size(),
                    &weight,
                    &matched,
                    runs.data(),
                    runs.size(),
                    &writtenRuns);
                THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_APP_FFI_OK || !matched || writtenRuns != runs.size());

                segments.resize(runs.size());
                std::transform(runs.begin(), runs.end(), segments.begin(), [](const auto& run) -> winrt::TerminalApp::HighlightedRun {
                    return { run.start, run.end };
                });
            }
        }
        return { std::move(segments), weight };
    }

    void FilteredCommand::_update()
    {
        auto itemName = _Item.Name();
        auto [segments, weight] = _matchedSegmentsAndWeight(_rustPattern, itemName);
        decltype(segments) subtitleSegments;

        if (HasSubtitle())
        {
            auto itemSubtitle = _Item.Subtitle();
            int32_t subtitleWeight = 0;
            std::tie(subtitleSegments, subtitleWeight) = _matchedSegmentsAndWeight(_rustPattern, itemSubtitle);
            weight = std::max(weight, subtitleWeight);
        }

        if (segments.empty())
        {
            NameHighlights(nullptr);
        }
        else
        {
            NameHighlights(winrt::single_threaded_vector(std::move(segments)));
        }

        if (subtitleSegments.empty())
        {
            SubtitleHighlights(nullptr);
        }
        else
        {
            SubtitleHighlights(winrt::single_threaded_vector(std::move(subtitleSegments)));
        }

        Weight(weight);
    }

    // Function Description:
    // - Implementation of Compare for FilteredCommand interface.
    // Compares first instance of the interface with the second instance, first by weight, then by name.
    // In the case of a tie prefers the first instance.
    // Arguments:
    // - other: another instance of FilteredCommand interface
    // Return Value:
    // - Returns true if the first is "bigger" (aka should appear first)
    int FilteredCommand::Compare(const winrt::TerminalApp::FilteredCommand& first, const winrt::TerminalApp::FilteredCommand& second)
    {
        auto firstWeight{ first.Weight() };
        auto secondWeight{ second.Weight() };

        if (firstWeight == secondWeight)
        {
            const auto firstName = first.Item().Name();
            const auto secondName = second.Item().Name();
            return til::compare_linguistic_insensitive(firstName, secondName) < 0;
        }

        return firstWeight > secondWeight;
    }
}
