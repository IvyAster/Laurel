pub mod handler;
pub mod error;
pub mod types;
pub mod config;

use crate::types::LR;
use crate::handler::validator;
use crate::handler::default_handler;
use actix_web::{App, HttpServer, web, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;


#[allow(non_snake_case)]
#[macro_export]
macro_rules! ActixApp{
    () => {
        App::new()
        .wrap_fn(error_handler!(req, srv))
        .app_data(web::PathConfig::default().error_handler(default_error_handler!(err, _req)))
        .app_data(web::JsonConfig::default().error_handler(default_error_handler!(err, _req)))
        .app_data(web::QueryConfig::default().error_handler(default_error_handler!(err, _req)))
        .default_service(web::to(default_handler))
        .wrap(HttpAuthentication::with_fn(validator))
    }
}

