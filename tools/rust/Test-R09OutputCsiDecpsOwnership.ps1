$ErrorActionPreference = 'Stop'

$engine = Get-Content -Raw -LiteralPath 'src/terminal/parser/OutputStateMachineEngine.cpp'
$ffi = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/src/output_csi_decps.rs'
$header = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/include/terminal_parser_ffi_output_csi_decps.h'
$owner = Get-Content -Raw -LiteralPath 'rust/terminal-parser/src/output_csi_decps.rs'
$probe = Get-Content -Raw -LiteralPath 'tools/rust/R09OutputCsiDecpsAbiProbe.hpp'

$requiredEngine = @(
    '#include "terminal_parser_ffi_output_csi_decps.h"',
    'terminal_parser_ffi_output_csi_decps_plan(',
    'TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_PLAY_SOUNDS',
    '_dispatch->PlaySounds(parameters);',
    'TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_NONE'
)
foreach ($needle in $requiredEngine) {
    if (-not $engine.Contains($needle)) { throw "DECPS product route missing: $needle" }
}
if ($engine.Contains('case CsiActionCodes::DECPS_PlaySound:')) {
    throw 'Legacy C++ DECPS classification still owns the product route.'
}

$requiredOwner = @('DecpsAction', 'PlaySounds', 'VTID(",~")')
foreach ($needle in $requiredOwner) {
    if (-not $owner.Contains($needle)) { throw "DECPS Rust owner evidence missing: $needle" }
}
$requiredFfi = @('plan_decps', 'OutputCsiDecpsKind::PlaySounds', 'OutputCsiDecpsKind::None', 'InvalidArgument')
foreach ($needle in $requiredFfi) {
    if (-not $ffi.Contains($needle)) { throw "DECPS FFI evidence missing: $needle" }
}
$requiredAbi = @('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_NONE = 0', 'TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_PLAY_SOUNDS = 1')
foreach ($needle in $requiredAbi) {
    if (-not $header.Contains($needle)) { throw "DECPS ABI evidence missing: $needle" }
}
if (-not $probe.Contains("expect_output_csi_decps_plan(`n                ',', '~', TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_PLAY_SOUNDS)")) {
    if (-not ($probe.Contains("',', '~'") -and $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_PLAY_SOUNDS'))) {
        throw 'DECPS native positive witness missing.'
    }
}
if (-not ($probe.Contains("',', '|'") -and $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_NONE'))) {
    throw 'DECPS native neighbor witness missing.'
}

Write-Host 'DECPS ownership gate passed.'
