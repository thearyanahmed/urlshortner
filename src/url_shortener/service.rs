use crate::url_shortener::configuration::Settings;
use crate::url_shortener::redis::RedisStore;
use crate::url_shortener::routes::{health_check, not_found, respond_with_json, testing_redis};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::types::chrono::{DateTime, Utc};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use serde::{Serialize};
use actix_web::HttpResponse;

// @todo add boot time
pub struct HttpRouter {}

pub struct UrlShortenerService {
    cache: Arc<Mutex<dyn CacheStore + Send + Sync>>,
    db: Arc<Mutex<dyn DataStore + Send + Sync>>,
}

impl UrlShortenerService {
    pub fn new(
        cache_store: impl CacheStore + Send + Sync + 'static,
        db_store: impl DataStore + Send + Sync + 'static,
    ) -> Self {
        let cache: Arc<Mutex<dyn CacheStore + Send + Sync>> = Arc::new(Mutex::new(cache_store));
        let db: Arc<Mutex<dyn DataStore + Send + Sync>> = Arc::new(Mutex::new(db_store));

        Self {
            cache,
            db,
        }
    }

    pub fn hello(&self) {
        let mut c = self.cache.lock().unwrap();

        let _ = c.find_by_key("a");

        let db = self.db.lock().unwrap();

        let _ = db.find_by_key("b");

        println!("hello world")
    }
}

pub trait DataStore {
    fn find_by_key(&self, key: &str) -> Result<String, String>;
    fn store(&self, key: &str) -> Result<String, String>;
}

pub trait CacheStore {
    fn find_by_key(&mut self, key: &str) -> Result<String, String>;
    fn store(&self, key: &str) -> Result<String, String>;
}

impl HttpRouter {
    pub async fn build_http_server(config: &Settings, svc: Arc<Mutex<UrlShortenerService>>) -> Result<(), std::io::Error> {
        let shared_app = web::Data::new(svc.clone());

        let address = format!("{}:{}", &config.base_url, &config.port);

        let listener = TcpListener::bind(&address)?;

        let server = HttpServer::new(move || {
            App::new()
                .route("/health-check", web::get().to(health_check))
                .route("/shorten", web::post().to(health_check))
                .route("/visit", web::get().to(testing_redis))
                .route("/do", web::get().to(test_run))
                .default_service(web::route().to(not_found))
                .app_data(shared_app.clone())
            })
            .listen(listener)?
            .run();

        server.await
    }
}


#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

pub async fn test_run(svc: web::Data<Arc<Mutex<UrlShortenerService>>>) -> HttpResponse {
    let x = svc.get_ref().lock().unwrap();
    x.hello();

    let data = HealthCheckResponse {
        status: "success pikachu".to_string(),
    };

    respond_with_json(&data, actix_web::http::StatusCode::OK)
}