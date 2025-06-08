#![doc = include_str!("../README.md")]

use bevy::log::tracing_subscriber::Layer;
use bevy::log::tracing_subscriber::fmt::layer;
use bevy::log::{BoxedLayer, DEFAULT_FILTER, Level, LogPlugin};
use bevy::prelude::App;
use config::LogConfiguration;
use std::str::FromStr;
use tracing_appender::rolling::daily;

/// Builds the [`LogPlugin`] for the application.
#[must_use]
pub fn log_plugin(config: &LogConfiguration) -> LogPlugin {
    LogPlugin {
        filter: format!("{DEFAULT_FILTER},{}", config.filter),
        level: Level::from_str(config.level.as_str()).unwrap_or(Level::INFO),
        custom_layer,
    }
}

/// This method builds a custom logging layer for `trace` and `log`.
///
/// This allows us to configure how the software should log, where and so forth.
#[allow(clippy::unnecessary_wraps)]
fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
    #[cfg(not(feature = "dev"))]
    return None;

    #[allow(
        unreachable_code,
        reason = "Only unreachable in dev, release mode will have this on"
    )]
    Some(Box::new(vec![
        layer()
            .with_file(false)
            .with_thread_names(true)
            .with_thread_ids(true)
            .with_level(true)
            .json()
            .with_writer(daily("logs", "dungeonrs"))
            .boxed(),
    ]))
}
