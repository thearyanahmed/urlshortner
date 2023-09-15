use actix_web::{HttpResponse};
use serde::Serialize;

// @todo refactor
pub fn respond_with_json<T: Serialize>(data: &T, status_code: actix_web::http::StatusCode) -> HttpResponse {
    match serde_json::to_string(data) {
        Ok(json) => HttpResponse::build(status_code).content_type("application/json").body(json),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}