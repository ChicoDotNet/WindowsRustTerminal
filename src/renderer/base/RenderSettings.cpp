// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"

#include "../inc/RenderSettings.hpp"
#include "../base/renderer.hpp"
#include "../../types/inc/ColorFix.hpp"
#include "../../types/inc/colorTable.hpp"
#include "../../../rust/terminal-parser-ffi/include/terminal_parser_ffi_render_attribute_colors.h"

#include <exception>

using namespace Microsoft::Console::Render;
using Microsoft::Console::Utils::InitializeColorTable;

namespace
{
    constexpr uint32_t _toRustRenderMode(const RenderSettings::Mode mode) noexcept
    {
        switch (mode)
        {
        case RenderSettings::Mode::IndexedDistinguishableColors:
            return TERMINAL_PARSER_FFI_RENDER_MODE_INDEXED_DISTINGUISHABLE_COLORS;
        case RenderSettings::Mode::AlwaysDistinguishableColors:
            return TERMINAL_PARSER_FFI_RENDER_MODE_ALWAYS_DISTINGUISHABLE_COLORS;
        case RenderSettings::Mode::IntenseIsBold:
            return TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BOLD;
        case RenderSettings::Mode::IntenseIsBright:
            return TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT;
        case RenderSettings::Mode::ScreenReversed:
            return TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED;
        case RenderSettings::Mode::SynchronizedOutput:
            return TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT;
        }

        return 0;
    }

    void _failFastOnRustPolicyFailure(const terminal_parser_ffi_status status) noexcept
    {
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::terminate();
        }
    }
}

RenderSettings::RenderSettings() noexcept
{
    _failFastOnRustPolicyFailure(terminal_parser_ffi_render_settings_default(&_renderSettingsPolicy));

    InitializeColorTable(_colorTable);

    SetColorTableEntry(TextColor::DEFAULT_FOREGROUND, INVALID_COLOR);
    SetColorTableEntry(TextColor::DEFAULT_BACKGROUND, INVALID_COLOR);
    SetColorTableEntry(TextColor::FRAME_FOREGROUND, INVALID_COLOR);
    SetColorTableEntry(TextColor::FRAME_BACKGROUND, INVALID_COLOR);
    SetColorTableEntry(TextColor::CURSOR_COLOR, INVALID_COLOR);
    SetColorTableEntry(TextColor::SELECTION_BACKGROUND, INVALID_COLOR);

    SetColorAliasIndex(ColorAlias::DefaultForeground, TextColor::DARK_WHITE);
    SetColorAliasIndex(ColorAlias::DefaultBackground, TextColor::DARK_BLACK);
    SetColorAliasIndex(ColorAlias::FrameForeground, TextColor::FRAME_FOREGROUND);
    SetColorAliasIndex(ColorAlias::FrameBackground, TextColor::FRAME_BACKGROUND);

    SaveDefaultSettings();
}

// Routine Description:
// - Saves the current color table and color aliases as the default values, so
//   we can later restore them when a hard reset (RIS) is requested.
void RenderSettings::SaveDefaultSettings() noexcept
{
    _defaultColorTable = _colorTable;
    _defaultColorAliasIndices = _colorAliasIndices;
}

// Routine Description:
// - Resets the render settings to their default values. which is typically
//   what they were set to at startup.
void RenderSettings::RestoreDefaultSettings() noexcept
{
    _colorTable = _defaultColorTable;
    _colorAliasIndices = _defaultColorAliasIndices;
    // DECSCNM and Synchronized Output are the only render mode we need to reset.
    // The others are all user preferences that can't be changed programmatically.
    _failFastOnRustPolicyFailure(terminal_parser_ffi_render_settings_restore_programmable_defaults(&_renderSettingsPolicy));
}

// Routine Description:
// - Updates the specified render mode.
// Arguments:
// - mode - The render mode to change.
// - enabled - Set to true to enable the mode, false to disable it.
void RenderSettings::SetRenderMode(const Mode mode, const bool enabled) noexcept
{
    _failFastOnRustPolicyFailure(terminal_parser_ffi_render_settings_set_mode(
        &_renderSettingsPolicy,
        _toRustRenderMode(mode),
        enabled ? 1u : 0u));
}

// Routine Description:
// - Retrieves the specified render mode.
// Arguments:
// - mode - The render mode to query.
// Return Value:
// - True if the mode is enabled. False if disabled.
bool RenderSettings::GetRenderMode(const Mode mode) const noexcept
{
    uint32_t enabled{};
    _failFastOnRustPolicyFailure(terminal_parser_ffi_render_settings_get_mode(
        &_renderSettingsPolicy,
        _toRustRenderMode(mode),
        &enabled));
    return enabled != 0;
}

// Routine Description:
// - Returns a reference to the active color table array.
const std::array<COLORREF, TextColor::TABLE_SIZE>& RenderSettings::GetColorTable() const noexcept
{
    return _colorTable;
}

// Routine Description:
// - Resets the first 16 color table entries with default values.
void RenderSettings::ResetColorTable() noexcept
{
    InitializeColorTable({ _colorTable.data(), 16 });
}

// Routine Description:
// - Updates the given index in the color table to a new value.
// Arguments:
// - tableIndex - The index of the color to update.
// - color - The new COLORREF to use as that color table value.
void RenderSettings::SetColorTableEntry(const size_t tableIndex, const COLORREF color)
{
    _colorTable.at(tableIndex) = color;
}

// Routine Description:
// - Retrieves the value in the color table at the specified index.
// Arguments:
// - tableIndex - The index of the color to retrieve.
// Return Value:
// - The COLORREF value for the color at that index in the table.
COLORREF RenderSettings::GetColorTableEntry(const size_t tableIndex) const
{
    return _colorTable.at(tableIndex);
}

// Routine Description:
// - Restores all of the xterm-addressable colors to the ones saved in SaveDefaultSettings.
void RenderSettings::RestoreDefaultIndexed256ColorTable()
{
    std::copy_n(_defaultColorTable.begin(), 256, _colorTable.begin());
}

// Routine Description:
// - Restores a color table entry to the value saved in SaveDefaultSettings.
void RenderSettings::RestoreDefaultColorTableEntry(const size_t tableIndex)
{
    _colorTable.at(tableIndex) = _defaultColorTable.at(tableIndex);
}

// Routine Description:
// - Sets the position in the color table for the given color alias and updates the color.
// Arguments:
// - alias - The color alias to update.
// - tableIndex - The new position of the alias in the color table.
// - color - The new COLORREF to assign to that alias.
void RenderSettings::SetColorAlias(const ColorAlias alias, const size_t tableIndex, const COLORREF color)
{
    SetColorAliasIndex(alias, tableIndex);
    SetColorTableEntry(tableIndex, color);
}

// Routine Description:
// - Retrieves the value in the color table of the given color alias.
// Arguments:
// - alias - The color alias to retrieve.
// Return Value:
// - The COLORREF value of the alias.
COLORREF RenderSettings::GetColorAlias(const ColorAlias alias) const
{
    return GetColorTableEntry(GetColorAliasIndex(alias));
}

// Routine Description:
// - Sets the position in the color table for the given color alias.
// Arguments:
// - alias - The color alias to update.
// - tableIndex - The new position of the alias in the color table.
void RenderSettings::SetColorAliasIndex(const ColorAlias alias, const size_t tableIndex) noexcept
{
    if (tableIndex < TextColor::TABLE_SIZE)
    {
        gsl::at(_colorAliasIndices, static_cast<size_t>(alias)) = tableIndex;
    }
}

// Routine Description:
// - Retrieves the position in the color table of the given color alias.
// Arguments:
// - alias - The color alias to retrieve.
// Return Value:
// - The position in the color table where the color is stored.
size_t RenderSettings::GetColorAliasIndex(const ColorAlias alias) const noexcept
{
    return gsl::at(_colorAliasIndices, static_cast<size_t>(alias));
}

void RenderSettings::RestoreDefaultColorAliasIndex(const ColorAlias alias) noexcept
{
    gsl::at(_colorAliasIndices, static_cast<size_t>(alias)) = gsl::at(_defaultColorAliasIndices, static_cast<size_t>(alias));
}

// Routine Description:
// - Calculates the RGB colors of a given text attribute, using the current
//   color table configuration and active render settings.
// Arguments:
// - attr - The TextAttribute to retrieve the colors for.
// Return Value:
// - The color values of the attribute's foreground and background.
std::pair<COLORREF, COLORREF> RenderSettings::GetAttributeColors(const TextAttribute& attr) const noexcept
{
    const auto fgTextColor = attr.GetForeground();
    const auto bgTextColor = attr.GetBackground();

    const auto defaultFgIndex = GetColorAliasIndex(ColorAlias::DefaultForeground);
    const auto defaultBgIndex = GetColorAliasIndex(ColorAlias::DefaultBackground);

    const auto brightenFg = attr.IsIntense() && GetRenderMode(Mode::IntenseIsBright);
    const auto dimFg = attr.IsFaint() || (_renderSettingsPolicy.blink_should_be_faint != 0 && attr.IsBlinking());
    const auto screenReversed = GetRenderMode(Mode::ScreenReversed);

    auto fg = fgTextColor.GetColor(_colorTable, defaultFgIndex, brightenFg);
    auto bg = bgTextColor.GetColor(_colorTable, defaultBgIndex);

    terminal_parser_ffi_render_attribute_colors colors{};
    _failFastOnRustPolicyFailure(terminal_parser_ffi_render_attribute_effects(
        static_cast<uint32_t>(fg),
        static_cast<uint32_t>(bg),
        dimFg ? 1u : 0u,
        attr.IsReverseVideo() ? 1u : 0u,
        screenReversed ? 1u : 0u,
        attr.IsInvisible() ? 1u : 0u,
        &colors));
    fg = static_cast<COLORREF>(colors.foreground);
    bg = static_cast<COLORREF>(colors.background);

    // We intentionally aren't _only_ checking for attr.IsInvisible here, because we also want to
    // catch the cases where the fg was intentionally set to be the same as the bg. In either case,
    // don't adjust the foreground.
    if constexpr (Feature_AdjustIndistinguishableText::IsEnabled())
    {
        const auto indexedDistinguishableColors = GetRenderMode(Mode::IndexedDistinguishableColors);
        const auto alwaysDistinguishableColors = GetRenderMode(Mode::AlwaysDistinguishableColors);
        if (
            (indexedDistinguishableColors || alwaysDistinguishableColors) &&
            fg != bg &&
            (alwaysDistinguishableColors || (fgTextColor.IsDefaultOrLegacy() && bgTextColor.IsDefaultOrLegacy())))
        {
            fg = ColorFix::GetPerceivableColor(fg, bg, 0.5f * 0.5f);
        }
    }

    return { fg, bg };
}

// Routine Description:
// - Calculates the RGBA colors of a given text attribute, using the current
//   color table configuration and active render settings. This differs from
//   GetAttributeColors in that it also sets the alpha color components.
// Arguments:
// - attr - The TextAttribute to retrieve the colors for.
// Return Value:
// - The color values of the attribute's foreground and background.
std::pair<COLORREF, COLORREF> RenderSettings::GetAttributeColorsWithAlpha(const TextAttribute& attr) const noexcept
{
    auto [fg, bg] = GetAttributeColors(attr);

    terminal_parser_ffi_render_attribute_colors colors{
        static_cast<uint32_t>(fg),
        static_cast<uint32_t>(bg),
    };
    _failFastOnRustPolicyFailure(terminal_parser_ffi_render_attribute_alpha(
        colors,
        attr.BackgroundIsDefault() ? 1u : 0u,
        attr.IsReverseVideo() ? 1u : 0u,
        GetRenderMode(Mode::ScreenReversed) ? 1u : 0u,
        attr.IsInvisible() ? 1u : 0u,
        &colors));

    return {
        static_cast<COLORREF>(colors.foreground),
        static_cast<COLORREF>(colors.background),
    };
}

// Routine Description:
// - Calculates the RGB underline color of a given text attribute, using the
//   current color table configuration and active render settings.
// - Returns the current foreground color when the underline color isn't set.
// Arguments:
// - attr - The TextAttribute to retrieve the underline color from.
// Return Value:
// - The color value of the attribute's underline.
COLORREF RenderSettings::GetAttributeUnderlineColor(const TextAttribute& attr) const noexcept
{
    const auto [fg, bg] = GetAttributeColors(attr);
    const auto ulTextColor = attr.GetUnderlineColor();
    if (ulTextColor.IsDefault())
    {
        return fg;
    }

    const auto defaultUlIndex = GetColorAliasIndex(ColorAlias::DefaultForeground);
    auto ul = ulTextColor.GetColor(_colorTable, defaultUlIndex, true);
    if (attr.IsInvisible())
    {
        ul = bg;
    }

    // We intentionally aren't _only_ checking for attr.IsInvisible here, because we also want to
    // catch the cases where the ul was intentionally set to be the same as the bg. In either case,
    // don't adjust the underline color.
    if constexpr (Feature_AdjustIndistinguishableText::IsEnabled())
    {
        const auto alwaysDistinguishableColors = GetRenderMode(Mode::AlwaysDistinguishableColors);
        const auto indexedDistinguishableColors = GetRenderMode(Mode::IndexedDistinguishableColors);
        if (
            ul != bg &&
            (alwaysDistinguishableColors ||
             (indexedDistinguishableColors && ulTextColor.IsDefaultOrLegacy() && attr.GetBackground().IsDefaultOrLegacy())))
        {
            ul = ColorFix::GetPerceivableColor(ul, bg, 0.5f * 0.5f);
        }
    }

    return ul;
}

void RenderSettings::ToggleBlinkRendition() noexcept
{
    _failFastOnRustPolicyFailure(terminal_parser_ffi_render_settings_toggle_blink(&_renderSettingsPolicy));
}
