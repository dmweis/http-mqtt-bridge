pub mod configuration;
pub mod routes;
pub mod telemetry;

use actix_web::{App, HttpServer};
use routes::{health_check, send_message};
use std::path::PathBuf;
use structopt::StructOpt;
use telemetry::{get_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opts {
    #[structopt(long)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let opts = Opts::from_args();
    let configuration =
        configuration::get_configuration(opts.config).expect("Failed to read configuration.");

    let subscriber = get_subscriber(
        "http-mqtt-bridge".into(),
        "info".into(),
        std::io::stdout,
        &configuration.logging_settings,
    );
    init_subscriber(subscriber);

    let address = format!(
        "{}:{}",
        configuration.server.host, configuration.server.port
    );
    tracing::info!("Running server on {}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(send_message)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
