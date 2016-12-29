use chrono::UTC;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::status::{Created, NoContent};
use rocket_contrib::JSON;

use models::pool::CONNECTIONS;
use models::post::{Post, NewPost, PostChangeSet};

#[get("/<post_id>", format = "application/json")]
pub fn show(post_id: i64) -> Result<JSON<Post>, Failure> {
    use models::schema::posts::dsl::*;

    posts.find(post_id)
        .get_result::<Post>(&*CONNECTIONS.get().unwrap())
        .and_then(|post| Ok(JSON(post)))
        .or(Err(Failure(Status::NotFound)))
}

#[get("/", format = "application/json")]
pub fn index() -> Result<JSON<Vec<Post>>, Failure> {
    use models::schema::posts::dsl::*;

    posts.load::<Post>(&*CONNECTIONS.get().unwrap())
        .and_then(|results| Ok(JSON(results)))
        .or(Err(Failure(Status::InternalServerError)))
}

#[post("/", data = "<params>", format = "application/json")]
pub fn create(params: JSON<NewPost>) -> Result<Created<JSON<Post>>, Failure> {
    use models::schema::posts;

    diesel::insert(&params.unwrap())
        .into(posts::table)
        .get_result::<Post>(&*CONNECTIONS.get().unwrap())
        .and_then(|post| {
            let location = format!("http://localhost:8000/posts/{}", post.id);

            Ok(Created(location, Some(JSON(post))))
        })
        .or(Err(Failure(Status::InternalServerError)))
}

#[patch("/<post_id>", data = "<params>", format="application/json")]
pub fn update(post_id: i64, params: JSON<PostChangeSet>) -> Result<JSON<Post>, Failure> {
    use models::schema::posts::dsl::*;

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
    use models::schema::posts::dsl::*;

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&*CONNECTIONS.get().unwrap())
        .and(Ok(NoContent))
        .or(Err(Failure(Status::NotFound)))
}

#[cfg(test)]
mod tests {
    use rocket;
    use rocket::http::Method::*;
    use rocket::testing::MockRequest;

    use super::*;

    #[test]
    fn test_show() {
        let server = rocket::ignite().mount("/", routes![show]);
        let mut req = MockRequest::new(Get, "/1");
        let res = req.dispatch_with(&server);

        assert_eq!(res.status(), Status::Ok);
    }

    #[test]
    fn test_index() {
        let server = rocket::ignite().mount("/", routes![index]);
        let mut req = MockRequest::new(Get, "/");
        let res = req.dispatch_with(&server);

        assert_eq!(res.status(), Status::Ok);
    }
}
