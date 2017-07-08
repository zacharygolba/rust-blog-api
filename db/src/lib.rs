#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;

pub mod models;
pub mod schema;

pub use self::diesel::prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
