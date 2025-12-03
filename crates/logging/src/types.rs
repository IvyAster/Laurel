use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub level: String,
    pub location: String,
    pub prefix: String,
    pub appender: Option<String>,
    pub rotation: Option<String>,
}