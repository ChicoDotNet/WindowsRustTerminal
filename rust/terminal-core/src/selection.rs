//! Platform-neutral selection state and pivot behavior from `TerminalCore`.
//!
//! This module deliberately stops below text-buffer-dependent word/line expansion.
//! It captures the deterministic state machine that decides which endpoint moves
//! and how the immutable pivot keeps a drag selection ordered.

/// A cell position in the terminal text buffer.
///
/// Ordering is row-major, matching `til::point` comparisons used by TerminalCore:
/// rows compare first, then columns within a row.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BufferPoint {
    pub x: i32,
    pub y: i32,
}

impl BufferPoint {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Ord for BufferPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for BufferPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The selection expansion policy selected by mouse/keyboard interaction.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectionExpansion {
    #[default]
    Char,
    Word,
    Line,
}

/// Which endpoint Mark Mode currently moves.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectionEndpoint {
    Start,
    End,
    #[default]
    Both,
}

/// Interaction mode associated with the active selection.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectionInteractionMode {
    #[default]
    None,
    Mouse,
    Mark,
}

/// Mutable TerminalCore selection state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelectionInfo {
    pub start: BufferPoint,
    pub end: BufferPoint,
    pub pivot: BufferPoint,
    pub block_selection: bool,
    pub active: bool,
}

impl SelectionInfo {
    /// Starts a character selection at `point`, preserving the pivot until a new
    /// selection is created.
    #[must_use]
    pub const fn anchored(point: BufferPoint) -> Self {
        Self {
            start: point,
            end: point,
            pivot: point,
            block_selection: false,
            active: true,
        }
    }

    /// Returns ordered anchors around the immutable pivot and tells the caller
    /// whether the moving target is the start endpoint.
    ///
    /// This is the safe Rust equivalent of `Terminal::_PivotSelection`.
    #[must_use]
    pub fn pivot_selection(&self, target: BufferPoint) -> PivotedSelection {
        if target <= self.pivot {
            PivotedSelection {
                start: target,
                end: self.pivot,
                target_start: true,
            }
        } else {
            PivotedSelection {
                start: self.pivot,
                end: target,
                target_start: false,
            }
        }
    }

    pub fn set_block_selection(&mut self, enabled: bool) {
        self.block_selection = enabled;
    }

    pub fn clear(&mut self) {
        self.active = false;
    }
}

/// Result of pivoting a drag target around the selection pivot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PivotedSelection {
    pub start: BufferPoint,
    pub end: BufferPoint,
    pub target_start: bool,
}

/// Deterministic endpoint switching used by Mark Mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EndpointState {
    pub target: SelectionEndpoint,
    pub anchor_inactive_endpoint: bool,
}

impl EndpointState {
    /// Mirrors `Terminal::SwitchSelectionEndpoint` while leaving pivot mutation
    /// explicit and testable.
    pub fn switch(&mut self, selection: &mut SelectionInfo) {
        if !selection.active {
            return;
        }

        match self.target {
            SelectionEndpoint::Both => {
                self.target = SelectionEndpoint::End;
                self.anchor_inactive_endpoint = true;
            }
            SelectionEndpoint::End => {
                self.target = SelectionEndpoint::Start;
                selection.pivot = selection.end;
            }
            SelectionEndpoint::Start => {
                self.target = SelectionEndpoint::End;
                selection.pivot = selection.start;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BufferPoint, EndpointState, SelectionEndpoint, SelectionInfo};

    #[test]
    fn buffer_points_are_row_major() {
        assert!(BufferPoint::new(99, 3) < BufferPoint::new(0, 4));
        assert!(BufferPoint::new(2, 4) < BufferPoint::new(3, 4));
    }

    #[test]
    fn pivot_selection_keeps_pivot_selected_when_drag_crosses_it() {
        let selection = SelectionInfo::anchored(BufferPoint::new(5, 2));

        let forward = selection.pivot_selection(BufferPoint::new(9, 2));
        assert_eq!(forward.start, BufferPoint::new(5, 2));
        assert_eq!(forward.end, BufferPoint::new(9, 2));
        assert!(!forward.target_start);

        let backward = selection.pivot_selection(BufferPoint::new(1, 1));
        assert_eq!(backward.start, BufferPoint::new(1, 1));
        assert_eq!(backward.end, BufferPoint::new(5, 2));
        assert!(backward.target_start);
    }

    #[test]
    fn pivot_equality_targets_start_like_terminal_core() {
        let selection = SelectionInfo::anchored(BufferPoint::new(5, 2));
        let pivoted = selection.pivot_selection(selection.pivot);

        assert!(pivoted.target_start);
        assert_eq!(pivoted.start, selection.pivot);
        assert_eq!(pivoted.end, selection.pivot);
    }

    #[test]
    fn switching_both_endpoints_targets_end_and_anchors_inactive_side() {
        let mut selection = SelectionInfo::anchored(BufferPoint::new(2, 3));
        let mut endpoints = EndpointState::default();

        endpoints.switch(&mut selection);

        assert_eq!(endpoints.target, SelectionEndpoint::End);
        assert!(endpoints.anchor_inactive_endpoint);
        assert_eq!(selection.pivot, BufferPoint::new(2, 3));
    }

    #[test]
    fn switching_end_to_start_pivots_on_end() {
        let mut selection = SelectionInfo {
            start: BufferPoint::new(1, 3),
            end: BufferPoint::new(7, 3),
            pivot: BufferPoint::new(1, 3),
            block_selection: false,
            active: true,
        };
        let mut endpoints = EndpointState {
            target: SelectionEndpoint::End,
            anchor_inactive_endpoint: false,
        };

        endpoints.switch(&mut selection);

        assert_eq!(endpoints.target, SelectionEndpoint::Start);
        assert_eq!(selection.pivot, selection.end);
    }

    #[test]
    fn switching_start_to_end_pivots_on_start() {
        let mut selection = SelectionInfo {
            start: BufferPoint::new(1, 3),
            end: BufferPoint::new(7, 3),
            pivot: BufferPoint::new(7, 3),
            block_selection: false,
            active: true,
        };
        let mut endpoints = EndpointState {
            target: SelectionEndpoint::Start,
            anchor_inactive_endpoint: false,
        };

        endpoints.switch(&mut selection);

        assert_eq!(endpoints.target, SelectionEndpoint::End);
        assert_eq!(selection.pivot, selection.start);
    }

    #[test]
    fn inactive_selection_does_not_switch_endpoints() {
        let mut selection = SelectionInfo::default();
        let mut endpoints = EndpointState::default();

        endpoints.switch(&mut selection);

        assert_eq!(endpoints, EndpointState::default());
    }
}
