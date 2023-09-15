use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use sqlx::Postgres;
use actix_web::{HttpServer, web, App};
use actix_web::dev::Server;
use urlshortner::url_shortener::configuration::get_configuration;
use urlshortner::url_shortener::{HttpRouter, UrlShortenerService};
use urlshortner::url_shortener::postgres::PostgresStore;
use urlshortner::url_shortener::redis::RedisStore;

extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let config = get_configuration().expect("failed to read configuration");

    let cache = RedisStore::new();
    let db = PostgresStore::new();

    let svc = UrlShortenerService::new(cache, db);
    let svc: Arc<Mutex<UrlShortenerService>> = Arc::new(Mutex::new(svc));

    HttpRouter::build_http_server(&config, svc).await?;

    Ok(())
}
