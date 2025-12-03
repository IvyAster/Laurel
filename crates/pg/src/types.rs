use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig{
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub database: String,
    pub options: Option<String>,
    pub max_connections: u32,
    pub min_connections: u32,
}