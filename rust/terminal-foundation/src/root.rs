#![forbid(unsafe_code)]

#[path = "lib.rs"]
mod foundation;
mod uuid;

pub use foundation::*;
pub use uuid::{Guid, create_v5_uuid};
