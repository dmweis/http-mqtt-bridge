use crate::configuration::MqttSettings;
use anyhow::{Context, Result};
use rumqttc::{self, AsyncClient, MqttOptions, Transport};
use rustls::ClientConfig;
use secrecy::ExposeSecret;

pub fn create_mqtt_client(config: &MqttSettings) -> Result<AsyncClient> {
    let mut mqtt_options = MqttOptions::new(&config.client_id, &config.host, config.port);
    mqtt_options.set_clean_session(true);
    mqtt_options.set_keep_alive(std::time::Duration::from_secs(5));
    mqtt_options.set_credentials(&config.username, config.password.expose_secret());

    let mut root_cert_store = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().context("could not load platform certs")? {
        root_cert_store.add(&rustls::Certificate(cert.0))?;
    }

    let client_config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    mqtt_options.set_transport(Transport::tls_with_config(client_config.into()));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(_) => (),

                Err(e) => {
                    tracing::error!("Error = {:?}", e);
                }
            }
        }
    });
    Ok(client)
}
