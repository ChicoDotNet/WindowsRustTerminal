$ErrorActionPreference = 'Stop'

$owner = Get-Content -Raw -LiteralPath 'rust/terminal-renderer/src/render_settings_policy.rs'
$ffi = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/src/render_settings.rs'
$header = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/include/terminal_parser_ffi_render_settings.h'
$probe = Get-Content -Raw -LiteralPath 'tools/rust/R09RenderSettingsAbiProbe.hpp'
$attributeHeader = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/include/terminal_parser_ffi_render_attribute_colors.h'
$attributeProbe = Get-Content -Raw -LiteralPath 'tools/rust/R09RenderAttributeColorsAbiProbe.hpp'
$aggregate = Get-Content -Raw -LiteralPath 'tools/rust/R09ControlCharacterAbiProbe.cpp'
$productHeader = Get-Content -Raw -LiteralPath 'src/renderer/inc/RenderSettings.hpp'
$product = Get-Content -Raw -LiteralPath 'src/renderer/base/RenderSettings.cpp'
$buildTargets = Get-Content -Raw -LiteralPath 'Directory.Build.targets'

$requiredOwner = @(
    'pub enum RenderMode',
    'IntenseIsBright',
    'ScreenReversed',
    'SynchronizedOutput',
    'pub struct RenderSettingsPolicy',
    'modes: RenderMode::IntenseIsBright.mask()',
    'pub fn set_mode(&mut self, mode: RenderMode, enabled: bool)',
    'pub const fn mode(self, mode: RenderMode) -> bool',
    'pub fn restore_programmable_defaults(&mut self)',
    'pub const fn toggle_blink_rendition(&mut self)'
)
foreach ($needle in $requiredOwner) {
    if (-not $owner.Contains($needle)) { throw "Render settings Rust owner evidence missing: $needle" }
}

$requiredFfi = @(
    'terminal_parser_ffi_render_settings_default',
    'terminal_parser_ffi_render_settings_set_mode',
    'terminal_parser_ffi_render_settings_get_mode',
    'terminal_parser_ffi_render_settings_restore_programmable_defaults',
    'terminal_parser_ffi_render_settings_toggle_blink',
    'RenderSettingsPolicy::default()',
    'policy.restore_programmable_defaults()',
    'policy.toggle_blink_rendition()',
    'return FfiStatus::InvalidArgument'
)
foreach ($needle in $requiredFfi) {
    if (-not $ffi.Contains($needle)) { throw "Render settings FFI evidence missing: $needle" }
}

$requiredAbi = @(
    'TERMINAL_PARSER_FFI_RENDER_MODE_INDEXED_DISTINGUISHABLE_COLORS = 1',
    'TERMINAL_PARSER_FFI_RENDER_MODE_ALWAYS_DISTINGUISHABLE_COLORS = 2',
    'TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BOLD = 3',
    'TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT = 4',
    'TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED = 5',
    'TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT = 6',
    'uint32_t modes;',
    'uint32_t blink_should_be_faint;',
    'static_assert(sizeof(terminal_parser_ffi_render_settings_state) == 8);'
)
foreach ($needle in $requiredAbi) {
    if (-not $header.Contains($needle)) { throw "Render settings C ABI contract evidence missing: $needle" }
}

$requiredWitness = @(
    'render_settings_policy_replay()',
    'TERMINAL_PARSER_FFI_RENDER_MODE_INTENSE_IS_BRIGHT, true',
    'TERMINAL_PARSER_FFI_RENDER_MODE_ALWAYS_DISTINGUISHABLE_COLORS, true',
    'TERMINAL_PARSER_FFI_RENDER_MODE_SCREEN_REVERSED, false',
    'TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT, false',
    'state.blink_should_be_faint != 1',
    'terminal_parser_ffi_render_settings_set_mode(&state, 0, 1)',
    'terminal_parser_ffi_render_settings_set_mode(&state, 7, 1)',
    'terminal_parser_ffi_render_settings_default(nullptr)',
    'render_attribute_colors_replay()'
)
foreach ($needle in $requiredWitness) {
    if (-not $probe.Contains($needle)) { throw "Render settings native contract witness missing: $needle" }
}

$requiredAttributeAbi = @(
    'terminal_parser_ffi_render_attribute_colors',
    'terminal_parser_ffi_render_attribute_effects(',
    'terminal_parser_ffi_render_attribute_effects_from_rendition(',
    'terminal_parser_ffi_render_attribute_alpha(',
    'terminal_parser_ffi_render_underline_invisibility(',
    'uint32_t* out_underline',
    'static_assert(sizeof(terminal_parser_ffi_render_attribute_colors) == 8);'
)
foreach ($needle in $requiredAttributeAbi) {
    if (-not $attributeHeader.Contains($needle)) { throw "Render attribute color C ABI evidence missing: $needle" }
}

$requiredAttributeWitness = @(
    'render_attribute_colors_replay()',
    'terminal_parser_ffi_render_attribute_effects(',
    'terminal_parser_ffi_render_attribute_effects_from_rendition(',
    'colors.foreground != 0x00302010',
    'colors.background != 0x00302010',
    'terminal_parser_ffi_render_attribute_alpha(',
    'colors.foreground != 0xFF112233',
    'terminal_parser_ffi_render_underline_invisibility(',
    'underline != 0x00445566',
    'underline != 0x00112233',
    'TERMINAL_PARSER_FFI_INVALID_ARGUMENT',
    'nullptr'
)
foreach ($needle in $requiredAttributeWitness) {
    if (-not $attributeProbe.Contains($needle)) { throw "Render attribute color native replay evidence missing: $needle" }
}

$requiredAggregate = @(
    '#include "R09RenderSettingsAbiProbe.hpp"',
    'const bool renderSettingsPolicyOk = r09::render_settings_policy_replay();',
    '!renderSettingsPolicyOk'
)
foreach ($needle in $requiredAggregate) {
    if (-not $aggregate.Contains($needle)) { throw "Render settings aggregate replay evidence missing: $needle" }
}

$requiredProductHeader = @(
    'terminal_parser_ffi_render_settings_state _renderSettingsPolicy{};'
)
foreach ($needle in $requiredProductHeader) {
    if (-not $productHeader.Contains($needle)) { throw "Render settings product state route missing: $needle" }
}

$forbiddenProductHeader = @(
    'til::enumset<Mode> _renderMode',
    'bool _blinkShouldBeFaint'
)
foreach ($needle in $forbiddenProductHeader) {
    if ($productHeader.Contains($needle)) { throw "Legacy C++ render settings ownership returned: $needle" }
}

$requiredProduct = @(
    'terminal_parser_ffi_render_settings_default(&_renderSettingsPolicy)',
    'terminal_parser_ffi_render_settings_set_mode(',
    'terminal_parser_ffi_render_settings_get_mode(',
    'terminal_parser_ffi_render_settings_restore_programmable_defaults(&_renderSettingsPolicy)',
    'terminal_parser_ffi_render_settings_toggle_blink(&_renderSettingsPolicy)',
    'TERMINAL_PARSER_FFI_RENDER_MODE_INDEXED_DISTINGUISHABLE_COLORS',
    'TERMINAL_PARSER_FFI_RENDER_MODE_SYNCHRONIZED_OUTPUT',
    'terminal_parser_ffi_render_attribute_effects_from_rendition(',
    'terminal_parser_ffi_render_attribute_alpha(',
    'attr.IsFaint() ? 1u : 0u',
    'attr.IsBlinking() ? 1u : 0u',
    '_renderSettingsPolicy.blink_should_be_faint',
    'screenReversed ? 1u : 0u'
)
foreach ($needle in $requiredProduct) {
    if (-not $product.Contains($needle)) { throw "Render settings product route missing: $needle" }
}

$forbiddenProduct = @(
    'const auto dimFg =',
    'terminal_parser_ffi_render_attribute_effects(',
    'const auto swapFgAndBg = attr.IsReverseVideo() ^ GetRenderMode(Mode::ScreenReversed);',
    'fg = (fg >> 1) & 0x7F7F7F;',
    'std::swap(fg, bg);',
    'fg |= 0xff000000;',
    'bg |= 0xff000000;'
)
foreach ($needle in $forbiddenProduct) {
    if ($product.Contains($needle)) { throw "Legacy C++ render attribute color ownership returned: $needle" }
}

$requiredBuild = @(
    "'`$(MSBuildProjectName)' == 'base'",
    '<_RustRendererFfiLib>',
    'terminal_parser_ffi.lib',
    'BuildRustRendererFfi',
    'cargo build --locked -p terminal-parser-ffi'
)
foreach ($needle in $requiredBuild) {
    if (-not $buildTargets.Contains($needle)) { throw "RendererBase Rust link evidence missing: $needle" }
}

Write-Host 'Render settings Rust owner, attribute-color/underline replay, product route, RendererBase route, and legacy ownership removal are guarded.'
