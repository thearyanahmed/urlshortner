use crate::url_shortener::routes::json_response;
use crate::url_shortener::UrlShortenerService;
use actix_web::{http, web, HttpResponse};
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Serialize)]
struct Hello {
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
    // ensure the url is valid
    let url = match &form.url {
        Some(url) => url,
        None => {
            let error = Hello {
                message: "URL is required".to_string(),
            };
            return json_response(&error, http::StatusCode::BAD_REQUEST);
        }
    };

    let svc: MutexGuard<UrlShortenerService> = svc.get_ref().lock().unwrap();

    // check if the passed in url is a valid url or not
    match svc.validate_url(url) {
        Ok(_) => {}
        Err(err) => {
            let error = Hello {
                message: err.to_string(),
            };
            return json_response(&error, http::StatusCode::BAD_REQUEST);
        }
    }

    let entity_result = match svc.find_by_url(url).await {
        Ok(res) => res,
        Err(e) => {
            let error = Hello {
                message: e.to_string(),
            };
            return json_response(&error, http::StatusCode::BAD_REQUEST);
        },
    };
    // @todo remove this

    if let Some(record) = entity_result { // entity already exists
        return json_response(&record, http::StatusCode::OK);
    }

    let entity_result = match svc.record_new_url(url) {
        Ok(res) => res,
        Err(e) => {
            let error = Hello {
                message: e.to_string(),
            };
            return json_response(&error, http::StatusCode::BAD_REQUEST);
        },
    };
    // @todo remove this

    if let Some(record) = entity_result { // entity already exists
        return json_response(&record, http::StatusCode::OK);
    }

    let error = Hello {
        message: "URL is required".to_string(),
    };
    return json_response(&error, http::StatusCode::BAD_REQUEST);

}
