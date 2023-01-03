pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

use startup::Application;
use std::path::PathBuf;
use structopt::StructOpt;
use telemetry::{get_subscriber, init_subscriber};

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

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
