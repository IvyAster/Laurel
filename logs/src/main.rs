#[deny(clippy::unwrap_used)]
mod model;
mod repository;
mod route;
mod schema;
mod service;
mod setup;

use actix_web::{App, HttpResponse, HttpServer, web};
use clap::Parser;
use laurel_actix::config::{AppArgs, ServerConfig, load_config};
use laurel_actix::{ActixApp};
use laurel_logging::types::LogConfig;
use laurel_pg::DbPool;
use laurel_pg::types::DbConfig;
use serde::Deserialize;

// use mimalloc::MiMalloc;
//
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Deserialize, Clone)]
pub struct UaConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogsAppConfig {
    pub server_config: ServerConfig,
    pub db_config: DbConfig,
    pub log_config: LogConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SystemApiConfig {
    pub uc_service: String,
    pub id_service: String,
}

#[actix_web::main]
#[allow(deprecated, unused_mut)]
async fn main() -> std::io::Result<()> {
    let (app_config, pool) = setup().await;
    let (host, port) = (
        (&app_config).server_config.host.clone(),
        (&app_config).server_config.port,
    );
    HttpServer::new(move || {
        let mut app = ActixApp!().configure(route::config).configure(|cfg| {
            setup::components::load_components(cfg, (&app_config).clone(), pool.clone())
        });
        app
    })
    //.workers(16)
    .bind((host, port))?
    .run()
    .await
}

#[allow(deprecated)]
async fn setup() -> (LogsAppConfig, DbPool) {
    let args = AppArgs::parse();
    let app_config: LogsAppConfig =
        load_config::<LogsAppConfig>(None, args.config).expect("Failed to load config");
    laurel_logging::setup(&app_config.log_config).expect("Failed to initialize logging");
    let pool = laurel_pg::setup(&app_config.db_config)
        .await
        .expect("Failed to setup db");
    (app_config, pool)
}
