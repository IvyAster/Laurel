#[deny(clippy::unwrap_used)]
mod docs;
mod model;
mod repository;
mod routes;
pub mod schema;
mod service;
mod setup;

use actix_web::dev::Service;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use clap::Parser;
use laurel_actix::config::{AppArgs, ServerConfig, load_config};
use laurel_actix::handler::default_handler;
use laurel_actix::handler::validator;
use laurel_actix::types::LR;
use laurel_actix::{ActixApp, default_error_handler, error_handler};
use laurel_logging::types::LogConfig;
use laurel_pg::DbPool;
use laurel_pg::types::DbConfig;
use laurel_redis::{Redis, RedisConfig};
use serde::Deserialize;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::dict_doc::DictApiDoc;

#[derive(Debug, Deserialize)]
pub struct SystemAppConfig {
    pub server_config: ServerConfig,
    pub db_config: DbConfig,
    pub log_config: LogConfig,
    pub redis_config: RedisConfig,
    pub api_config: SystemApiConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SystemApiConfig {
    pub uc_service: String,
    pub id_service: String,
}

#[actix_web::main]
#[allow(deprecated, unused_mut)]
async fn main() -> std::io::Result<()> {
    let (app_config, pool, redis) = setup().await;
    HttpServer::new(move || {
        let mut app = ActixApp!()
            .configure(routes::config)
            .configure(|cfg| {
                setup::components_setup::load_components(
                    cfg,
                    &app_config.api_config,
                    pool.clone(),
                    redis.clone(),
                )
            });

        #[cfg(feature = "use_api_docs")]{
            app = app.service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", DictApiDoc::openapi()),
            )
        }

        app
    })
    //.workers(16)
    .bind((app_config.server_config.host, app_config.server_config.port))?
    .run()
    .await
}

#[allow(deprecated)]
async fn setup() -> (SystemAppConfig, DbPool, Redis) {
    let args = AppArgs::parse();
    let app_config: SystemAppConfig =
        load_config::<SystemAppConfig>(None, args.config).expect("Failed to load config"); //.try_deserialize().expect("system app config load failed");
    laurel_logging::setup(&app_config.log_config).expect("Failed to initialize logging");
    let redis = Redis::new(&app_config.redis_config)
        .await
        .expect("Failed to create redis pool");
    let pool = laurel_pg::setup(&app_config.db_config)
        .await
        .expect("Failed to setup db");
    (app_config, pool, redis)
}
