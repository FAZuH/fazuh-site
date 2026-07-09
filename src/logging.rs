use std::path::PathBuf;

use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::Config;

pub fn init_logging() {
    let config = Config::get();

    let log_dir = PathBuf::from(&config.default_log_dir);
    let file_appender = tracing_appender::rolling::daily(log_dir, &config.log_filename);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_target(false)
        .with_ansi(false)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level)),
        );

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level)),
        );

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stderr_layer)
        .init();
}
