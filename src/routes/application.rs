use std::collections::HashMap;

use rocket_contrib::JSON;

#[get("/health", format = "application/json")]
pub fn health() -> JSON<HashMap<&'static str, &'static str>> {
    JSON(map!{
        "status" => "200",
        "detail" => "OK",
    })
}
