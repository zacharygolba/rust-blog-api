use rocket;

mod application;
mod authors;
mod errors;
mod posts;

pub fn mount(server: rocket::Rocket) -> rocket::Rocket {
    server.mount("/", routes![application::health])
        .mount("/posts", routes![
            posts::show,
            posts::index,
            posts::create,
            posts::update,
            posts::destroy,
        ])
        .mount("/authors", routes![
            authors::show,
            authors::index,
            authors::create,
            authors::update,
            authors::destroy,
        ])
        .catch(errors![
            errors::not_found,
            errors::bad_request,
            errors::internal_server_error,
        ])
}
