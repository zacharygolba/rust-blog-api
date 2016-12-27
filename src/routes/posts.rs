use chrono::UTC;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::status::NoContent;
use rocket_contrib::JSON;

use pool::CONNECTIONS;
use models::post::{Post, NewPost, PostChanges};

#[get("/<post_id>", format = "application/json")]
pub fn show(post_id: i64) -> Result<JSON<Post>, Failure> {
    use schema::posts::dsl::*;

    posts.find(post_id)
        .get_result::<Post>(&*CONNECTIONS.get().unwrap())
        .and_then(|post| Ok(JSON(post)))
        .or(Err(Failure(Status::NotFound)))
}

#[get("/", format = "application/json")]
pub fn index() -> Result<JSON<Vec<Post>>, Failure> {
    use schema::posts::dsl::*;

    posts.load::<Post>(&*CONNECTIONS.get().unwrap())
        .and_then(|results| Ok(JSON(results)))
        .or(Err(Failure(Status::InternalServerError)))
}

#[post("/", data = "<params>", format = "application/json")]
pub fn create(params: JSON<NewPost>) -> Result<JSON<Post>, Failure> {
    use schema::posts;

    diesel::insert(&params.unwrap())
        .into(posts::table)
        .get_result(&*CONNECTIONS.get().unwrap())
        .and_then(|post| Ok(JSON(post)))
        .or(Err(Failure(Status::InternalServerError)))
}

#[patch("/<post_id>", data = "<params>", format="application/json")]
pub fn update(post_id: i64, params: JSON<PostChanges>) -> Result<JSON<Post>, Failure> {
    use schema::posts::dsl::*;

    let mut change_set = params.unwrap();

    change_set.updated_at = Some(UTC::now());

    diesel::update(posts.find(post_id))
        .set(&change_set)
        .get_result::<Post>(&*CONNECTIONS.get().unwrap())
        .and_then(|post| Ok(JSON(post)))
        .or_else(|err| {
            match err {
                Error::NotFound => Err(Failure(Status::NotFound)),
                _ => Err(Failure(Status::InternalServerError)),
            }
        })
}

#[delete("/<post_id>", format = "application/json")]
pub fn destroy(post_id: i64) -> Result<NoContent, Failure> {
    use schema::posts::dsl::*;

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&*CONNECTIONS.get().unwrap())
        .and(Ok(NoContent))
        .or(Err(Failure(Status::NotFound)))
}
