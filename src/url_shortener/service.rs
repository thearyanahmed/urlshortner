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
pub struct Runner {
    server: Server,
}

pub struct Test {
    cache: Arc<Mutex<dyn CacheStore + Send + Sync>>
}

impl Test {
    pub fn hello(&self) {
        let mut c = self.cache.lock().unwrap();

        let _ = c.find_by_key("a");

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

// url comes -> write to db -> also write to cache -> send the response
// key comes, checks in cache, not found? check in db, found -> write in cache
// add a feature flag that enables tracking? or expose a middleware

impl Runner {
    pub async fn build_http_server(config: &Settings) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", &config.base_url, &config.port);

        let listener = TcpListener::bind(&address)?;
        let cache= RedisStore::new();

        let cache: Arc<Mutex<dyn CacheStore + Send + Sync>> = Arc::new(Mutex::new(cache));
        // @note clone would be removed
        let shared_cache: web::Data<Mutex<dyn CacheStore + Send + Sync>> = web::Data::from(cache.clone());

        let test = Test {
            cache
        };

        let test : Arc<Mutex<Test>> = Arc::new(Mutex::new(test));

        let server = run(listener, shared_cache, test)?;

        Ok(Self { server })
    }

    pub async fn listen_and_serve(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(listener: TcpListener, cache: web::Data<Mutex<dyn CacheStore + Send + Sync>>, app: Arc<Mutex<Test>>) -> Result<Server, std::io::Error> {
    let shared_app = web::Data::new(app.clone());

    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/shorten", web::post().to(health_check))
            .route("/visit", web::get().to(testing_redis))
            .route("/do", web::get().to(test_run))
            .default_service(web::route().to(not_found))
            .app_data(shared_app.clone())
            .app_data(cache.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

pub async fn test_run(svc : web::Data<Arc<Mutex<Test>>>) -> HttpResponse {
    let x = svc.get_ref().lock().unwrap();
    x.hello();

    let data = HealthCheckResponse {
        status: "success pikachu".to_string(),
    };

    respond_with_json(&data, actix_web::http::StatusCode::OK)
}