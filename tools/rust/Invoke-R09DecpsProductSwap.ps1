$ErrorActionPreference = 'Stop'

$path = 'src/terminal/parser/OutputStateMachineEngine.cpp'
$text = Get-Content -Raw -LiteralPath $path

$includeAnchor = '#include "terminal_parser_ffi_output_csi_rep.h"'
$includeReplacement = "$includeAnchor`r`n#include `"terminal_parser_ffi_output_csi_decps.h`""
if (-not $text.Contains($includeAnchor)) { throw 'REP include anchor not found.' }
if (-not $text.Contains('#include "terminal_parser_ffi_output_csi_decps.h"')) {
    $text = $text.Replace($includeAnchor, $includeReplacement)
}

$legacy = @'
    switch (id)
    {



























    case CsiActionCodes::DECPS_PlaySound:
        _dispatch->PlaySounds(parameters);
        break;



    default:
        _dispatch->UnknownSequence();
        break;
    }
'@

$route = @'
    terminal_parser_ffi_output_csi_decps_result decpsPlan{};
    const auto decpsStatus = terminal_parser_ffi_output_csi_decps_plan(
        static_cast<uint64_t>(id),
        &decpsPlan);
    THROW_HR_IF(E_UNEXPECTED, decpsStatus != TERMINAL_PARSER_FFI_OK);

    switch (decpsPlan.kind)
    {
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_PLAY_SOUNDS:
        _dispatch->PlaySounds(parameters);
        break;
    case TERMINAL_PARSER_FFI_OUTPUT_CSI_DECPS_NONE:
        _dispatch->UnknownSequence();
        break;
    default:
        THROW_HR(E_UNEXPECTED);
    }
'@

if (-not $text.Contains($legacy)) { throw 'DECPS legacy switch anchor not found; refusing non-mechanical swap.' }
$text = $text.Replace($legacy, $route)
Set-Content -LiteralPath $path -Value $text -NoNewline
