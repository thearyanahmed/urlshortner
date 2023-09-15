use urlshortner::configuration::get_configuration;

extern crate pretty_env_logger;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let _config = get_configuration().expect("failed to read configuration");

    Ok(())
}
