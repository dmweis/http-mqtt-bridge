use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
    EnvFilter, Layer, Registry,
};

use crate::configuration::LoggingSettings;

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
    settings: &LoggingSettings,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let mut layers = vec![];

    if let Some(ref path) = settings.log_path {
        let file = std::fs::File::options()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        let bunyan_format = BunyanFormattingLayer::new(name.clone(), file);
        layers.push(bunyan_format.boxed());
    }

    if settings.log_bunyan {
        let bunyan_format = BunyanFormattingLayer::new(name, sink);
        layers.push(JsonStorageLayer.boxed());
        layers.push(bunyan_format.boxed());
    } else {
        let compact = fmt::layer().with_writer(sink);
        layers.push(compact.boxed());
    }

    Registry::default().with(env_filter).with(layers)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
