use actix_web::web;

pub mod dict;
pub mod fe_micro_service;
pub mod menu;
mod account;
mod account_api;
mod profile;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(fe_micro_service::config)
        .configure(menu::config)
        .configure(dict::config)
        .configure(account::config)
        .configure(account_api::config)
        .configure(profile::config);
}
