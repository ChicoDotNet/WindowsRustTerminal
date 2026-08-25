//! Portable aggregate policy for TerminalCore sizing and history capacity.
//!
//! Microsoft Terminal clamps visible dimensions and total backing rows to the
//! signed 16-bit coordinate domain used by the native product. Keeping that
//! policy here gives Rust a real aggregate owner without pulling WinRT or
//! renderer concerns into the semantic core.

const MAX_COORD: i32 = i16::MAX as i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalDimensions {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalLayout {
    viewport: TerminalDimensions,
    configured_history_rows: u16,
    total_rows: u16,
}

impl TerminalLayout {
    #[must_use]
    pub fn from_settings(history_size: i32, rows: i32, columns: i32) -> Self {
        let height = clamp_dimension(rows);
        let width = clamp_dimension(columns);
        let configured_history_rows = clamp_history(history_size, height);
        Self {
            viewport: TerminalDimensions { width, height },
            configured_history_rows,
            total_rows: height + configured_history_rows,
        }
    }

    #[must_use]
    pub const fn viewport(&self) -> TerminalDimensions {
        self.viewport
    }

    #[must_use]
    pub const fn total_rows(&self) -> u16 {
        self.total_rows
    }

    #[must_use]
    pub const fn configured_history_rows(&self) -> u16 {
        self.configured_history_rows
    }

    /// Applies the same user-resize capacity rule as TerminalCore: viewport
    /// dimensions remain in range and the backing row count is clamped to
    /// `SHRT_MAX` without mutating the configured history allowance. Shrinking
    /// the viewport can therefore restore rows that a larger viewport had
    /// temporarily clipped.
    pub fn user_resize(&mut self, columns: i32, rows: i32) {
        let height = clamp_dimension(rows);
        let width = clamp_dimension(columns);
        let requested_total = i32::from(height) + i32::from(self.configured_history_rows);
        self.total_rows = requested_total.min(MAX_COORD) as u16;
        self.viewport = TerminalDimensions { width, height };
    }
}

fn clamp_dimension(value: i32) -> u16 {
    value.clamp(1, MAX_COORD) as u16
}

fn clamp_history(history_size: i32, visible_rows: u16) -> u16 {
    let available = MAX_COORD - i32::from(visible_rows);
    history_size.clamp(0, available) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn microsoft_screen_size_limits_width_and_height_are_clamped_to_bounds() {
        let negative_columns = TerminalLayout::from_settings(10_000, 9_999_999, -1_234);
        assert_eq!(negative_columns.viewport().height, i16::MAX as u16);
        assert_eq!(negative_columns.viewport().width, 1);

        let zero_rows = TerminalLayout::from_settings(10_000, 0, 9_999_999);
        assert_eq!(zero_rows.viewport().height, 1);
        assert_eq!(zero_rows.viewport().width, i16::MAX as u16);
    }

    #[test]
    fn microsoft_screen_size_limits_scrollback_history_is_clamped_to_bounds() {
        const VISIBLE: i32 = 100;
        assert_eq!(
            TerminalLayout::from_settings(0, VISIBLE, 100).total_rows(),
            100
        );
        assert_eq!(
            TerminalLayout::from_settings(-100, VISIBLE, 100).total_rows(),
            100
        );
        assert_eq!(
            TerminalLayout::from_settings(i32::from(i16::MAX) - VISIBLE, VISIBLE, 100).total_rows(),
            i16::MAX as u16
        );
        assert_eq!(
            TerminalLayout::from_settings(i32::from(i16::MAX) - VISIBLE + 1, VISIBLE, 100)
                .total_rows(),
            i16::MAX as u16
        );
        assert_eq!(
            TerminalLayout::from_settings(99_999_999, VISIBLE, 100).total_rows(),
            i16::MAX as u16
        );
    }

    #[test]
    fn microsoft_screen_size_limits_resize_is_clamped_to_bounds() {
        const COLS: i32 = 50;
        const ROWS: i32 = 50;
        let history = i32::from(i16::MAX) - ROWS * 2;
        let mut terminal = TerminalLayout::from_settings(history, ROWS, COLS);
        assert_eq!(terminal.total_rows(), (history + ROWS) as u16);

        terminal.user_resize(COLS, ROWS * 2);
        assert_eq!(terminal.total_rows(), i16::MAX as u16);

        terminal.user_resize(COLS, ROWS * 3);
        assert_eq!(terminal.total_rows(), i16::MAX as u16);

        terminal.user_resize(COLS, ROWS);
        assert_eq!(terminal.total_rows(), (history + ROWS) as u16);
    }
}
