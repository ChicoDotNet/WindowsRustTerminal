from pathlib import Path
import re

path = Path("rust/terminal-input/src/lib.rs")
text = path.read_text()


def replace_once(old: str, new: str, label: str) -> None:
    global text
    if old not in text:
        raise SystemExit(f"R02b marker not found: {label}")
    text = text.replace(old, new, 1)


replace_once(
    "//! R02a intentionally excludes keyboard-layout translation and mouse encoding.\n//! Those are isolated behind later increments so this crate stays deterministic\n//! on Linux and Windows.\n\n#![forbid(unsafe_code)]\n",
    "//! R02b adds layout-aware Unicode and Kitty keyboard encoding behind a narrow\n//! mapper abstraction. Mouse encoding remains isolated for R02c.\n\n#![forbid(unsafe_code)]\n\nmod keyboard;\npub use keyboard::{KeyboardMapper, PortableKeyboardMapper};\n",
    "module declaration",
)

replace_once(
    "    pub const NUMLOCK_ON: u32 = 0x0020;\n    pub const ENHANCED_KEY: u32 = 0x0100;",
    "    pub const NUMLOCK_ON: u32 = 0x0020;\n    pub const CAPSLOCK_ON: u32 = 0x0080;\n    pub const ENHANCED_KEY: u32 = 0x0100;",
    "caps lock state",
)

replace_once(
    "    pub const PAUSE: u16 = 0x13;\n    pub const ESCAPE: u16 = 0x1b;",
    "    pub const PAUSE: u16 = 0x13;\n    pub const CAPITAL: u16 = 0x14;\n    pub const ESCAPE: u16 = 0x1b;",
    "capital key",
)
replace_once(
    "    pub const DELETE: u16 = 0x2e;\n    pub const NUMPAD0: u16 = 0x60;",
    "    pub const DELETE: u16 = 0x2e;\n    pub const SNAPSHOT: u16 = 0x2c;\n    pub const LWIN: u16 = 0x5b;\n    pub const RWIN: u16 = 0x5c;\n    pub const APPS: u16 = 0x5d;\n    pub const NUMPAD0: u16 = 0x60;",
    "extended special keys",
)
replace_once(
    "    pub const F20: u16 = 0x83;\n    pub const LSHIFT: u16 = 0xa0;",
    "    pub const F20: u16 = 0x83;\n    pub const F21: u16 = 0x84;\n    pub const F22: u16 = 0x85;\n    pub const F23: u16 = 0x86;\n    pub const F24: u16 = 0x87;\n    pub const NUMLOCK: u16 = 0x90;\n    pub const SCROLL: u16 = 0x91;\n    pub const LSHIFT: u16 = 0xa0;",
    "F24 and locks",
)
replace_once(
    "    pub const RMENU: u16 = 0xa5;\n    pub const PACKET: u16 = 0xe7;",
    "    pub const RMENU: u16 = 0xa5;\n    pub const VOLUME_MUTE: u16 = 0xad;\n    pub const VOLUME_DOWN: u16 = 0xae;\n    pub const VOLUME_UP: u16 = 0xaf;\n    pub const MEDIA_NEXT_TRACK: u16 = 0xb0;\n    pub const MEDIA_PREV_TRACK: u16 = 0xb1;\n    pub const MEDIA_STOP: u16 = 0xb2;\n    pub const MEDIA_PLAY_PAUSE: u16 = 0xb3;\n    pub const PACKET: u16 = 0xe7;",
    "media keys",
)

replace_once(
    "    last_virtual_key: Option<u16>,\n    force_disable_kitty_keyboard_protocol: bool,",
    "    last_virtual_key: Option<u16>,\n    leading_surrogate: Option<u16>,\n    force_disable_kitty_keyboard_protocol: bool,",
    "surrogate field",
)
replace_once(
    "            last_virtual_key: None,\n            force_disable_kitty_keyboard_protocol: false,",
    "            last_virtual_key: None,\n            leading_surrogate: None,\n            force_disable_kitty_keyboard_protocol: false,",
    "surrogate initialization",
)
replace_once(
    "        self.last_virtual_key = None;\n        self.reset_kitty_keyboard_protocols();",
    "        self.last_virtual_key = None;\n        self.leading_surrogate = None;\n        self.reset_kitty_keyboard_protocols();",
    "surrogate reset",
)

start = text.index("    /// Translates one typed key event into its VT input representation.\n")
end = text.index("    fn set_mode(&mut self, mode: Mode, enabled: bool) {", start)
new_handle = '''    /// Translates one typed key event using the portable keyboard mapper.\n    ///\n    /// An empty string means the event was handled but intentionally emitted nothing.\n    #[must_use]\n    pub fn handle_key(&mut self, event: KeyEvent) -> String {\n        self.handle_key_with_mapper(event, &PortableKeyboardMapper)\n    }\n\n    /// Translates one key event with an injected keyboard-layout mapper.\n    ///\n    /// This is the platform seam for the future Windows `ToUnicodeEx` adapter and\n    /// for deterministic layout fixtures in Rust-native tests.\n    #[must_use]\n    pub fn handle_key_with_mapper<M: KeyboardMapper>(\n        &mut self,\n        event: KeyEvent,\n        mapper: &M,\n    ) -> String {\n        keyboard::handle_key(self, event, mapper)\n    }\n\n'''
text = text[:start] + new_handle + text[end:]

pattern = re.compile(r"\n    fn encode_special\(.*?\n}\n\nconst fn mode_bit", re.S)
text, count = pattern.subn("\n}\n\nconst fn mode_bit", text, count=1)
if count != 1:
    raise SystemExit("R02b marker not found: legacy encoder block")

for fn_name, next_name in [
    ("is_modifier_key", "modifier_parameter"),
    ("modifier_parameter", "with_alt_prefix"),
    ("with_alt_prefix", "codepoint_string"),
]:
    pattern = re.compile(rf"\nfn {fn_name}\(.*?(?=\nfn {next_name}\()", re.S)
    text, count = pattern.subn("\n", text, count=1)
    if count != 1:
        raise SystemExit(f"R02b marker not found: obsolete {fn_name}")

path.write_text(text)
