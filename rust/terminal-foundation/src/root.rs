#![forbid(unsafe_code)]

mod env;
#[path = "lib.rs"]
mod foundation;
mod uuid;

pub use env::Environment;
pub use foundation::*;
pub use uuid::{Guid, create_v5_uuid};
