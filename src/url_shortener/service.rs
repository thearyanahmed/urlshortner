use crate::url_shortener::configuration::Settings;
use crate::url_shortener::routes::{health_check, not_found, shorten_url};
use crate::url_shortener::Url;
use actix_web::HttpServer as ActixHttpServer;
use actix_web::{web, App};
use async_trait::async_trait;
use log::info;
use ring::digest::{Context, SHA256};
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use serde::Serialize;
use sqlx::types::chrono::Utc;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use url::{ParseError, Url as UrlParser};

extern crate base64;
extern crate ring;

use base64::{engine::general_purpose, Engine as _};
use sqlx::Error;

#[async_trait]
pub trait DataStore {
    async fn find_by_url(&mut self, key: &str) -> Result<Vec<Url>, sqlx::Error>;
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
    // url_size: usize, // @todo 
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

    pub fn validate_url(&self, url: &str) -> Result<UrlParser, ParseError> {
        UrlParser::parse(url)
    }

    pub async fn find_by_url(&self, url: &str) -> Result<Option<Url>, sqlx::Error> {
        let mut db = self.db.lock().unwrap();

        let result: Result<Vec<Url>, Error> = db.find_by_url(url).await;

        match result {
            Ok(records) => {
                if !records.is_empty() {
                    Ok(Some(records[0].clone()))
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(err)
        }
    }

    fn generate_unique_key(&self, input: &str, len: usize) -> String {
        // Hash the input string using SHA-256
        let mut context = Context::new(&SHA256);
        context.update(input.as_bytes());
        let digest = context.finish();

        // Generate a random salt for additional uniqueness
        let mut salt = [0u8; 8];
        let rng = SystemRandom::new();
        rng.fill(&mut salt)
            .expect("Failed to generate random bytes");

        // Combine the hash and salt to create a unique ID
        let combined = [&salt, digest.as_ref()].concat();

        // Encode the combined result in base64 to make it URL-safe
        let base64_encoded = general_purpose::STANDARD_NO_PAD.encode(combined);

        // Truncate to 7 characters
        let truncated = &base64_encoded[..len];

        truncated.to_string()
    }

    pub fn record_new_url(&self, long_url: &str) -> Result<Option<Url>, sqlx::Error> {
        let short_url = &self.generate_unique_key(long_url, 10);

        info!("short url {}", short_url);

        let db = self.db.lock().unwrap();

        let _ = db.store(long_url, short_url);

        let cache = self.cache.lock().unwrap();
        let _ = cache.store(short_url, long_url);

        let url_entity = Url {
            id: 1,
            original_url: "hello".to_string(),
            short_url: short_url.to_string(),
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
