mod account_route;
mod profile_route;
mod account_route_api;

use actix_web::{web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(account_route::config)
        .configure(account_route_api::config)
        .configure(profile_route::config);
}

