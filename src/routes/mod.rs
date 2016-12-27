use std::collections::HashMap;

use rocket_contrib::JSON;

pub mod posts;
pub mod errors;

#[get("/health", format = "application/json")]
pub fn health() -> JSON<HashMap<&'static str, &'static str>> {
    JSON(map!{
        "status" => "200",
        "detail" => "OK",
    })
}
