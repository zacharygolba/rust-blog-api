pub mod fields;
pub mod include;

use blog_db::models::author::{Author, NewAuthor};
use blog_db::pool::ConnectionPool;
use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket_contrib::Json;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::status::{Created, NoContent};
use rocket::State;
use serde_json::Value;

use error::{CreatedResult, JsonResult};
use query::{CollectionQuery, MemberQuery};

use self::fields::Fields;
use self::include::Include;

#[get("/<author_id>?<query>")]
pub fn show(
    author_id: i64,
    query: MemberQuery<Fields, Include>,
    pool: State<ConnectionPool>,
) -> JsonResult<Author> {
    use blog_db::authors::dsl::*;

    println!("{:#?}", query);

    authors.find(author_id)
           .first::<Author>(&*pool.get().unwrap())
           .map(|author| Json(author))
           .map_err(|err| match err {
        DieselError::NotFound => Failure(Status::NotFound),
        _ => Failure(Status::InternalServerError),
    })
}

#[get("/?<query>")]
pub fn index(
    query: CollectionQuery<Fields, Include>,
    pool: State<ConnectionPool>,
) -> Json<Vec<Author>> {
    use blog_db::authors::dsl::*;

    let page = query.page().unwrap_or_default();

    println!("{:#?}", query);

    authors.limit(page.size() as i64)
           .offset(page.offset() as i64)
           .load::<Author>(&*pool.get().unwrap())
           .map(|data| Json(data))
           .unwrap()
}

#[post("/", data = "<params>", format = "application/json")]
pub fn create(params: Json<NewAuthor>, pool: State<ConnectionPool>) -> CreatedResult<Author> {
    use blog_db::authors;

    diesel::insert(&*params)
        .into(authors::table)
        .get_result::<Author>(&*pool.get().unwrap())
        .and_then(|author| {
            let location = format!("http://localhost:3000/authors/{}", author.id);

            Ok(Created(location, Some(Json(author))))
        })
        .or(Err(Failure(Status::InternalServerError)))
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
