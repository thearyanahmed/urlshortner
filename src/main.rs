use std::net::TcpListener;
// use sqlx::PgPool;
use actix_web::{HttpServer, web, App};
use actix_web::dev::Server;
use urlshortner::url_shortener::configuration::get_configuration;
use urlshortner::url_shortener::{Runner};
use urlshortner::url_shortener::redis::RedisStore;

extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let config = get_configuration().expect("failed to read configuration");
    //
    let app = Runner::build_http_server(&config).await?;
    //
    app.listen_and_serve().await?;

    Ok(())
}
