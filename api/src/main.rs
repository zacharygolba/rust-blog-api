#![feature(custom_attribute, plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate rocket;
extern crate serde;

extern crate blog_db;

mod error;
mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes::root())
        .mount("/authors", routes::authors())
        .mount("/comments", routes::comments())
        .mount("/posts", routes::posts())
        .mount("/tags", routes::tags())
        .launch();
}
