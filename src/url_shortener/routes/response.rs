use actix_web::{http, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

pub fn json_response<T: Serialize>(data: &T, status_code: actix_web::http::StatusCode) -> HttpResponse {
    match serde_json::to_string(data) {
        Ok(json) => HttpResponse::build(status_code).content_type("application/json").body(json),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn error_response(err: impl ToString, code: http::StatusCode) -> HttpResponse {
    let error = ErrorResponse {
        message: err.to_string(),
    };
    return json_response(&error, code);
}
