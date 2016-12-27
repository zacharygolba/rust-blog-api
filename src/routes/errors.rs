use std::collections::HashMap;

use rocket_contrib::JSON;

type ErrorMap = HashMap<&'static str, Vec<HashMap<&'static str, &'static str>>>;

#[error(400)]
pub fn bad_request() -> JSON<ErrorMap> {
    JSON(map!{
        "errors" => vec![
            map!{
                "status" => "400",
                "detail" => "Bad Request",
            },
        ],
    })
}

#[error(404)]
pub fn not_found() -> JSON<ErrorMap> {
    JSON(map!{
        "errors" => vec![
            map!{
                "status" => "404",
                "detail" => "Not Found",
            },
        ],
    })
}

#[error(500)]
pub fn internal_server_error() -> JSON<ErrorMap> {
    JSON(map!{
        "errors" => vec![
            map!{
                "status" => "500",
                "detail" => "Internal Server Error",
            },
        ],
    })
}
