use crate::url_shortener::routes::json_response;
use crate::url_shortener::UrlShortenerService;
use actix_web::{http, web, HttpResponse};
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct FormData {
    url: Option<String>,
}

pub async fn shorten_url(
    form: web::Json<FormData>,
    svc: web::Data<Arc<Mutex<UrlShortenerService>>>,
) -> HttpResponse {

    let url = match &form.url {
        Some(url) => url,
        None => return error_response("url is required")
    };

    let svc: MutexGuard<UrlShortenerService> = svc.get_ref().lock().unwrap();

    match svc.validate_url(url) {
        Err(err) => return error_response(err),
        _ => {}
    }

    let entity_result = match svc.find_by_url(url).await {
        Ok(res) => res,
        Err(err) => return error_response(err)
    };

    let base_url = svc.get_url_prefix();

    if let Some(record) = entity_result { // entity already exists
        let tiny_url = record.to_tiny_url_response(base_url);

        return json_response(&tiny_url, http::StatusCode::OK);
    }

    return match svc.record_new_url(url).await {
        Ok(res) => {
            let tiny_url = res.to_tiny_url_response(base_url);

            json_response(&tiny_url, http::StatusCode::OK)
        },
        Err(err) => error_response(err)
    }
}

fn error_response(err : impl ToString) -> HttpResponse {
    let error = ErrorResponse {
        message: err.to_string(),
    };
    return json_response(&error, http::StatusCode::BAD_REQUEST);
}