#![allow(unused_variables)]

use rocket_contrib::Json;
use rocket::response::status::NoContent;
use serde_json::Value;

use error::{Result, JsonResult};

#[get("/<id>")]
pub fn show(id: i64) -> JsonResult<Value> {
    let id = id.to_string();
    let resp = json!({
        "data": {
            "id": &id,
            "type": "tags",
            "attributes": {},
        },
        "links": {
            "self": format!("http://localhost:3000/tags/{}", &id),
        },
    });

    Ok(Json(resp))
}

#[get("/")]
pub fn index() -> JsonResult<Value> {
    let resp = json!({
        "data": [],
        "links": {
            "self": "http://localhost:3000/tags",
            "first": "http://localhost:3000/tags",
            "prev": null,
            "next": null,
            "last": "http://localhost:3000/tags",
        },
    });

    Ok(Json(resp))
}

#[post("/", format = "application/json")]
pub fn create() -> JsonResult<Value> {
    let id = "1";
    let resp = json!({
        "data": {
            "id": id,
            "type": "tags",
            "attributes": {},
        },
        "links": {
            "self": format!("http://localhost:3000/tags/{}", id),
        },
    });

    Ok(Json(resp))
}

#[patch("/<id>", format = "application/json")]
pub fn update(id: i64) -> JsonResult<Value> {
    let id = id.to_string();
    let resp = json!({
        "data": {
            "id": id,
            "type": "tags",
            "attributes": {},
        },
        "links": {
            "self": format!("http://localhost:3000/tags/{}", id),
        },
    });

    Ok(Json(resp))
}

#[delete("/<id>")]
pub fn destroy(id: i64) -> Result<NoContent> {
    Ok(NoContent)
}

#[cfg(test)]
mod tests {}
