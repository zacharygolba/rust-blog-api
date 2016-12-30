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

#[cfg(test)]
mod tests {
    use rocket;
    use rocket::http::Status;
    use rocket::http::Method::*;
    use rocket::testing::MockRequest;

    use super::*;

    #[derive(Deserialize, Serialize)]
    struct Params {
        message: String,
    }

    #[patch("/echo", data = "<params>", format="application/json")]
    fn echo(params: JSON<Params>) -> JSON<Params> {
        params
    }

    #[get("/error")]
    fn error() -> Result<(), ()> {
        Err(())
    }

    #[test]
    #[should_panic]
    fn test_bad_request() {
        let server = rocket::ignite()
            .mount("/", routes![echo])
            .catch(errors![bad_request]);

        let mut req = MockRequest::new(Patch, "/echo");
        let res = req.dispatch_with(&server);

        assert_eq!(res.status(), Status::BadRequest);
    }

    #[test]
    fn test_not_found() {
        let server = rocket::ignite()
            .mount("/", routes![])
            .catch(errors![not_found]);

        let mut req = MockRequest::new(Get, "/");
        let res = req.dispatch_with(&server);

        assert_eq!(res.status(), Status::NotFound);
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let server = rocket::ignite()
            .mount("/", routes![error])
            .catch(errors![internal_server_error]);

        let mut req = MockRequest::new(Get, "/error");
        let res = req.dispatch_with(&server);

        assert_eq!(res.status(), Status::InternalServerError);
    }
}
