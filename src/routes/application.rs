use std::collections::HashMap;

use rocket_contrib::JSON;

#[get("/health", format = "application/json")]
pub fn health() -> JSON<HashMap<&'static str, &'static str>> {
    JSON(map!{
        "status" => "200",
        "detail" => "OK",
    })
}

#[cfg(test)]
mod tests {
    use rocket;
    use rocket::http::Status;
    use rocket::http::Method::*;
    use rocket::testing::MockRequest;

    use super::*;

    #[test]
    fn test_health() {
        let server = rocket::ignite().mount("/", routes![health]);
        let mut req = MockRequest::new(Get, "/health");
        let res = req.dispatch_with(&server);

        assert_eq!(res.status(), Status::Ok);
    }
}
