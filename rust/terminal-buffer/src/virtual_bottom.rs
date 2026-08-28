//! Safe viewport/cursor/virtual-bottom semantics derived from Host ScreenBuffer tests.
//!
//! This owner intentionally excludes text reflow. It models the portable state
//! transitions shared by cursor movement, viewport sizing, offscreen line feeds,
//! cursor visibility and returning to the virtual viewport.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewportState {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
}

impl ViewportState {
    #[must_use]
    pub const fn bottom(self) -> u16 {
        self.top.saturating_add(self.height.saturating_sub(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualBottomState {
    viewport: ViewportState,
    virtual_bottom: u16,
    cursor: CursorPosition,
}

impl VirtualBottomState {
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        let viewport = ViewportState { left: 0, top: 0, width, height };
        Self {
            viewport,
            virtual_bottom: viewport.bottom(),
            cursor: CursorPosition { x: 0, y: 0 },
        }
    }

    #[must_use]
    pub const fn viewport(&self) -> ViewportState {
        self.viewport
    }

    #[must_use]
    pub const fn virtual_bottom(&self) -> u16 {
        self.virtual_bottom
    }

    #[must_use]
    pub const fn cursor(&self) -> CursorPosition {
        self.cursor
    }

    #[must_use]
    pub fn virtual_viewport(&self) -> ViewportState {
        ViewportState {
            left: self.viewport.left,
            top: self.virtual_bottom.saturating_sub(self.viewport.height.saturating_sub(1)),
            width: self.viewport.width,
            height: self.viewport.height,
        }
    }

    pub fn set_viewport_origin(&mut self, left: u16, top: u16, update_virtual_bottom: bool) {
        self.viewport.left = left;
        self.viewport.top = top;
        if update_virtual_bottom {
            self.virtual_bottom = self.viewport.bottom();
        }
    }

    pub fn set_cursor_direct(&mut self, x: u16, y: u16) {
        self.cursor = CursorPosition { x, y };
    }

    /// Output-driven cursor movement grows virtual bottom when content advances
    /// below it, but never forces a manually scrolled viewport to follow.
    pub fn advance_output_lines(&mut self, lines: u16) {
        self.cursor.y = self.cursor.y.saturating_add(lines);
        if self.cursor.y > self.virtual_bottom {
            self.virtual_bottom = self.cursor.y;
        }
    }

    /// Console API cursor positioning makes the cursor visible inside the virtual
    /// viewport. Moving above that range may move virtual bottom upward; moving
    /// within it preserves virtual bottom.
    pub fn set_console_cursor_position(&mut self, x: u16, y: u16) {
        self.cursor = CursorPosition { x, y };
        let virtual_viewport = self.virtual_viewport();
        if y < virtual_viewport.top {
            self.viewport.top = y;
            self.virtual_bottom = self.viewport.bottom();
        } else if y > self.virtual_bottom {
            self.viewport.top = y.saturating_sub(self.viewport.height.saturating_sub(1));
            self.virtual_bottom = y;
        } else {
            self.viewport.top = virtual_viewport.top;
        }
    }

    /// Internal viewport resizing preserves virtual bottom unless the resized
    /// viewport crosses through it, in which case the bottom realigns.
    pub fn internal_set_viewport_height(&mut self, height: u16) {
        let old_bottom = self.viewport.bottom();
        self.viewport.height = height;
        let new_bottom = self.viewport.bottom();
        let crossed = (old_bottom < self.virtual_bottom && new_bottom >= self.virtual_bottom)
            || (old_bottom > self.virtual_bottom && new_bottom <= self.virtual_bottom);
        if crossed {
            self.virtual_bottom = new_bottom;
        }
    }

    /// Window-resize VT changes viewport dimensions without rebasing virtual bottom.
    pub fn resize_window(&mut self, width: u16, height: u16) {
        self.viewport.width = width;
        self.viewport.height = height;
    }

    /// A line feed issued while the cursor is outside the visible viewport must
    /// not perturb the virtual bottom unless the cursor actually crosses it.
    pub fn offscreen_linefeed(&mut self) {
        self.cursor.y = self.cursor.y.saturating_add(1);
        if self.cursor.y > self.virtual_bottom {
            self.virtual_bottom = self.cursor.y;
        }
    }

    /// Scrolls only enough to make the cursor visible while preserving virtual bottom.
    pub fn make_cursor_visible(&mut self) {
        if self.cursor.y < self.viewport.top {
            self.viewport.top = self.cursor.y;
        } else if self.cursor.y > self.viewport.bottom() {
            self.viewport.top = self.cursor.y.saturating_sub(self.viewport.height.saturating_sub(1));
        }
    }

    /// Returns to the virtual viewport while retaining horizontal scroll offset.
    pub fn move_to_virtual_bottom(&mut self) {
        self.viewport.top = self.virtual_viewport().top;
    }
}
