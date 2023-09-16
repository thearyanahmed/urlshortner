use std::sync::{Arc, Mutex, MutexGuard};
use actix_web::{HttpResponse, http, web};
use crate::url_shortener::UrlShortenerService;
use crate::url_shortener::routes::json_response;

pub async fn health_check(svc: web::Data<Arc<Mutex<UrlShortenerService>>>) -> HttpResponse {
    let svc : MutexGuard<UrlShortenerService> = svc.get_ref().lock().unwrap();

    let data = svc.get_service_health();

    json_response(&data, http::StatusCode::OK)
}
