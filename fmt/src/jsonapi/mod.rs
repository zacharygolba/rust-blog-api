#[macro_use]
mod macros;

pub mod document;
pub mod resource;
pub mod types;


mod statics;

pub use self::document::Document;
pub use self::resource::{IntoResource, Resource};
pub use self::types::*;
