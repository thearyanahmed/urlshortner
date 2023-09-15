use std::sync::{Arc, Mutex};
use urlshortner::url_shortener::configuration::get_configuration;
use urlshortner::url_shortener::{HttpServer, UrlShortenerService};
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

    HttpServer::build_http_server(&config, svc).await?;

    Ok(())
}
