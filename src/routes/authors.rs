use chrono::UTC;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::status::NoContent;
use rocket_contrib::JSON;

use pool::CONNECTIONS;
use models::author::{Author, NewAuthor, AuthorChanges};

#[get("/<author_id>", format = "application/json")]
pub fn show(author_id: i64) -> Result<JSON<Author>, Failure> {
    use schema::authors::dsl::*;

    authors.find(author_id)
        .get_result::<Author>(&*CONNECTIONS.get().unwrap())
        .and_then(|author| Ok(JSON(author)))
        .or(Err(Failure(Status::NotFound)))
}

#[get("/", format = "application/json")]
pub fn index() -> Result<JSON<Vec<Author>>, Failure> {
    use schema::authors::dsl::*;

    authors.load::<Author>(&*CONNECTIONS.get().unwrap())
        .and_then(|results| Ok(JSON(results)))
        .or(Err(Failure(Status::InternalServerError)))
}

#[post("/", data = "<params>", format = "application/json")]
pub fn create(params: JSON<NewAuthor>) -> Result<JSON<Author>, Failure> {
    use schema::authors;

    diesel::insert(&params.unwrap())
        .into(authors::table)
        .get_result(&*CONNECTIONS.get().unwrap())
        .and_then(|author| Ok(JSON(author)))
        .or(Err(Failure(Status::InternalServerError)))
}

#[patch("/<author_id>", data = "<params>", format="application/json")]
pub fn update(author_id: i64, params: JSON<AuthorChanges>) -> Result<JSON<Author>, Failure> {
    use schema::authors::dsl::*;

    let mut change_set = params.unwrap();

    change_set.updated_at = Some(UTC::now());

    diesel::update(authors.find(author_id))
        .set(&change_set)
        .get_result::<Author>(&*CONNECTIONS.get().unwrap())
        .and_then(|author| Ok(JSON(author)))
        .or_else(|err| {
            match err {
                Error::NotFound => Err(Failure(Status::NotFound)),
                _ => Err(Failure(Status::InternalServerError)),
            }
        })
}

#[delete("/<author_id>", format = "application/json")]
pub fn destroy(author_id: i64) -> Result<NoContent, Failure> {
    use schema::authors::dsl::*;

    diesel::delete(authors.filter(id.eq(author_id)))
        .execute(&*CONNECTIONS.get().unwrap())
        .and(Ok(NoContent))
        .or(Err(Failure(Status::NotFound)))
}
