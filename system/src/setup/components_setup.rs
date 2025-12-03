use crate::{repository, SystemAppConfig};
use crate::repository::dict::DictRepository;
use crate::repository::fe_micro_service::FeMicroServiceRepository;
use crate::repository::menu::MenuRepository;
use crate::service::dict::DictService;
use crate::service::fe_micro_service::FeMicroServiceImpl;
use crate::service::menu::MenuService;
use crate::service::token::TokenService;
use actix_web::web;
use laurel_actix::handler::TokenHandler;
use laurel_id_api::id::IdApi;
use laurel_middleware::reqwest_middle::RequestLoggingMiddleware;
use laurel_pg::DbPool;
use laurel_redis::Redis;
use std::sync::Arc;
use std::time::Duration;
use laurel_logs_api::logs::LogApi;
use crate::repository::account::AccountRepository;
use crate::repository::passport::PassportRepository;
use crate::repository::profile::ProfileRepository;
use crate::service::account::AccountService;
use crate::service::profile::ProfileService;

#[allow(unused)]
pub fn load_components(
    cfg: &mut web::ServiceConfig,
    service_config: SystemAppConfig,
    pool: DbPool,
    redis: Redis,
) {
    let request_client = reqwest::ClientBuilder::new()
        .pool_max_idle_per_host(20)
        .pool_max_idle_per_host(20)        // 每个主机最多保留 20 个空闲连接
        .pool_idle_timeout(Some(Duration::from_secs(30))) // 30秒未使用则关闭

        // 关键参数2：超时设置
        .timeout(Duration::from_secs(20))

        // 关键参数3：HTTP/2 配置
        //.http2_initial_stream_window_size(65535)
        //.http2_initial_connection_window_size(1048576)

        // 关键参数4：禁用不必要的功能
        //.gzip(false)  // 如果不需要压缩，禁用可节省内存
        //.brotli(false)
        .build()
        .expect("failed to build reqwest client");
    let client = Arc::new(reqwest_middleware::ClientBuilder::new(request_client)
        .with(RequestLoggingMiddleware)
        .build()
    );

    let token_service: Arc<TokenService> = Arc::new(
        TokenService::new(
            redis.clone(),
            vec!["/api/system/account/login".to_string()],
            vec![
                "/interface".to_string(),
                "/swagger-ui".to_string(),
                "/api-docs".to_string(),
            ],
            service_config.uc_config.secret.clone(),
        )
    );
    let dyn_token_service: Arc<dyn TokenHandler> = Arc::clone(&token_service) as Arc<dyn TokenHandler>;
    cfg.app_data(web::Data::new(dyn_token_service));

    let fe_micro_service_repository = Arc::new(FeMicroServiceRepository::new(pool.clone()));
    #[allow(deprecated)]
    let id_api = IdApi::build(
        Arc::clone(&client),
        service_config.api_config.id_service.clone(),
        None
    ) ;
    let ip_api =
        laurel_tool_api::ip::IpApi::build(
            Arc::clone(&client),
            service_config.api_config.tool_service.clone(),
            None
    );

    let ua_api = laurel_tool_api::ua::UaApi::build(
        Arc::clone(&client),
        service_config.api_config.tool_service.clone(),
        None
    );
    let micro_app_service = FeMicroServiceImpl::builder()
        .micro_app_repository(fe_micro_service_repository)
        .id_api(id_api.clone())
        .build();
    cfg.app_data(web::Data::new(micro_app_service));

    let menu_repository = Arc::new(MenuRepository::new(pool.clone()));
    let menu_service = MenuService::builder()
        .id_api(id_api.clone())
        .menu_repository(menu_repository)
        .build();
    cfg.app_data(web::Data::new(menu_service));

    let dict_repository = Arc::new(DictRepository::new(pool.clone()));
    let dict_service = DictService::new(dict_repository);
    cfg.app_data(web::Data::new(dict_service));


    let account_repository = Arc::new(AccountRepository::new(pool.clone()));
    let passport_repository = Arc::new(PassportRepository::new(pool.clone()));
    let profile_repository = Arc::new(ProfileRepository::new(pool.clone()));
    let ticket_repository = Arc::new(repository::ticket::Repository::new(pool.clone()));

    let log_api = Arc::new(
        LogApi::build(Arc::clone(&client), service_config.api_config.log_service.clone(), None)
    );
    cfg.app_data(web::Data::new(AccountService::new(
        account_repository,
        passport_repository,
        redis.clone(),
        Arc::clone(&log_api),
        Arc::clone(&ticket_repository),
        Arc::clone(&token_service),
        id_api.clone(),
        ip_api,
        ua_api,
    )));
    cfg.app_data(web::Data::new(ProfileService::new(profile_repository)));
}

