use std::sync::Arc;
use actix_web::web;
use laurel_actix::handler::TokenHandler;
use laurel_redis::Redis;
use crate::{service, AppConfig};

pub mod ip_searcher;

pub fn setup(
    cfg: &mut web::ServiceConfig,
             config: &AppConfig,
    redis: Redis
){

    let (v4, v6) = ip_searcher::setup(&config.ip_config);
    let ip_service = service::ip::IpSearchService::new(Arc::new(v4), Arc::new(v6), redis.clone());
    cfg.app_data(web::Data::new(ip_service));

    let token_service: Arc<dyn TokenHandler> = Arc::new(
        service::token::TokenService::new(
            match &config.server_config.excludes {
                Some(p) => p.clone(),
                None => vec![]
            },
            match &config.server_config.exclude_starts {
                Some(p) => p.clone(),
                None => vec![]
            }
        )
    );
    cfg.app_data(web::Data::new(token_service));

    let f = std::fs::File::open(&config.ua_config.path)
        .expect(
            format!(
                "load ua regexes config file: {} error", (&config).ua_config.path.as_str()
            ).as_str()
        );
    let regexes: ua_parser::Regexes = serde_yaml_ng::from_reader(f).expect("config ua regexes error");
    let extractor = ua_parser::Extractor::try_from(regexes).expect("load ua regexes error");
    cfg.app_data(web::Data::new(extractor));
}


#[test]
pub fn test_ua_parser(){
    let f = std::fs::File::open("./config/regexes.yaml")
        .expect(
            format!(
                "load ua regexes config file: {} error", "./config/regexes.yaml"
            ).as_str()
        );
    let regexes: ua_parser::Regexes = serde_yaml_ng::from_reader(f).expect("config ua regexes error");
    let extractor = ua_parser::Extractor::try_from(regexes).expect("load ua regexes error");
    let ua = extractor.extract("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36");
    println!("{:?}", ua);

    let ua = extractor.extract("Mozilla/5.0 (Linux; Android 13; Redmi Note 12) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36 UCBrowser/16.0.0.0");
    println!("{:?}", ua);

}