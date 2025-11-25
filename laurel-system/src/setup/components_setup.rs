use crate::SystemApiConfig;
use crate::repository::fe_micro_service_repository::{FeMicroServiceRepository};
use crate::service::fe_micro_service_impl::{FeMicroServiceImpl};
use crate::service::token_service::TokenService;
use actix_web::web;
use laurel_id_api::id_api::IdApi;
use laurel_middleware::reqwest_middle::RequestLoggingMiddleware;
use laurel_uc_api::account_api::AccountApi;
use std::sync::Arc;
use laurel_actix::handler::TokenHandler;
use laurel_pg::DbPool;
use laurel_redis::Redis;
use crate::repository::dict_repository::DictRepository;
use crate::repository::menu_repository::MenuRepository;
use crate::service::dict_service::DictService;
use crate::service::menu_service::MenuService;

#[allow(unused)]
pub fn load_components(
    cfg: &mut web::ServiceConfig,
    service_config: &SystemApiConfig,
    pool: DbPool,
    redis: Redis
) {
    let request_client = reqwest::ClientBuilder::new()
        .pool_max_idle_per_host(5)
        .build()
        .expect("failed to build reqwest client");
    let client = reqwest_middleware::ClientBuilder::new(request_client)
        .with(RequestLoggingMiddleware)
        .build();

    let token_service: Box<dyn TokenHandler> = Box::new(TokenService::new(
        Arc::new(
            AccountApi::builder()
                .host(service_config.uc_service.clone())
                .client(Arc::new(client))
                .build(),
        ),
        vec!["/api/uc/account/login".to_string()],
        vec!["/interface".to_string(), "/swagger-ui".to_string(), "/api-docs".to_string()],
    ));
    cfg.app_data(web::Data::new(token_service));

    let fe_micro_service_repository = Arc::new(FeMicroServiceRepository::new(pool.clone()));
    #[allow(deprecated)]
    let id_api = Arc::new(
        IdApi::builder()
            .client(Arc::new(
                reqwest_middleware::ClientBuilder::new(
                    reqwest::ClientBuilder::new().build().unwrap(),
                )
                .with(RequestLoggingMiddleware)
                .build(),
            ))
            .host(service_config.id_service.clone())
            .build(),
    );
    let micro_app_service = FeMicroServiceImpl::builder()
        .micro_app_repository(fe_micro_service_repository)
        .id_api(Arc::clone(&id_api))
        .build();
    cfg.app_data(web::Data::new(micro_app_service));

    let menu_repository = Arc::new(
        MenuRepository::new(pool.clone())
    );
    let menu_service = MenuService::builder()
        .id_api(Arc::clone(&id_api))
        .menu_repository(menu_repository)
        .build();
    cfg.app_data(web::Data::new(menu_service));

    let dict_repository = Arc::new(
        DictRepository::new(pool.clone())
    );
    let dict_service = DictService::new(dict_repository);
    cfg.app_data(web::Data::new(dict_service));
}
