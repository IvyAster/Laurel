use actix_web::web;

pub mod dict_route;
pub mod fe_micro_service_route;
pub mod menu_route;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(fe_micro_service_route::config);
    cfg.configure(menu_route::config);
    cfg.configure(dict_route::config);
}
