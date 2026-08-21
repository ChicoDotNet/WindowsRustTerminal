from pathlib import Path

path = Path("rust/terminal-input/src/lib.rs")
text = path.read_text()


def replace_once(old: str, new: str, label: str) -> None:
    global text
    if old not in text:
        raise SystemExit(f"R02c marker not found: {label}")
    text = text.replace(old, new, 1)


replace_once(
    "mod keyboard;\npub use keyboard::{KeyboardMapper, PortableKeyboardMapper};\n",
    "mod keyboard;\nmod mouse;\n\npub use keyboard::{KeyboardMapper, PortableKeyboardMapper};\npub use mouse::{MouseButtonState, MouseMessage, Point};\n",
    "mouse module export",
)

replace_once(
    "    kitty_alt_stack: Vec<u8>,\n}",
    "    kitty_alt_stack: Vec<u8>,\n    mouse_input_state: mouse::MouseInputState,\n}",
    "mouse state field",
)

replace_once(
    "            kitty_alt_stack: Vec::new(),\n        };",
    "            kitty_alt_stack: Vec::new(),\n            mouse_input_state: mouse::MouseInputState::default(),\n        };",
    "mouse state default",
)

path.write_text(text)

mouse_path = Path("rust/terminal-input/src/mouse.rs")
mouse = mouse_path.read_text()
mouse = mouse.replace(
    "        MouseMessage::LeftDoubleClick | MouseMessage::LeftDown => 0,\n        MouseMessage::LeftUp | MouseMessage::MiddleUp | MouseMessage::RightUp => 3,",
    "        MouseMessage::LeftDoubleClick | MouseMessage::LeftDown | MouseMessage::Move => 0,\n        MouseMessage::LeftUp | MouseMessage::MiddleUp | MouseMessage::RightUp => 3,",
    1,
)
mouse = mouse.replace("        MouseMessage::Move => 0,\n", "", 1)
mouse = mouse.replace("i16::MAX as i32", "i32::from(i16::MAX)")
mouse_path.write_text(mouse)
