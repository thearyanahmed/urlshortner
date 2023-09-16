use crate::url_shortener::routes::{error_response};
use crate::url_shortener::UrlShortenerService;
use actix_web::{http, web, HttpResponse};
use actix_web::Result;
use std::sync::{Arc, Mutex, MutexGuard};

pub async fn visit(
    params: web::Path<String>,
    svc: web::Data<Arc<Mutex<UrlShortenerService>>>,
) -> Result<HttpResponse> {
    let key = params.into_inner();
    let key: &str = &key;

    let svc: MutexGuard<UrlShortenerService> = svc.get_ref().lock().unwrap();

    let result = match svc.get_cache_record_by_key(key).await {
        Ok(res) => res,
        Err(_) => None, // pass, we check in db
    };

    if let Some(record) = result {
        let _ = svc.store_visit_in_db(key).await;

        return Ok(
            redirect_away(record.original_url)
        );
    }

    let entity_result = match svc.get_db_record_by_key(key).await {
        Ok(res) => res,
        Err(_err) => None,
    };

    if let Some(record) = entity_result { // entity found
        let _ = svc.store_visit_in_db(key).await;

        let original_url: &str = &record.original_url;
        let _= svc.store_new_url_in_cache(key, original_url);

        return Ok(
            redirect_away(record.original_url)
        );
    }

    Ok(error_response("resource not found".to_string(), http::StatusCode::NOT_FOUND))
}

fn redirect_away(to: String) -> HttpResponse {
    HttpResponse::PermanentRedirect()
        .append_header(("Location", to))
        .finish()
}