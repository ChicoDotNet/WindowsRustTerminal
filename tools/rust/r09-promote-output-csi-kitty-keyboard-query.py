from pathlib import Path
import hashlib

path = Path("src/terminal/parser/OutputStateMachineEngine.cpp")
raw = path.read_bytes()

if b"\r" in raw.replace(b"\r\n", b""):
    raise SystemExit("unexpected lone CR")

has_crlf = b"\r\n" in raw
data = raw.replace(b"\r\n", b"\n")

include_anchor = b'#include "terminal_parser_ffi_output_csi_user_preference_charset.h"\n'
include_line = b'#include "terminal_parser_ffi_output_csi_kitty_keyboard_query.h"\n'
if include_line not in data:
    if include_anchor not in data:
        raise SystemExit("kitty query include anchor missing")
    data = data.replace(include_anchor, include_anchor + include_line, 1)

plan_anchor = (
    b'    if (userPreferenceCharsetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE)\n'
    b'    {\n'
    b'        _ClearLastChar();\n'
    b'        return true;\n'
    b'    }\n\n'
    b'    switch (id)\n'
)
plan_block = (
    b'    if (userPreferenceCharsetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE)\n'
    b'    {\n'
    b'        _ClearLastChar();\n'
    b'        return true;\n'
    b'    }\n\n'
    b'    terminal_parser_ffi_output_csi_kitty_keyboard_query_result kittyKeyboardQueryPlan{};\n'
    b'    const auto kittyKeyboardQueryStatus = terminal_parser_ffi_output_csi_kitty_keyboard_query_plan(\n'
    b'        static_cast<uint64_t>(id),\n'
    b'        &kittyKeyboardQueryPlan);\n'
    b'    THROW_HR_IF(E_UNEXPECTED, kittyKeyboardQueryStatus != TERMINAL_PARSER_FFI_OK);\n\n'
    b'    switch (kittyKeyboardQueryPlan.kind)\n'
    b'    {\n'
    b'    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_QUERY:\n'
    b'        _dispatch->QueryKittyKeyboardProtocol();\n'
    b'        break;\n'
    b'    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE:\n'
    b'        break;\n'
    b'    default:\n'
    b'        THROW_HR(E_UNEXPECTED);\n'
    b'    }\n\n'
    b'    if (kittyKeyboardQueryPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE)\n'
    b'    {\n'
    b'        _ClearLastChar();\n'
    b'        return true;\n'
    b'    }\n\n'
    b'    switch (id)\n'
)
if b'terminal_parser_ffi_output_csi_kitty_keyboard_query_result kittyKeyboardQueryPlan{};' not in data:
    if plan_anchor not in data:
        raise SystemExit("kitty query plan anchor missing")
    data = data.replace(plan_anchor, plan_block, 1)

legacy_case = (
    b'    case CsiActionCodes::KKP_KittyKeyboardQuery:\n'
    b'        _dispatch->QueryKittyKeyboardProtocol();\n'
    b'        break;\n'
)
if legacy_case not in data:
    raise SystemExit("legacy kitty query case missing or already removed")
data = data.replace(legacy_case, b'', 1)

if b'case CsiActionCodes::KKP_KittyKeyboardQuery:' in data:
    raise SystemExit("legacy kitty query case still present")
if data.count(b'terminal_parser_ffi_output_csi_kitty_keyboard_query_plan(') != 1:
    raise SystemExit("unexpected kitty query plan call count")

output = data.replace(b"\n", b"\r\n") if has_crlf else data
path.write_bytes(output)
print(hashlib.sha256(output).hexdigest())
