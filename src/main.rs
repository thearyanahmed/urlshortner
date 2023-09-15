use std::net::TcpListener;
// use sqlx::PgPool;
use urlshortner::configuration::{get_configuration, Settings};
use urlshortner::routes::health_check;
use actix_web::{HttpServer, web, App};
use actix_web::dev::Server;

extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let config = get_configuration().expect("failed to read configuration");

    let app = UrlShortenerService::build(&config).await?;

    app.listen_and_serve().await?;

    Ok(())
}

pub struct UrlShortenerService {
    server: Server
}

impl UrlShortenerService {
    // @todo async?
    pub async fn build(config: &Settings) -> Result<Self, std::io::Error>  {
        let address = format!("{}:{}",&config.base_url,&config.port);

        let listener = TcpListener::bind(&address)?;

        let server = run(listener)?;

        Ok(Self {server})
    }

    pub async fn listen_and_serve(self) -> Result<(),std::io::Error> {
        self.server.await
    }
}


fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // let db_pool = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check",web::get().to(health_check))
            // .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}