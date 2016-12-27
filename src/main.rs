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

mod pool;
mod models;
mod schema;
mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::health
        ])
        .mount("/posts", routes![
            routes::posts::show,
            routes::posts::index,
            routes::posts::create,
            routes::posts::update,
            routes::posts::destroy,
        ])
        .mount("/authors", routes![
            routes::authors::show,
            routes::authors::index,
            routes::authors::create,
            routes::authors::update,
            routes::authors::destroy,
        ])
        .catch(errors![
            routes::errors::not_found,
            routes::errors::bad_request,
            routes::errors::internal_server_error,
        ])
        .launch();
}
