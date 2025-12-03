pub mod handler;
pub mod error;
pub mod types;
pub mod config;
pub mod utils;



#[allow(non_snake_case)]
#[macro_export]
macro_rules! ActixApp{
    () => {
        App::new()
        .wrap_fn(laurel_actix::error_handler!(req, srv))
        .app_data(web::PathConfig::default().error_handler(laurel_actix::default_error_handler!(err, _req)))
        .app_data(web::JsonConfig::default().error_handler(laurel_actix::default_error_handler!(err, _req)))
        .app_data(web::QueryConfig::default().error_handler(laurel_actix::default_error_handler!(err, _req)))
        .default_service(web::to(laurel_actix::handler::default_handler))
        .wrap(actix_web_httpauth::middleware::HttpAuthentication::with_fn(laurel_actix::handler::validator))
    }
}

