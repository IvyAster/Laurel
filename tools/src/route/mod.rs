use actix_web::web;

pub mod ip;
pub mod ua;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(ip::ip_api::config);
    cfg.configure(ua::ua_api::config);
}
