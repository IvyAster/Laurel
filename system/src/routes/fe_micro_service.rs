use crate::model::fe_micro_service::{
    FeMicroServiceCreateReq, FeMicroServiceQuery, FeMicroServiceVo, MicroServiceStatus,
    MicroServiceUpdateReq,
};
use crate::service::fe_micro_service::FeMicroServiceImpl;
use actix_web::{get, post, web};
use laurel_actix::Data;
use laurel_actix::types::{route, Autowired, RequestBody, RequestParam};
use laurel_common::types::{HappyEnum, Pagination, SelectOption};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/system/fe-micro-service")
            .service(list_micro_services)
            .service(list_micro_app_status_options)
            .service(create_micro_service)
            .service(update_micro_service)
            .service(list_used_services),
    );
}

#[post("/page/services")]
pub async fn list_micro_services(
    fe_micro_service: Autowired<FeMicroServiceImpl>,
    req: RequestBody<FeMicroServiceQuery>,
) -> route::Result<Pagination<FeMicroServiceVo>> {
    let (page, size) = match &req.page {
        Some(p) => (p.page, p.size),
        _ => (1, 15),
    };
    let pageable = fe_micro_service.page(&req, page, size).await?;
    Data!(
        pageable.to_with_index()
    )
}

#[get("/used/services")]
pub async fn list_used_services(
    micro_service: Autowired<FeMicroServiceImpl>,
    req: RequestParam<FeMicroServiceQuery>,
) -> route::Result<Vec<FeMicroServiceVo>> {
    let app_id = req.app_id.as_str();
    let services: Vec<FeMicroServiceVo> = micro_service
        .list_with_status(app_id, "open")
        .await?
        .into_iter()
        .map(|ms| FeMicroServiceVo::from(ms))
        .collect();
    Data!(
        services
    )
}

#[get("/status/options")]
pub async fn list_micro_app_status_options() -> route::Result<Vec<SelectOption<&'static str, &'static str>>>
{
    Data!(
        MicroServiceStatus::options()
    )
}

#[post("/create")]
pub async fn create_micro_service(
    micro_app_service: Autowired<FeMicroServiceImpl>,
    req: RequestBody<FeMicroServiceCreateReq>,
) -> route::Result<FeMicroServiceVo> {
    Data!(
        FeMicroServiceVo::from(micro_app_service.create(&req).await?)
    )
}

#[post("/update")]
pub async fn update_micro_service(
    micro_app_service: Autowired<FeMicroServiceImpl>,
    req: RequestBody<MicroServiceUpdateReq>,
) -> route::Result<FeMicroServiceVo> {
    Data!(
        FeMicroServiceVo::from(micro_app_service.update(&req).await?)
    )
}
