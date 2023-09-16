use actix_web::{HttpResponse,http};
use serde::Serialize;
use crate::url_shortener::routes::json_response;

#[derive(Serialize)]
struct NotFoundResponse {
    message: String,
}

pub async fn not_found() -> HttpResponse {
    let data = NotFoundResponse {
        message: "resource not found".to_string(),
    };

    json_response(&data, http::StatusCode::NOT_FOUND)
}
