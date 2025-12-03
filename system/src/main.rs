#[deny(clippy::unwrap_used)]
mod docs;
mod model;
mod repository;
mod routes;
mod schema;
mod service;
mod setup;
mod utils;

use actix_web::{App, HttpResponse, HttpServer, web};
use clap::Parser;
use laurel_actix::ActixApp;
use laurel_actix::config::{AppArgs, ServerConfig, load_config};
use laurel_logging::types::LogConfig;
use laurel_pg::DbPool;
use laurel_pg::types::DbConfig;
use laurel_redis::{Redis, RedisConfig};
use serde::Deserialize;

// use mimalloc::MiMalloc;
//
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

// #[derive(Debug, Deserialize, Clone)]
// pub struct UaConfig {
//     pub path: String,
// }

#[derive(Debug, Deserialize, Clone)]
pub struct UcConfig{
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SystemAppConfig {
    pub server_config: ServerConfig,
    pub db_config: DbConfig,
    pub log_config: LogConfig,
    pub redis_config: RedisConfig,
    pub api_config: SystemApiConfig,
    pub uc_config: UcConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SystemApiConfig {
    pub id_service: String,
    pub log_service: String,
    pub tool_service: String,
}

#[actix_web::main]
#[allow(deprecated, unused_mut)]
async fn main() -> std::io::Result<()> {
    let (app_config, pool, redis) = setup().await;
    let (host, port) = (
        (&app_config).server_config.host.clone(),
        (&app_config).server_config.port,
    );
    HttpServer::new(move || {
        let mut app = ActixApp!().configure(routes::config).configure(|cfg| {
            setup::components_setup::load_components(
                cfg,
                //&app_config.api_config,
                (&app_config).clone(),
                pool.clone(),
                redis.clone(),
            )
        });

        #[cfg(feature = "use_api_docs")]
        {
            use crate::docs::system::SystemApiDoc;
            use actix_web::web;
            use utoipa::OpenApi;

            app = app.configure(|cfg: &mut web::ServiceConfig| {
                cfg.service(
                    web::scope("/api-docs/system").route(
                        "/docs",
                        web::get()
                            .to(|| async { HttpResponse::Ok().json(SystemApiDoc::openapi()) }),
                    ), //.service(web::resource("dict.json").to(DictApiDoc::openapi))
                );
            });
        }

        app
    })
    .bind((host, port))?
    .run()
    .await
}

#[allow(deprecated)]
async fn setup() -> (SystemAppConfig, DbPool, Redis) {
    let args = AppArgs::parse();
    let app_config: SystemAppConfig =
        load_config::<SystemAppConfig>(None, args.config).expect("Failed to load config");
    laurel_logging::setup(&app_config.log_config).expect("Failed to initialize logging");
    let redis = Redis::new(&app_config.redis_config)
        .await
        .expect("Failed to create redis pool");
    let pool = laurel_pg::setup(&app_config.db_config)
        .await
        .expect("Failed to setup db");
    (app_config, pool, redis)
}
