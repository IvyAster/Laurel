use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub location: String,
    pub prefix: String,
    pub appender: Option<String>,
    pub rotation: Option<String>,
}