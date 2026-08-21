from pathlib import Path

path = Path("rust/terminal-adapter/src/sixel.rs")
text = path.read_text(encoding="utf-8")
text = text.replace(
    """    pub fn image_height(&self) -> usize {\n        if self.canvas.width == 0 {\n            0\n        } else {\n            self.image_buffer.len() / self.canvas.width\n        }\n    }\n""",
    """    pub fn image_height(&self) -> usize {\n        self.image_buffer\n            .len()\n            .checked_div(self.canvas.width)\n            .unwrap_or(0)\n    }\n""",
)
text = text.replace(
    """#[expect(\n    clippy::cast_possible_truncation,\n    reason = \"DEC HLS conversion intentionally matches the C++ f32-to-byte rounding path\"\n)]\n""",
    """#[expect(\n    clippy::cast_possible_truncation,\n    clippy::cast_sign_loss,\n    reason = \"DEC HLS conversion intentionally matches the C++ f32-to-byte rounding path\"\n)]\n""",
)
text = text.replace(
    """        assert_eq!(parser.image_width(), 3);\n        assert_eq!(parser.image_cursor().x, 0);\n""",
    """        // Raster attributes perform a carriage return after the first two sixels,\n        // so the final sixel overprints from column zero instead of extending the width.\n        assert_eq!(parser.image_width(), 2);\n        assert_eq!(parser.image_cursor().x, 0);\n""",
)
path.write_text(text, encoding="utf-8")
