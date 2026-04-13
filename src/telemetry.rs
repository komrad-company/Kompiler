use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::configuration::TelemetryConfiguration;

pub fn intialize(config: TelemetryConfiguration) {
    let filter = EnvFilter::new("INFO"); // TODO use LogLevel struct
    let registry = tracing_subscriber::registry().with(filter);

    let file_layer = config.file.map(|path| {
        let appender =
            tracing_appender::rolling::never(path.parent().unwrap(), path.file_name().unwrap());
        tracing_subscriber::fmt::layer()
            .json()
            .with_writer(appender)
    });
    registry.with(file_layer).init();
}
