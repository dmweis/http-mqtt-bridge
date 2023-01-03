use crate::configuration::Settings;
use crate::routes::{health_check, send_message};
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// New type because app_data is type based
pub struct ApplicationBaseUrl(pub String);

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(send_message)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        tracing::info!("Running server on {}", address);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
