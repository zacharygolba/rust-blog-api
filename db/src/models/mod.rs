pub mod author;
pub mod comment;
pub mod post;
pub mod tag;

use diesel::prelude::*;

pub use self::author::Author;
pub use self::comment::Comment;
pub use self::post::Post;
pub use self::tag::Tag;

pub trait Model {}
