use actix_web::{HttpResponse,http};
use serde::{Serialize};
use crate::url_shortener::routes::respond_with_json;

#[derive(Serialize)]
struct NotFoundResponse {
    message: String,
}

pub async fn not_found() -> HttpResponse {
    let data = NotFoundResponse {
        message: "resource not found".to_string(),
    };

    respond_with_json(&data, http::StatusCode::NOT_FOUND)
}
