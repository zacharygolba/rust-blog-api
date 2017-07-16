use blog_db::models::author::{Author, NewAuthor};
use blog_db::pool::ConnectionPool;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::status::Created;
use rocket::State;

use error::CreatedResult;
use query::CollectionParams;

use super::params::{Fields, Include};

type Params<'req> = CollectionParams<'req, Fields, Include>;

#[get("/?<params>")]
pub fn index(params: Params, pool: State<ConnectionPool>) -> Json<Vec<Author>> {
    use blog_db::authors::dsl::*;

    println!("{:#?}", params);

    let page = params.page();

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

#[cfg(test)]
mod tests {}
