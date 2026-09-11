$ErrorActionPreference = 'Stop'

$owner = Get-Content -Raw -LiteralPath 'rust/terminal-host/src/pty_signal.rs'
$ffi = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/src/pty_signal.rs'
$header = Get-Content -Raw -LiteralPath 'rust/terminal-parser-ffi/include/terminal_parser_ffi_pty_signal.h'
$probe = Get-Content -Raw -LiteralPath 'tools/rust/R09PtySignalAbiProbe.hpp'
$aggregate = Get-Content -Raw -LiteralPath 'tools/rust/R09ControlCharacterAbiProbe.cpp'
$product = Get-Content -Raw -LiteralPath 'src/host/PtySignalInputThread.cpp'
$buildTargets = Get-Content -Raw -LiteralPath 'Directory.Build.targets'

$requiredOwner = @(
    'ShowHideWindow = 1',
    'ClearBuffer = 2',
    'SetParent = 3',
    'ResizeWindow = 8',
    'pub fn plan_signal(bytes: [u8; 2])',
    'Self::ShowHideWindow | Self::ClearBuffer => 2',
    'Self::ResizeWindow => 4',
    'Self::SetParent => 8'
)
foreach ($needle in $requiredOwner) {
    if (-not $owner.Contains($needle)) { throw "PTY Rust read-plan owner evidence missing: $needle" }
}

$requiredFfi = @(
    'terminal_parser_ffi_pty_signal_read_plan',
    'plan_signal(signal_id.to_le_bytes())',
    'PtySignal::ShowHideWindow => PtySignalKind::ShowHideWindow',
    'PtySignal::ClearBuffer => PtySignalKind::ClearBuffer',
    'PtySignal::SetParent => PtySignalKind::SetParent',
    'PtySignal::ResizeWindow => PtySignalKind::ResizeWindow',
    'return FfiStatus::InvalidArgument'
)
foreach ($needle in $requiredFfi) {
    if (-not $ffi.Contains($needle)) { throw "PTY FFI evidence missing: $needle" }
}

$requiredAbi = @(
    'TERMINAL_PARSER_FFI_PTY_SIGNAL_SHOW_HIDE_WINDOW = 1',
    'TERMINAL_PARSER_FFI_PTY_SIGNAL_CLEAR_BUFFER = 2',
    'TERMINAL_PARSER_FFI_PTY_SIGNAL_SET_PARENT = 3',
    'TERMINAL_PARSER_FFI_PTY_SIGNAL_RESIZE_WINDOW = 8',
    'uint32_t payload_len;',
    'terminal_parser_ffi_pty_signal_read_plan('
)
foreach ($needle in $requiredAbi) {
    if (-not $header.Contains($needle)) { throw "PTY C ABI contract evidence missing: $needle" }
}

$requiredWitness = @(
    'expect_pty_signal_read_plan(1, TERMINAL_PARSER_FFI_PTY_SIGNAL_SHOW_HIDE_WINDOW, 2)',
    'expect_pty_signal_read_plan(2, TERMINAL_PARSER_FFI_PTY_SIGNAL_CLEAR_BUFFER, 2)',
    'expect_pty_signal_read_plan(3, TERMINAL_PARSER_FFI_PTY_SIGNAL_SET_PARENT, 8)',
    'expect_pty_signal_read_plan(8, TERMINAL_PARSER_FFI_PTY_SIGNAL_RESIZE_WINDOW, 4)',
    'terminal_parser_ffi_pty_signal_read_plan(4, &invalid)',
    'terminal_parser_ffi_pty_signal_read_plan(1, nullptr)'
)
foreach ($needle in $requiredWitness) {
    if (-not $probe.Contains($needle)) { throw "PTY native contract witness missing: $needle" }
}

$requiredAggregate = @(
    '#include "R09PtySignalAbiProbe.hpp"',
    'const bool ptySignalReadPlanOk = r09::pty_signal_read_plan_replay();',
    '!ptySignalReadPlanOk'
)
foreach ($needle in $requiredAggregate) {
    if (-not $aggregate.Contains($needle)) { throw "PTY aggregate replay wiring missing: $needle" }
}

$requiredProductRoute = @(
    '#include "terminal_parser_ffi_pty_signal.h"',
    'terminal_parser_ffi_pty_signal_read_plan(static_cast<uint16_t>(signalId), &plan)',
    'switch (plan.kind)',
    'plan.payload_len != sizeof(msg)',
    'plan.payload_len != sizeof(resizeMsg)',
    'plan.payload_len != sizeof(reparentMessage)'
)
foreach ($needle in $requiredProductRoute) {
    if (-not $product.Contains($needle)) { throw "PTY product route evidence missing: $needle" }
}

$forbiddenProductOwnership = @(
    'switch (signalId)',
    'case PtySignal::ShowHideWindow',
    'case PtySignal::ClearBuffer',
    'case PtySignal::ResizeWindow',
    'case PtySignal::SetParent'
)
foreach ($needle in $forbiddenProductOwnership) {
    if ($product.Contains($needle)) { throw "PTY legacy classification returned to C++ product code: $needle" }
}

$requiredHostLink = @(
    "'$(MSBuildProjectName)' == 'Host'",
    'rust\terminal-parser-ffi\include',
    'terminal_parser_ffi.lib',
    'cargo build --locked -p terminal-parser-ffi'
)
foreach ($needle in $requiredHostLink) {
    if (-not $buildTargets.Contains($needle)) { throw "PTY Host/Rust link evidence missing: $needle" }
}

Write-Host 'PTY read-plan contract and product ownership gate passed.'
