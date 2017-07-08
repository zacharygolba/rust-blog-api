mod authors;
mod comments;
mod posts;
mod tags;

use rocket_contrib::JSON;
use rocket::Route;
use serde_json::Value;

pub use self::authors::*;
pub use self::comments::*;
pub use self::posts::*;
pub use self::tags::*;

pub fn root() -> Vec<Route> {
    routes![
        health,
    ]
}

#[get("/health", format = "application/json")]
fn health() -> JSON<Value> {
    let resp = json!({
        "detail": "OK",
        "status": "200",
    });

    JSON(resp)
}
