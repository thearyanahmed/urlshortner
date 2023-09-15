use std::sync::Mutex;
use actix_web::{HttpResponse, http, web};
use serde::{Serialize};
use crate::url_shortener::CacheStore;
use crate::url_shortener::routes::respond_with_json;

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

pub async fn testing_redis(service: web::Data<Mutex<dyn CacheStore + Send + Sync>> ) -> HttpResponse {
    let mut cache = service.lock().unwrap();

    match cache.find_by_key("hello") {
        Ok(_) => print!("ok,"),
        Err(_) => print!("not ok,"),
    }

    let data = HealthCheckResponse {
        status: "bleh success".to_string(),
    };

    respond_with_json(&data, http::StatusCode::OK)
}
