#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate inflector;
extern crate rocket;
extern crate serde;
extern crate serde_json;

pub mod error;
pub mod jsonapi;

pub use error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
