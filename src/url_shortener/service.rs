use crate::url_shortener::configuration::Settings;
use crate::url_shortener::routes::{health_check, not_found, shorten_url};
use crate::url_shortener::Url;
use actix_web::HttpServer as ActixHttpServer;
use actix_web::{web, App};
use async_trait::async_trait;
use log::{error, info};
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
    async fn find_by_url(&mut self, key: &str) -> Result<Vec<Url>, Error>;
    async fn store(&self, original_url: &str, key: &str) -> Result<Url, Error>;
    fn is_alive(&self) -> bool;
}

#[async_trait]
pub trait CacheStore {
    fn is_alive(&mut self) -> bool;
    fn find_by_key(&mut self, key: &str) -> Result<String, String>;
    async fn store(&mut self, key: &str, value: &str) -> Result<bool, String>;
}

pub struct HttpServer {}

pub struct UrlShortenerService {
    cache: Arc<Mutex<dyn CacheStore + Send + Sync>>,
    db: Arc<Mutex<dyn DataStore + Send + Sync>>,
    // url_size: usize, // @todo
    base_url: String,
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
        url_prefix: &str,
    ) -> Self {
        let cache: Arc<Mutex<dyn CacheStore + Send + Sync>> = Arc::new(Mutex::new(cache_store));
        let db: Arc<Mutex<dyn DataStore + Send + Sync>> = Arc::new(Mutex::new(db_store));

        Self { cache, db, base_url: url_prefix.to_owned() }
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.to_string()
    }

    pub fn validate_url(&self, url: &str) -> Result<UrlParser, ParseError> {
        UrlParser::parse(url)
    }

    pub async fn find_by_url(&self, url: &str) -> Result<Option<Url>, Error> {
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
        let base64_encoded = general_purpose::URL_SAFE.encode(combined);

        // Truncate to 7 characters
        let truncated = &base64_encoded[..len];

        truncated.to_string()
    }

    pub async fn record_new_url(&self, full_url: &str) -> Result<Url, Error> {
        let key = &self.generate_unique_key(full_url, 10);

        let db = self.db.lock().unwrap();

        let result : Url = db.store(full_url,key).await?;

        let mut cache = self.cache.lock().unwrap();

        match cache.store(key,full_url).await  {
            Ok(_) => {},
            Err(err) => error!("{}",err)
        }

        Ok(result)
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
