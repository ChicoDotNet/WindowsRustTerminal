from pathlib import Path
import hashlib

path = Path("src/terminal/parser/OutputStateMachineEngine.cpp")
data = path.read_bytes()

expected_sha256 = ""

include_anchor = b'#include "terminal_parser_ffi_output_csi_user_preference_charset.h"\r\n'
include_line = b'#include "terminal_parser_ffi_output_csi_kitty_keyboard_query.h"\r\n'
if include_line not in data:
    if include_anchor not in data:
        raise SystemExit("kitty query include anchor missing")
    data = data.replace(include_anchor, include_anchor + include_line, 1)

plan_anchor = (
    b'    if (userPreferenceCharsetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE)\r\n'
    b'    {\r\n'
    b'        _ClearLastChar();\r\n'
    b'        return true;\r\n'
    b'    }\r\n\r\n'
    b'    switch (id)\r\n'
)
plan_block = (
    b'    if (userPreferenceCharsetPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE)\r\n'
    b'    {\r\n'
    b'        _ClearLastChar();\r\n'
    b'        return true;\r\n'
    b'    }\r\n\r\n'
    b'    terminal_parser_ffi_output_csi_kitty_keyboard_query_result kittyKeyboardQueryPlan{};\r\n'
    b'    const auto kittyKeyboardQueryStatus = terminal_parser_ffi_output_csi_kitty_keyboard_query_plan(\r\n'
    b'        static_cast<uint64_t>(id),\r\n'
    b'        &kittyKeyboardQueryPlan);\r\n'
    b'    THROW_HR_IF(E_UNEXPECTED, kittyKeyboardQueryStatus != TERMINAL_PARSER_FFI_OK);\r\n\r\n'
    b'    switch (kittyKeyboardQueryPlan.kind)\r\n'
    b'    {\r\n'
    b'    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_QUERY:\r\n'
    b'        _dispatch->QueryKittyKeyboardProtocol();\r\n'
    b'        break;\r\n'
    b'    case TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE:\r\n'
    b'        break;\r\n'
    b'    default:\r\n'
    b'        THROW_HR(E_UNEXPECTED);\r\n'
    b'    }\r\n\r\n'
    b'    if (kittyKeyboardQueryPlan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_KITTY_KEYBOARD_QUERY_NONE)\r\n'
    b'    {\r\n'
    b'        _ClearLastChar();\r\n'
    b'        return true;\r\n'
    b'    }\r\n\r\n'
    b'    switch (id)\r\n'
)
if b'terminal_parser_ffi_output_csi_kitty_keyboard_query_result kittyKeyboardQueryPlan{};' not in data:
    if plan_anchor not in data:
        raise SystemExit("kitty query plan anchor missing")
    data = data.replace(plan_anchor, plan_block, 1)

legacy_case = (
    b'    case CsiActionCodes::KKP_KittyKeyboardQuery:\r\n'
    b'        _dispatch->QueryKittyKeyboardProtocol();\r\n'
    b'        break;\r\n'
)
if legacy_case not in data:
    raise SystemExit("legacy kitty query case missing or already removed")
data = data.replace(legacy_case, b'', 1)

if b'case CsiActionCodes::KKP_KittyKeyboardQuery:' in data:
    raise SystemExit("legacy kitty query case still present")
if data.count(b'terminal_parser_ffi_output_csi_kitty_keyboard_query_plan(') != 1:
    raise SystemExit("unexpected kitty query plan call count")
if b'\n' in data.replace(b'\r\n', b''):
    raise SystemExit("line ending drift detected")

path.write_bytes(data)
print(hashlib.sha256(data).hexdigest())
