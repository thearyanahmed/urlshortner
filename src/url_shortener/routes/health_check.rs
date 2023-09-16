use std::sync::{Arc, Mutex, MutexGuard};
use actix_web::{HttpResponse, http, web};
use serde::{Serialize};
use crate::url_shortener::{UrlShortenerService};
use crate::url_shortener::routes::respond_with_json;

pub async fn health_check(svc: web::Data<Arc<Mutex<UrlShortenerService>>>) -> HttpResponse {
    let svc : MutexGuard<UrlShortenerService> = svc.get_ref().lock().unwrap();

    let data = svc.get_service_health();

    respond_with_json(&data, http::StatusCode::OK)
}
