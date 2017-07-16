#![feature(custom_attribute, plugin, try_from)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;

// #[macro_use]
// extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate diesel;
extern crate dotenv;
extern crate rayon;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

extern crate blog_db;
extern crate blog_fmt;

mod error;
mod routes;
mod query;

use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes![routes::root::health,])
        .mount(
            "/authors",
            routes![
                routes::authors::create,
                routes::authors::destroy,
                routes::authors::index,
                routes::authors::show,
                routes::authors::update,
            ],
        )
        .mount(
            "/comments",
            routes![
                routes::comments::create,
                routes::comments::destroy,
                routes::comments::index,
                routes::comments::show,
                routes::comments::update,
            ],
        )
        .mount(
            "/posts",
            routes![
                routes::posts::create,
                routes::posts::destroy,
                routes::posts::index,
                routes::posts::show,
                routes::posts::update,
            ],
        )
        .mount(
            "/tags",
            routes![
                routes::tags::create,
                routes::tags::destroy,
                routes::tags::index,
                routes::tags::show,
                routes::tags::update,
            ],
        )
        .manage({
            let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

            blog_db::connect(url).expect("Failed to create pool.")
        })
        .launch();
}
