#![forbid(unsafe_code)]

#[path = "lib.rs"]
mod foundation;
mod env;
mod uuid;

pub use env::Environment;
pub use foundation::*;
pub use uuid::{Guid, create_v5_uuid};
