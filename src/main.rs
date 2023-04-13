use zero2prod::configuration::get_configuration;
use zero2prod::startup::build;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // setup logger
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // setup conection pool
    let configuration = get_configuration().expect("Failed to read configuration.");
    let server = build(configuration).await?;

    server.await?;
    Ok(())
}
