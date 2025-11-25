#[deny(clippy::unwrap_used)]
use std::sync::Arc;
pub mod model;
pub mod repository;
mod routes;
mod schema;
pub mod service;
pub mod setup;
mod utils;

use actix_web::HttpResponse;
use actix_web::dev::Service;
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use laurel_actix::ActixApp;
use laurel_actix::config::{ServerConfig, load_config};
use laurel_actix::handler::default_handler;
use laurel_actix::handler::validator;
use laurel_actix::types::LR;
use laurel_actix::{default_error_handler, error_handler};
use laurel_id_api::id_api::IdApi;
use laurel_logging::types::LogConfig;
use laurel_middleware::reqwest_middle::RequestLoggingMiddleware;
use laurel_pg::DbPool;
use laurel_pg::types::DbConfig;
use laurel_redis::{Redis, RedisConfig};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub id_service: String,
}

#[derive(Debug, Deserialize)]
pub struct UcAppConfig {
    pub server_config: ServerConfig,
    pub db_config: DbConfig,
    pub log_config: LogConfig,
    pub redis_config: RedisConfig,
    pub api_config: ApiConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (app_config, pool, redis) = setup().await;
    #[allow(deprecated)]
    let id_api = IdApi::builder()
        .client(Arc::new(
            reqwest_middleware::ClientBuilder::new(reqwest::ClientBuilder::new().build().unwrap())
                .with(RequestLoggingMiddleware)
                .build(),
        ))
        .host(app_config.api_config.id_service.clone())
        .build();
    info!("id = {}", id_api.id().await.unwrap());
    HttpServer::new(move || {
        ActixApp!().configure(routes::config).configure(|cfg| {
            setup::components_setup::load_components(cfg, pool.clone(), redis.clone())
        })
        //app
    })
    //.workers(16)
    .bind((app_config.server_config.host, app_config.server_config.port))?
    .run()
    .await
}

async fn setup() -> (UcAppConfig, DbPool, Redis) {
    let app_config: UcAppConfig =
        load_config::<UcAppConfig>(None, None).expect("failed to load config"); //.try_deserialize().unwrap();//.expect("transfer config failed");
    //AppConfig::load_app_config(None, None).expect("Failed to load config");
    laurel_logging::setup(&app_config.log_config).expect("Failed to setup logging");
    let redis = Redis::new(&app_config.redis_config) //laurel_setup::redis_setup::setup_redis(&app_config.redis_config)
        .await
        .expect("Failed to create redis pool");
    let pool = laurel_pg::setup(&app_config.db_config)
        .await
        .expect("Failed to create pool");
    (app_config, pool, redis)
}
