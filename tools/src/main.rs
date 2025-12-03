mod setup;
mod service;
mod model;
mod route;

use clap::Parser;

use serde::Deserialize;
use laurel_actix::config::{load_config, AppArgs, ServerConfig};
use laurel_logging::types::LogConfig;
use laurel_redis::{Redis, RedisConfig};
use laurel_actix::{ActixApp};
use actix_web::{App, HttpResponse, HttpServer, web};

// use mimalloc::MiMalloc;
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server_config: ServerConfig,
    pub log_config: LogConfig,
    pub redis_config: RedisConfig,
    pub ip_config: IpConfig,
    pub ua_config: UaConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IpConfig{
    pub ip_v4: String,
    pub ip_v6: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UaConfig {
    pub path: String,
}

#[actix_web::main]
#[allow(deprecated, unused_mut)]
async fn main() -> std::io::Result<()> {
    let (app_config, redis) = setup().await;
    let (host, port) = ((&app_config.server_config).host.clone(), (&app_config.server_config).port);
    HttpServer::new(move || {
        ActixApp!().configure(route::config).configure(|cfg| {
            setup::setup(
                cfg,
                &app_config,
                redis.clone(),
            )
        })
    })
        .bind((host, port))?
        .run()
        .await
}

#[allow(deprecated)]
async fn setup() -> (AppConfig, Redis) {
    let args = AppArgs::parse();
    let app_config: AppConfig =
        load_config::<AppConfig>(None, args.config).expect("Failed to load config");
    laurel_logging::setup(&app_config.log_config).expect("Failed to initialize logging");
    let redis = Redis::new(&app_config.redis_config)
        .await
        .expect("Failed to create redis pool");
    (app_config, redis)
}
