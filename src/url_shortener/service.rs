use crate::url_shortener::configuration::Settings;
use crate::url_shortener::routes::{health_check, not_found};
use actix_web::{web, App};
use sqlx::types::chrono::{Utc};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use actix_web::{HttpServer as ActixHttpServer};
use async_trait::async_trait;
use serde::Serialize;
use log::{info};

#[async_trait]
pub trait DataStore {
    async fn find_by_key(&self, key: &str) -> Result<String, String>;
    fn store(&self, key: &str) -> Result<String, String>;
    fn is_alive(&self) -> bool;
}

pub trait CacheStore {
    fn is_alive(&mut self) -> bool;
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
    cache_is_alive: bool,
    db_is_alive: bool,
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
        let db = self.db.lock().unwrap();

        let health_status = ServiceHealth {
            cache_is_alive: c.is_alive(),
            db_is_alive: db.is_alive(),
            reporting_time: Utc::now().to_string(),
        };

        let _ = db.find_by_key("b");

        println!("connection {}", db.is_alive());

        health_status
    }
}

impl HttpServer {
    pub async fn listen_and_serve(config: &Settings, svc: Arc<Mutex<UrlShortenerService>>) -> Result<(), std::io::Error> {
        let shared_app = web::Data::new(svc.clone());

        let address = format!("{}:{}", &config.base_url, &config.port);

        info!("serving on {}", address);

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
