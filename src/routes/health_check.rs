use actix_web::{HttpResponse, web};
use serde::{Serialize};

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

pub async fn health_check() -> HttpResponse {
    let data = HealthCheckResponse {
        status: "success".to_string(),
    };

    match serde_json::to_string(&data) {
        Ok(json) => HttpResponse::Ok().content_type("application/json").body(json),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
