#![forbid(unsafe_code)]

mod commandline;
mod fzf;

pub use commandline::{
    AppCommandlineArgs, Commandline, CommandlineError, FocusDirection, LaunchMode,
    NewTerminalArgs, SplitDirection, SplitType, StartupAction, build_commands,
    convert_execute_commandline_to_actions, parse_startup,
};
pub use fzf::{MatchResult, Pattern, TextRun, match_text, parse_pattern};
