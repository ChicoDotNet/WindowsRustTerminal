$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$sourcePath = Join-Path $repoRoot 'src/terminal/parser/OutputStateMachineEngine.cpp'
$ffiPath = Join-Path $repoRoot 'rust/terminal-parser-ffi/src/output_csi_window_manipulation.rs'
$probePath = Join-Path $repoRoot 'tools/rust/R09OutputCsiWindowManipulationAbiProbe.hpp'

$source = Get-Content -Raw -LiteralPath $sourcePath
$ffi = Get-Content -Raw -LiteralPath $ffiPath
$probe = Get-Content -Raw -LiteralPath $probePath

function Get-FunctionBody([string] $Signature)
{
    $start = $source.IndexOf($Signature, [StringComparison]::Ordinal)
    if ($start -lt 0) { throw "R09 CSI window manipulation ownership gate: $Signature signature not found." }
    $openBrace = $source.IndexOf('{', $start)
    if ($openBrace -lt 0) { throw "R09 CSI window manipulation ownership gate: $Signature opening brace not found." }
    $depth = 0
    for ($i = $openBrace; $i -lt $source.Length; $i++)
    {
        switch ($source[$i])
        {
            '{' { $depth++ }
            '}' { $depth--; if ($depth -eq 0) { return $source.Substring($start, $i - $start + 1) } }
        }
    }
    throw "R09 CSI window manipulation ownership gate: $Signature closing brace not found."
}

$csiBody = Get-FunctionBody 'bool OutputStateMachineEngine::ActionCsiDispatch(const VTID id, const VTParameters parameters)'

if (-not $source.Contains('#include "terminal_parser_ffi_output_csi_window_manipulation.h"')) { throw 'R09 CSI window manipulation ownership gate: native product no longer includes the Rust ABI contract.' }
if (-not $csiBody.Contains('terminal_parser_ffi_output_csi_window_manipulation_plan')) { throw 'R09 CSI window manipulation ownership gate: ActionCsiDispatch no longer delegates window manipulation planning to Rust.' }
if (-not $csiBody.Contains('static_cast<int32_t>(parameters.at(0).value_or(0))')) { throw 'R09 CSI window manipulation ownership gate: function parameter serialization into the Rust ABI is missing.' }
if (-not $csiBody.Contains('static_cast<int32_t>(parameters.at(1).value_or(0))')) { throw 'R09 CSI window manipulation ownership gate: first payload parameter serialization into the Rust ABI is missing.' }
if (-not $csiBody.Contains('static_cast<int32_t>(parameters.at(2).value_or(0))')) { throw 'R09 CSI window manipulation ownership gate: second payload parameter serialization into the Rust ABI is missing.' }
if (-not $csiBody.Contains('_dispatch->WindowManipulation(static_cast<DispatchTypes::WindowManipulationType>(windowManipulationPlan.function), windowManipulationPlan.parameter1, windowManipulationPlan.parameter2);')) { throw 'R09 CSI window manipulation ownership gate: native WindowManipulationType adaptation or dispatch materialization is missing.' }
if (-not $csiBody.Contains('if (windowManipulationPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_WINDOW_MANIPULATION_NONE)')) { throw 'R09 CSI window manipulation ownership gate: Rust-owned plan no longer short-circuits before the residual CSI switch.' }
if ($csiBody.Contains('case CsiActionCodes::DTTERM_WindowManipulation:')) { throw 'R09 CSI window manipulation ownership gate: duplicate C++ classification returned.' }

if (-not $ffi.Contains('terminal_parser_ffi_output_csi_window_manipulation_plan')) { throw 'R09 CSI window manipulation ownership gate: terminal-parser-ffi no longer exports the planning seam.' }
if (-not $ffi.Contains('engine.action_csi_dispatch')) { throw 'R09 CSI window manipulation ownership gate: FFI no longer delegates classification to the Rust output engine.' }
if (-not $ffi.Contains('OutputAction::WindowManipulation')) { throw 'R09 CSI window manipulation ownership gate: FFI no longer materializes the Rust WindowManipulation action.' }
if (-not $ffi.Contains('Some(parameter0)')) { throw 'R09 CSI window manipulation ownership gate: function parameter replay is missing.' }
if (-not $probe.Contains('plan.function != 8 || plan.parameter1 != 24 || plan.parameter2 != 80')) { throw 'R09 CSI window manipulation ownership gate: native Microsoft-contract witness is missing.' }
if (-not $probe.Contains('defaulted.function != 1 || defaulted.parameter1 != 1 || defaulted.parameter2 != 1')) { throw 'R09 CSI window manipulation ownership gate: native numeric-defaulting witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_OUTPUT_CSI_WINDOW_MANIPULATION_NONE')) { throw 'R09 CSI window manipulation ownership gate: unrelated CSI rejection witness is missing.' }
if (-not $probe.Contains('TERMINAL_PARSER_FFI_INVALID_ARGUMENT')) { throw 'R09 CSI window manipulation ownership gate: null-pointer validation witness is missing.' }

Write-Host 'R09 CSI window manipulation ownership gate passed: Rust owns CSI t classification and numeric normalization; C++ retains only ABI serialization, WindowManipulationType adaptation, and native dispatch materialization.'
