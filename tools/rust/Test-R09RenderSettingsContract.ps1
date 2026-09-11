$ErrorActionPreference = 'Stop'

$owner = Get-Content -Raw -LiteralPath 'rust/terminal-renderer/src/render_settings_policy.rs'
$ffi = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/src/render_settings.rs'
$header = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/include/terminal_parser_ffi_render_settings.h'
$probe = Get-Content -Raw -LiteralPath 'tools/rust/R09RenderSettingsAbiProbe.hpp'

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
    'terminal_parser_ffi_render_settings_default(nullptr)'
)
foreach ($needle in $requiredWitness) {
    if (-not $probe.Contains($needle)) { throw "Render settings native contract witness missing: $needle" }
}

Write-Host 'Render settings Rust owner, C ABI, and native replay contract are prepared.'
