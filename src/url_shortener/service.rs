use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use sqlx::types::chrono::{DateTime, Utc};
use actix_web::{HttpServer, web, App};
use actix_web::dev::Server;
use actix_web::web::Data;
use crate::url_shortener::configuration::Settings;
use crate::url_shortener::redis::RedisStore;
use crate::url_shortener::routes::{health_check, not_found, testing_redis};

// @todo add boot time
pub struct UrlShortenerService {
    server: Server,
    boot_time: DateTime<Utc>,
    // datastore service # persistent
    // cache service # caching
    // db: Box<dyn DataStore>,
    // cache: dyn CacheStore,
}


// Define your shared data
struct AppStore {
    cache: Mutex<Box<dyn CacheStore>>,
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

impl UrlShortenerService {
    pub async fn build(config: &Settings) -> Result<Self, std::io::Error>
    {
        let address = format!("{}:{}", &config.base_url, &config.port);

        let listener = TcpListener::bind(&address)?;

        let boot_time = Utc::now();

        let server = run(listener)?;

        Ok(Self{
            server,
            boot_time,
        })
    }

    pub async fn listen_and_serve(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let cache = RedisStore::new();

    let arc_service: Arc<Mutex<dyn CacheStore + Send + Sync>> = Arc::new(Mutex::new(cache));

    let data_service: web::Data<Mutex<dyn CacheStore + Send + Sync>> = web::Data::from(arc_service);


    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/shorten", web::post().to(health_check))
            .route("/visit", web::get().to(testing_redis))
            .default_service(web::route().to(not_found))
            .app_data(data_service.clone())
            // .app_data(db.clone())
            // .app_data(shared_redis.clone())

    })
        .listen(listener)?
        .run();

    Ok(server)
}