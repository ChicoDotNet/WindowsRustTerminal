#![forbid(unsafe_code)]

mod fzf;

pub use fzf::{MatchResult, Pattern, TextRun, match_text, parse_pattern};
