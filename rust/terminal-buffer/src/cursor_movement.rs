//! Safe VT cursor movement semantics derived from Host ScreenBuffer tests.
//!
//! This owner models cursor movement and DEC margin clamping only. It
//! intentionally excludes text mutation, saved cursor attributes/charset state,
//! scrolling/reflow and renderer coordination.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorMovementState {
    width: u16,
    height: u16,
    cursor: CursorPosition,
    vertical_margins: Option<(u16, u16)>,
    horizontal_margins: Option<(u16, u16)>,
    horizontal_margin_mode: bool,
}

impl CursorMovementState {
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        Self {
            width,
            height,
            cursor: CursorPosition { x: 0, y: 0 },
            vertical_margins: None,
            horizontal_margins: None,
            horizontal_margin_mode: false,
        }
    }

    #[must_use]
    pub const fn cursor(&self) -> CursorPosition {
        self.cursor
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.cursor.x = x.min(self.width - 1);
        self.cursor.y = y.min(self.height - 1);
    }

    pub fn set_vertical_margins(&mut self, top: u16, bottom: u16) {
        assert!(top <= bottom);
        assert!(bottom < self.height);
        self.vertical_margins = Some((top, bottom));
    }

    pub fn clear_vertical_margins(&mut self) {
        self.vertical_margins = None;
    }

    pub fn set_horizontal_margin_mode(&mut self, enabled: bool) {
        self.horizontal_margin_mode = enabled;
    }

    pub fn set_horizontal_margins(&mut self, left: u16, right: u16) {
        assert!(left <= right);
        assert!(right < self.width);
        self.horizontal_margins = Some((left, right));
    }

    fn vertical_bounds(&self) -> (u16, u16) {
        self.vertical_margins.unwrap_or((0, self.height - 1))
    }

    fn horizontal_bounds(&self) -> (u16, u16) {
        if self.horizontal_margin_mode {
            self.horizontal_margins.unwrap_or((0, self.width - 1))
        } else {
            (0, self.width - 1)
        }
    }

    pub fn cursor_up(&mut self, count: u16) {
        let count = count.max(1);
        let (top, bottom) = self.vertical_bounds();
        let y = self.cursor.y;
        self.cursor.y = if y >= top && y <= bottom {
            y.saturating_sub(count).max(top)
        } else if y > bottom {
            let target = y.saturating_sub(count);
            if target < top { top } else { target }
        } else {
            y.saturating_sub(count)
        };
    }

    pub fn cursor_down(&mut self, count: u16) {
        let count = count.max(1);
        let (top, bottom) = self.vertical_bounds();
        let y = self.cursor.y;
        self.cursor.y = if y >= top && y <= bottom {
            y.saturating_add(count).min(bottom)
        } else if y < top {
            let target = y.saturating_add(count).min(self.height - 1);
            if target > bottom { bottom } else { target }
        } else {
            y.saturating_add(count).min(self.height - 1)
        };
    }

    pub fn cursor_left(&mut self, count: u16) {
        let count = count.max(1);
        let (left, right) = self.horizontal_bounds();
        let x = self.cursor.x;
        self.cursor.x = if x >= left && x <= right {
            x.saturating_sub(count).max(left)
        } else if x > right {
            let target = x.saturating_sub(count);
            if target < left { left } else { target }
        } else {
            x.saturating_sub(count)
        };
    }

    pub fn cursor_right(&mut self, count: u16) {
        let count = count.max(1);
        let (left, right) = self.horizontal_bounds();
        let x = self.cursor.x;
        self.cursor.x = if x >= left && x <= right {
            x.saturating_add(count).min(right)
        } else if x < left {
            let target = x.saturating_add(count).min(self.width - 1);
            if target > right { right } else { target }
        } else {
            x.saturating_add(count).min(self.width - 1)
        };
    }

    pub fn cursor_next_line(&mut self, count: u16) {
        let was_inside_vertical_margins = self
            .vertical_margins
            .is_some_and(|(top, bottom)| self.cursor.y >= top && self.cursor.y <= bottom);
        self.cursor_down(count);
        self.cursor.x = if was_inside_vertical_margins {
            self.horizontal_bounds().0
        } else {
            0
        };
    }

    pub fn cursor_previous_line(&mut self, count: u16) {
        let was_inside_vertical_margins = self
            .vertical_margins
            .is_some_and(|(top, bottom)| self.cursor.y >= top && self.cursor.y <= bottom);
        self.cursor_up(count);
        self.cursor.x = if was_inside_vertical_margins {
            self.horizontal_bounds().0
        } else {
            0
        };
    }

    /// HPR is relative to the full row and intentionally ignores horizontal margins.
    pub fn horizontal_position_relative(&mut self, count: u16) {
        self.cursor.x = self.cursor.x.saturating_add(count.max(1)).min(self.width - 1);
    }

    /// VPR is relative to the full viewport and intentionally ignores vertical margins.
    pub fn vertical_position_relative(&mut self, count: u16) {
        self.cursor.y = self.cursor.y.saturating_add(count.max(1)).min(self.height - 1);
    }
}
