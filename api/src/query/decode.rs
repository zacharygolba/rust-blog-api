use rocket::http::RawStr;

pub fn entry((key, value): (&RawStr, &RawStr)) -> Option<(String, String)> {
    key.url_decode()
       .map_err(|_| key.as_str().to_owned())
       .and_then(|k| {
        value.url_decode()
             .map_err(|_| value.as_str().to_owned())
             .map(|v| (k, v))
    })
       .ok()
}
