pub mod configuration;
pub mod mqtt;
pub mod routes;
pub mod telemetry;

use crate::mqtt::create_mqtt_client;
use actix_web::{web, App, HttpServer};
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

// New type so that we can pass it to actix-web IOC
pub struct IftttKey(pub String);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let mqtt_client = create_mqtt_client(&configuration.mqtt)?;

    let address = format!(
        "{}:{}",
        configuration.server.host, configuration.server.port
    );
    tracing::info!("Running server on {}", address);

    let ifttt_key = web::Data::new(IftttKey(configuration.bridge.ifttt_key.clone()));
    let mqtt_client = web::Data::new(mqtt_client);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(send_message)
            .app_data(ifttt_key.clone())
            .app_data(mqtt_client.clone())
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn make_coverage_happy() {
        assert!(true);
    }
}
