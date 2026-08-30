#![forbid(unsafe_code)]

mod env;
#[path = "lib.rs"]
mod foundation;
mod throttled;
mod til_math;
mod types_utils;
mod uuid;

pub use env::Environment;
pub use foundation::*;
pub use throttled::{Throttled, ThrottledError, ThrottledOptions};
pub use til_math::{IntegralRound, MathNarrowingError, checked_round_i32};
pub use types_utils::{clamp_to_short_max, evaluate_starting_directory, filter_string_for_paste, split_string, string_to_uint, trim_paste};
pub use uuid::{Guid, create_v5_uuid};
