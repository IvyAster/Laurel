use tracing_subscriber::fmt;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::types::LogConfig;

pub mod types;


fn get_level(level: &String) -> Level {
    level.parse::<Level>().unwrap_or(Level::INFO)
}

fn get_rotation(rotation: &Option<String>) -> Rotation {
    match rotation {
        Some(ro) => match ro.as_str() {
            "daily" => Rotation::DAILY,
            "never" => Rotation::NEVER,
            "minute" => Rotation::MINUTELY,
            "hour" => Rotation::HOURLY,
            _ => Rotation::NEVER,
        },
        _ => Rotation::NEVER,
    }
}

macro_rules! json_layer {
    ($writer:expr) => {
        fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_writer($writer)
    };
}

pub fn setup(log_config: &LogConfig) -> anyhow::Result<(),  anyhow::Error> {
    let log_level = get_level(&log_config.level);
    let env_filter = EnvFilter::from_default_env().add_directive(log_level.into());

    match log_config.appender.as_deref() {
        Some("file") => {
            let file_appender = RollingFileAppender::new(
                get_rotation(&log_config.rotation),
                &log_config.location,
                &log_config.prefix,
            );

            tracing_subscriber::registry()
                .with(env_filter)
                .with(json_layer!(file_appender))
                .init();
        }
        Some("all") => {
            let file_appender = RollingFileAppender::new(
                get_rotation(&log_config.rotation),
                &log_config.location,
                &log_config.prefix,
            );

            tracing_subscriber::registry()
                .with(env_filter)
                .with(json_layer!(file_appender))
                .with(json_layer!(std::io::stdout))
                .init();
        }
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(json_layer!(std::io::stdout))
                .init();
        }
    }

    Ok(())
}