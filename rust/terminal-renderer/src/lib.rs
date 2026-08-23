#![forbid(unsafe_code)]

mod attribute_color_policy;
mod css_length_percentage;
mod font_info_desired_policy;
mod font_info_policy;
mod render_settings_policy;
mod retry_policy;
mod timer_policy;
mod title_state;

pub use attribute_color_policy::{
    AttributeColorFlags, AttributeColors, apply_attribute_alpha, apply_attribute_effects,
};
pub use css_length_percentage::{CssLengthPercentage, ReferenceFrame};
pub use font_info_desired_policy::{CellSize, FontInfoDesiredPolicy};
pub use font_info_policy::{FontCellSizes, validate_font_cell_sizes};
pub use render_settings_policy::{RenderMode, RenderSettingsPolicy};
pub use retry_policy::{
    MAX_RETRIES_FOR_RENDER_ENGINE, RENDER_BACKOFF_BASE_MILLIS, RenderAttempt, render_attempts,
};
pub use timer_policy::{
    TIMER_REPR_MAX, TimerRepr, reschedule_repeating_timer, saturating_timer_add,
    saturating_timer_sub, timer_to_millis,
};
pub use title_state::{TitleState, TitleUpdate};
