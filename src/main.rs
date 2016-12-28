#![feature(plugin, proc_macro, slice_patterns)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod models;
mod routes;

fn main() {
    routes::mount(rocket::ignite()).launch();
}
