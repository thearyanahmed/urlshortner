use crate::url_shortener::configuration::Settings;
use crate::url_shortener::routes::{health_check, not_found, shorten_url};
use crate::url_shortener::Url as UrlEntity;
use actix_web::HttpServer as ActixHttpServer;
use actix_web::{web, App};
use async_trait::async_trait;
use log::info;
use serde::Serialize;
use sqlx::types::chrono::Utc;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use url::{ParseError, Url};

#[async_trait]
pub trait DataStore {
    async fn find_by_url(&self, key: &str) -> Result<String, String>;
    fn store(&self, long_url: &str, short_url: &str) -> Result<String, String>;
    fn is_alive(&self) -> bool;
}

pub trait CacheStore {
    fn is_alive(&mut self) -> bool;
    fn find_by_key(&mut self, key: &str) -> Result<String, String>;
    fn store(&self, key: &str, value: &str) -> Result<String, String>;
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

        Self { cache, db }
    }

    pub fn validate_url(&self, url: &str) -> Result<Url, ParseError> {
        Url::parse(url)
    }

    pub fn exists(&self, _url: &str) -> Result<Option<UrlEntity>, sqlx::Error> {
        // let mut db = self.db.lock().unwrap();

        // db.find_by_url(key)
        Ok(Some(UrlEntity {
            id: 1,
            original_url: "hello".to_string(),
            short_url: "hello".to_string(),
        }))
    }

    pub fn record_new_url(&self, long_url: &str) -> Result<Option<UrlEntity>, sqlx::Error> {
        let mut db = self.db.lock().unwrap();

        let short_url = "some short url";

        db.store(long_url, short_url);

        let mut cache = self.cache.lock().unwrap();
        cache.store(short_url, long_url);

        let url_entity = UrlEntity {
            id: 1,
            original_url: "hello".to_string(),
            short_url: "hello".to_string(),
        };

        Ok(Some(url_entity))
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

        // let _ = db.find_by_key("b");

        health_status
    }
}

impl HttpServer {
    pub async fn listen_and_serve(
        config: &Settings,
        svc: Arc<Mutex<UrlShortenerService>>,
    ) -> Result<(), std::io::Error> {
        let shared_app = web::Data::new(svc.clone());

        let address = format!("{}:{}", &config.base_url, &config.port);

        info!("serving on {}", address);

        let listener = TcpListener::bind(&address)?;

        let server = ActixHttpServer::new(move || {
            App::new()
                .route("/health-check", web::get().to(health_check))
                .route("/shorten", web::post().to(shorten_url))
                .default_service(web::route().to(not_found))
                .app_data(shared_app.clone())
        })
        .listen(listener)?
        .run();

        server.await
    }
}
