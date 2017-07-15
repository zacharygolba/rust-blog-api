use rocket_contrib::Json;
use serde_json::Value;

#[get("/health", format = "application/json")]
pub fn health() -> Json<Value> {
    let resp = json!({
        "detail": "OK",
        "status": "200",
    });

    Json(resp)
}
