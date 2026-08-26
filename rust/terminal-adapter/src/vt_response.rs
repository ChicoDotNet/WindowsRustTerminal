//! Portable VT response serialization for adapter reports.
//!
//! This owner intentionally has no host, Win32, renderer, or terminal-input dependency. It
//! serializes deterministic responses and retains the response stream in the same order that
//! `ITerminalApi::ReturnResponse` observes them. Adapter live wiring is a separate integration
//! concern.

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VtResponseEngine {
    response: String,
}

impl VtResponseEngine {
    #[must_use]
    pub fn response(&self) -> &str {
        &self.response
    }

    pub fn clear(&mut self) {
        self.response.clear();
    }

    pub fn operating_status(&mut self) {
        self.push("\u{1b}[0n");
    }

    pub fn cursor_position_report(&mut self, cursor_x: i32, cursor_y: i32, viewport_top: i32) {
        let row = cursor_y.saturating_sub(viewport_top).saturating_add(1);
        let column = cursor_x.saturating_add(1);
        self.push(&format!("\u{1b}[{row};{column}R"));
    }

    pub fn extended_cursor_position_report(
        &mut self,
        cursor_x: i32,
        cursor_y: i32,
        viewport_top: i32,
        page: i32,
    ) {
        let row = cursor_y.saturating_sub(viewport_top).saturating_add(1);
        let column = cursor_x.saturating_add(1);
        let page = page.max(1);
        self.push(&format!("\u{1b}[?{row};{column};{page}R"));
    }

    fn push(&mut self, response: &str) {
        self.response.push_str(response);
    }
}

#[cfg(test)]
mod tests {
    use super::VtResponseEngine;

    #[test]
    fn microsoft_operating_status_serializes_good_condition() {
        let mut responses = VtResponseEngine::default();
        responses.operating_status();
        assert_eq!(responses.response(), "\u{1b}[0n");
    }

    #[test]
    fn microsoft_cpr_is_viewport_relative_one_based_and_appends() {
        let mut responses = VtResponseEngine::default();

        // Microsoft PrepData(XCENTER, YCENTER): x=50, y=34, viewport top=20.
        responses.cursor_position_report(50, 34, 20);
        assert_eq!(responses.response(), "\u{1b}[15;51R");

        // The source test retains the first response, moves the cursor by +1,+1, and reports again.
        responses.cursor_position_report(51, 35, 20);
        assert_eq!(responses.response(), "\u{1b}[15;51R\u{1b}[16;52R");
    }

    #[test]
    fn microsoft_decxcpr_includes_current_page() {
        let mut responses = VtResponseEngine::default();

        responses.extended_cursor_position_report(50, 34, 20, 1);
        assert_eq!(responses.response(), "\u{1b}[?15;51;1R");

        responses.clear();
        responses.extended_cursor_position_report(50, 34, 20, 3);
        assert_eq!(responses.response(), "\u{1b}[?15;51;3R");
    }
}
