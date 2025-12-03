use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(login_log_api::config)
        .configure(login_log::config);
}

pub mod login_log_api {
    use crate::service::login_log;
    use actix_web::{post, web};
    use laurel_actix::Data;
    use laurel_actix::types::{Autowired, RequestBody, route};
    use laurel_logs_api::logs::LoginLogCreateReqBo;

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/interface/logs/login").service(save_log));
    }

    #[post("/create")]
    pub async fn save_log(
        service: Autowired<login_log::Service>,
        body: RequestBody<LoginLogCreateReqBo>,
    ) -> route::Result<i64> {
        Data!(service.create(&body).await?)
    }
}

pub mod login_log {
    use crate::model::login_log::{LoginLogQueryReq, LoginLogVo};
    use crate::service::login_log;
    use actix_web::{post, web};
    use laurel_actix::Data;
    use laurel_actix::types::{Autowired, RequestBody, route};
    use laurel_common::types::Pagination;

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/api/logs/login").service(page_logs));
    }

    #[post("/pages")]
    pub async fn page_logs(
        service: Autowired<login_log::Service>,
        body: RequestBody<LoginLogQueryReq>,
    ) -> route::Result<Pagination<LoginLogVo>> {
        let page = match &body.page {
            Some(p) => (p.page, p.size),
            _ => (1, 15),
        };
        Data!(service.page(&body, page).await?.to_with_index())
    }
}
