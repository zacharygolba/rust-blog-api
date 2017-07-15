#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate dotenv;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate serde;

extern crate blog_fmt;

pub mod model;
pub mod models;
pub mod pool;
pub mod prelude;
pub mod schema;
pub mod types;

pub use self::diesel::{delete, insert, update};
pub use self::models::*;
pub use self::pool::connect;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
