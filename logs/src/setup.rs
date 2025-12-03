pub mod components {
    use crate::{LogsAppConfig, repository, service};
    use actix_web::web;
    use laurel_pg::DbPool;
    use std::sync::Arc;
    use laurel_actix::handler::TokenHandler;

    #[allow(unused)]
    pub fn load_components(
        cfg: &mut web::ServiceConfig,
        service_config: LogsAppConfig,
        pool: DbPool,
    ) {
        let login_log_repository = Arc::new(repository::login_log::Repository::new(pool.clone()));

        let login_log_service = service::login_log::Service::new(login_log_repository);
        cfg.app_data(web::Data::new(login_log_service));

        #[allow(deprecated)]
        let token_service: Arc<dyn TokenHandler> = Arc::new(
            service::token::TokenService::new(
                match &service_config.server_config.excludes {
                    Some(p) => p.clone(),
                    None => vec![]
                },
                match &service_config.server_config.exclude_starts {
                    Some(p) => p.clone(),
                    None => vec![]
                }
            )
        );
        cfg.app_data(web::Data::new(token_service));
    }
}
