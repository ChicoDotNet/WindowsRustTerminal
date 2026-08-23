#![forbid(unsafe_code)]

mod css_length_percentage;
mod render_settings_policy;
mod retry_policy;
mod timer_policy;
mod title_state;

pub use css_length_percentage::{CssLengthPercentage, ReferenceFrame};
pub use render_settings_policy::{RenderMode, RenderSettingsPolicy};
pub use retry_policy::{
    MAX_RETRIES_FOR_RENDER_ENGINE, RENDER_BACKOFF_BASE_MILLIS, RenderAttempt, render_attempts,
};
pub use timer_policy::{
    TIMER_REPR_MAX, TimerRepr, reschedule_repeating_timer, saturating_timer_add,
    saturating_timer_sub, timer_to_millis,
};
pub use title_state::{TitleState, TitleUpdate};
