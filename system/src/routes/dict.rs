use crate::model::dict::{
    DictCreateReq, DictDeleteReq, DictQueryReq, DictUpdateReq, DictValueCreateReq,
    DictValueDeleteReq, DictValueQueryReq, DictValueUpdateReq, DictValueVo, DictVo,
};
use crate::service::dict::DictService;
use actix_web::{get, post, web};
use laurel_actix::Data;
use laurel_actix::types::{Autowired, RequestBody, RequestParam, route};
use laurel_common::types::Pagination;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/system/dict")
            .service(page_dict)
            .service(create_dict)
            .service(update_dict)
            .service(delete_dict)
            .service(page_dict_values)
            .service(create_dict_value)
            .service(update_dict_value)
            .service(delete_dict_value),
    );
}

#[utoipa::path(
    get,
    tag = "dictionaries",
    context_path = "/api/system/dict",
    params(
        DictQueryReq
    ),
    responses(
        (status = 200, description = "获取字典分页列表", body = Pagination<DictVo>)
    )
)]
#[get("/pages")]
async fn page_dict(
    dict_service: Autowired<DictService>,
    query: RequestParam<DictQueryReq>,
) -> route::Result<Pagination<DictVo>> {
    let (page, size) = match &query.page {
        Some(p) => (p.page, p.size),
        _ => (1, 15),
    };

    Data!(dict_service.page_dict(&query, page, size).await?.to())
}

#[utoipa::path(
    post,
    tag = "dictionaries",
    context_path = "/api/system/dict",
    request_body = DictCreateReq,
    responses(
        (status = 200, description = "创建字典", body = DictVo)
    )
)]
#[post("/create")]
async fn create_dict(
    dict_service: Autowired<DictService>,
    body: RequestBody<DictCreateReq>,
) -> route::Result<DictVo> {
    Data!(
        DictVo::from(dict_service.create_dict(&body).await?)
    )
}

#[utoipa::path(
    post,
    context_path = "/api/system/dict",
    tag = "dictionaries",
    request_body = DictUpdateReq,
    responses(
        (status = 200, description = "更新字典", body = DictVo)
    )
)]
#[post("/update")]
async fn update_dict(
    dict_service: Autowired<DictService>,
    body: RequestBody<DictUpdateReq>,
) -> route::Result<DictVo> {
    Data!(
        dict_service
            .update_dict(&body)
            .await?
            .map(|d| d.into())
            .or(None)
    )
}

#[utoipa::path(
    post,
    tag = "dictionaries",
    context_path = "/api/system/dict",
    request_body = DictDeleteReq,
    responses(
        (status = 200, description = "删除字典", body = DictVo)
    )
)]
#[post("/delete")]
async fn delete_dict(
    dict_service: Autowired<DictService>,
    body: RequestBody<DictDeleteReq>,
) -> route::Result<DictVo> {
    Data!(
        dict_service
            .delete_dict(&body)
            .await?
            .map(|d| d.into())
            .or(None)
    )
}

#[utoipa::path(
    get,
    tag = "dictionaries",
    context_path = "/api/system/dict/value",
    params(
        DictValueQueryReq
    ),
    responses(
        (status = 200, description = "获取字典值分页列表", body = Pagination<DictValueVo>)
    )
)]
#[get("/value/pages")]
async fn page_dict_values(
    dict_service: Autowired<DictService>,
    query: RequestParam<DictValueQueryReq>,
) -> route::Result<Pagination<DictValueVo>> {
    let (page, size) = match &query.page {
        Some(p) => (p.page, p.size),
        _ => (1, 15),
    };
    Data!(dict_service.page_values(&query, page, size).await?.to())
}

#[utoipa::path(
    post,
    tag = "dictionaries",
    context_path = "/api/system/dict/value",
    request_body = DictValueCreateReq,
    responses(
        (status = 200, description = "创建字典值", body = DictValueVo)
    )
)]
#[post("/value/create")]
async fn create_dict_value(
    dict_service: Autowired<DictService>,
    body: RequestBody<DictValueCreateReq>,
) -> route::Result<DictValueVo> {
    Data!(
        DictValueVo::from(dict_service.create_value(&body).await?)
    )
}

#[utoipa::path(
    post,
    tag = "dictionaries",
    context_path = "/api/system/dict/value",
    request_body = DictValueUpdateReq,
    responses(
        (status = 200, description = "更新字典值", body = DictValueVo)
    )
)]
#[post("/value/update")]
async fn update_dict_value(
    dict_service: Autowired<DictService>,
    body: RequestBody<DictValueUpdateReq>,
) -> route::Result<DictValueVo> {
    Data!(
        dict_service
            .update_value(&body)
            .await?
            .map(|d| d.into())
            .or(None)
    )
}

#[utoipa::path(
    post,
    tag = "dictionaries",
    context_path = "/api/system/dict/value",
    request_body = DictValueDeleteReq,
    responses(
        (status = 200, description = "删除字典值", body = DictValueVo)
    )
)]
#[post("/value/delete")]
async fn delete_dict_value(
    dict_service: Autowired<DictService>,
    body: RequestBody<DictValueDeleteReq>,
) -> route::Result<DictValueVo> {
    Data!(
        dict_service
            .delete_value(&body)
            .await?
            .map(|d| d.into())
            .or(None)
    )
}
