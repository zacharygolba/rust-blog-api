use chrono::{DateTime, Utc};
use rocket_contrib::JSON;
use rocket::response::status::NoContent;
use rocket::Route;
use serde_json::Value;

use error::{Result, JSONResult};

pub fn posts() -> Vec<Route> {
    routes![
        create,
        destroy,
        index,
        show,
        update,
    ]
}

#[get("/<id>")]
fn show(id: i64) -> JSONResult<Value> {
    let id = id.to_string();
    let resp = json!({
        "data": {
            "id": &id,
            "type": "posts",
            "attributes": {},
        },
        "links": {
            "self": format!("http://localhost:3000/posts/{}", &id),
        },
    });

    Ok(JSON(resp))
}

#[get("/")]
fn index() -> JSONResult<Value> {
    let resp = json!({
        "data": [],
        "links": {
            "self": "http://localhost:3000/posts",
            "first": "http://localhost:3000/posts",
            "prev": null,
            "next": null,
            "last": "http://localhost:3000/posts",
        },
    });

    Ok(JSON(resp))
}

#[post("/", format = "application/vnd.api+json")]
fn create() -> JSONResult<Value> {
    let id = "1";
    let resp = json!({
        "data": {
            "id": id,
            "type": "posts",
            "attributes": {},
        },
        "links": {
            "self": format!("http://localhost:3000/posts/{}", id),
        },
    });

    Ok(JSON(resp))
}

#[patch("/<id>", format = "application/vnd.api+json")]
fn update(id: i64) -> JSONResult<Value> {
    let id = id.to_string();
    let resp = json!({
        "data": {
            "id": id,
            "type": "posts",
            "attributes": {},
        },
        "links": {
            "self": format!("http://localhost:3000/posts/{}", id),
        },
    });

    Ok(JSON(resp))
}

#[delete("/<id>")]
fn destroy(id: i64) -> Result<NoContent> {
    Ok(NoContent)
}

#[cfg(test)]
mod tests {}
