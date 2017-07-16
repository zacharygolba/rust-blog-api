use blog_db::models::author::Author;
use blog_db::pool::ConnectionPool;
use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket_contrib::Json;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::status::NoContent;
use rocket::State;
use serde_json::Value;

use error::JsonResult;
use query::MemberParams;

use super::fields::Fields;
use super::include::Include;

type Params<'req> = MemberParams<'req, Fields, Include>;

#[get("/<author_id>?<params>")]
pub fn show(author_id: i64, params: Params, pool: State<ConnectionPool>) -> JsonResult<Author> {
    use blog_db::authors::dsl::*;

    println!("{:#?}", params);

    authors.find(author_id)
           .first::<Author>(&*pool.get().unwrap())
           .map(|author| Json(author))
           .map_err(|err| match err {
        DieselError::NotFound => Failure(Status::NotFound),
        _ => Failure(Status::InternalServerError),
    })
}

#[patch("/<author_id>", format = "application/json")]
pub fn update(author_id: i64) -> Json<Value> {
    let author_id = author_id.to_string();
    let resp = json!({
        "data": {
            "id": author_id,
            "type": "authors",
            "attributes": {},
        },
        "links": {
            "self": format!("http://localhost:3000/authors/{}", author_id),
        },
    });

    Json(resp)
}

#[delete("/<author_id>")]
pub fn destroy(author_id: i64, pool: State<ConnectionPool>) -> Result<NoContent, Failure> {
    use blog_db::authors::dsl::*;

    diesel::delete(authors.filter(id.eq(author_id)))
        .execute(&*pool.get().unwrap())
        .map(|_| NoContent)
        .map_err(|err| match err {
            DieselError::NotFound => Failure(Status::NotFound),
            _ => Failure(Status::InternalServerError),
        })
}

#[cfg(test)]
mod tests {}
