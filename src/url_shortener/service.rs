use crate::url_shortener::configuration::Settings;
use crate::url_shortener::redis::RedisStore;
use crate::url_shortener::routes::{health_check, not_found, respond_with_json};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App};
use sqlx::types::chrono::{DateTime, Utc};
use std::net::TcpListener;
use std::sync::{Arc, Mutex, MutexGuard};
use serde::{Serialize};
use actix_web::HttpResponse;
use actix_web::{HttpServer as ActixHttpServer};

pub trait DataStore {
    fn find_by_key(&self, key: &str) -> Result<String, String>;
    fn store(&self, key: &str) -> Result<String, String>;
}

pub trait CacheStore {
    fn ping(&mut self) -> Result<bool, String>;
    fn find_by_key(&mut self, key: &str) -> Result<String, String>;
    fn store(&self, key: &str) -> Result<String, String>;
}

pub struct HttpServer {}

pub struct UrlShortenerService {
    cache: Arc<Mutex<dyn CacheStore + Send + Sync>>,
    db: Arc<Mutex<dyn DataStore + Send + Sync>>,
}

#[derive(Serialize)]
pub struct ServiceHealth {
    redis_health: bool,
    db_health: bool,
    reporting_time: String,
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

    pub fn get_service_health(&self) -> ServiceHealth {
        let mut c = self.cache.lock().unwrap();

        let _ = c.find_by_key("a");

        let mut health_status = ServiceHealth {
            redis_health: false,
            db_health: false,
            reporting_time: Utc::now().to_string(),
        };

        match c.ping() {
            Ok(res) => health_status.redis_health = res,
            Err(e) => {} // @todo log / info
        }

        let db = self.db.lock().unwrap();

        let _ = db.find_by_key("b");

        health_status
    }
}

impl HttpServer {
    pub async fn listen_and_serve(config: &Settings, svc: Arc<Mutex<UrlShortenerService>>) -> Result<(), std::io::Error> {
        let shared_app = web::Data::new(svc.clone());

        let address = format!("{}:{}", &config.base_url, &config.port);

        let listener = TcpListener::bind(&address)?;

        let server = ActixHttpServer::new(move || {
            App::new()
                .route("/health-check", web::get().to(health_check))
                .route("/shorten", web::post().to(health_check))
                .default_service(web::route().to(not_found))
                .app_data(shared_app.clone())
        })
            .listen(listener)?
            .run();

        server.await
    }
}
