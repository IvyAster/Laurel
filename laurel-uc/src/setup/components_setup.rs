use std::sync::Arc;
use actix_web::web;
use laurel_actix::handler::TokenHandler;
use laurel_pg::DbPool;
use laurel_redis::Redis;
use crate::repository::account_repository::AccountRepository;
use crate::repository::passport_repository::PassportRepository;
use crate::repository::profile_repository::ProfileRepository;
use crate::service::account_service::AccountService;
use crate::service::profile_service::ProfileService;
use crate::service::token_service::TokenService;


pub fn load_components(cfg: &mut  web::ServiceConfig, pool: DbPool, redis_pool: Redis){ //fred::clients::Pool){
    let account_repository = Arc::new(AccountRepository::new(pool.clone()));
    let passport_repository = Arc::new(PassportRepository::new(pool.clone()));
    let profile_repository = Arc::new(ProfileRepository::new(pool.clone()));
    cfg.app_data(web::Data::new(AccountService::new(account_repository, passport_repository, redis_pool.clone())));
    cfg.app_data(web::Data::new(ProfileService::new(profile_repository)));
    let token_service: Box<dyn TokenHandler> = Box::new(TokenService::new(
        redis_pool.clone(),
        vec!["/api/uc/account/login".to_string()],
        vec!["/interface".to_string()]
    ));
    cfg.app_data(web::Data::new(token_service));
}
