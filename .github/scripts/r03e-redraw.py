from pathlib import Path

path = Path("rust/terminal-adapter/src/page_manager.rs")
text = path.read_text()

replacements = [
    (
        """        let was_visible = old_active_page == old_visible_page;\n        let size = PageSize::from(self.visible_geometry);\n\n        if make_visible && self.visible_page_number != new_page_number {\n""",
        """        let was_visible = old_active_page == old_visible_page;\n        let size = PageSize::from(self.visible_geometry);\n        let mut redraw_required = false;\n\n        if make_visible && self.visible_page_number != new_page_number {\n""",
    ),
    (
        """            self.visible_page_number = new_page_number;\n            self.events.push(PageEvent::RedrawAll);\n        }\n""",
        """            self.visible_page_number = new_page_number;\n            redraw_required = true;\n        }\n""",
    ),
    (
        """        self.active_page_number = new_page_number;\n        let new_active_top = self.top_for(self.active_page_number, self.visible_page_number);\n""",
        """        self.active_page_number = new_page_number;\n        if redraw_required {\n            self.events.push(PageEvent::RedrawAll);\n        }\n        let new_active_top = self.top_for(self.active_page_number, self.visible_page_number);\n""",
    ),
    (
        """                PageEvent::RedrawAll,\n                PageEvent::CopyProperties {\n                    from: PageBufferRef::Background(4),\n                    to: PageBufferRef::Visible,\n                    old_top: 0,\n                    new_top: 20,\n                },\n""",
        """                PageEvent::CopyProperties {\n                    from: PageBufferRef::Background(4),\n                    to: PageBufferRef::Visible,\n                    old_top: 0,\n                    new_top: 20,\n                },\n                PageEvent::RedrawAll,\n""",
    ),
]

for old, new in replacements:
    if text.count(old) != 1:
        raise SystemExit(f"expected exactly one redraw-order anchor, got {text.count(old)}")
    text = text.replace(old, new, 1)

path.write_text(text)
