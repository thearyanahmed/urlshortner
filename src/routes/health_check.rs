use actix_web::{HttpResponse, http};
use serde::{Serialize};
use crate::routes::response::respond_with_json;

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

pub async fn health_check() -> HttpResponse {
    let data = HealthCheckResponse {
        status: "success".to_string(),
    };

    respond_with_json(&data, http::StatusCode::OK)
}
