use urlshortner::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world updated!");

    let config = get_configuration().expect("failed to read configuration");

    println!("{:?} - {:?}",config.base_url, config.port);
    Ok(())
}
