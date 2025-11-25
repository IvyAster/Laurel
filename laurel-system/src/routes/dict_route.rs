use crate::model::dict_model::{
    DictCreateReq, DictDeleteReq, DictQueryReq, DictUpdateReq, DictValueCreateReq,
    DictValueDeleteReq, DictValueQueryReq, DictValueUpdateReq, DictValueVo, DictVo,
};
use crate::service::dict_service::DictService;
use actix_web::{get, post, web};
use laurel_actix::types::{Autowired, Done, LR, RequestBody, RequestParam};
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
) -> Done<Pagination<DictVo>> {
    let (page, size) = match &query.page {
        Some(p) => (p.page, p.size),
        _ => (1, 15),
    };

    Ok(LR::of(
        dict_service.page_dict(&query, page, size).await?.to(),
    ))
}

#[utoipa::path(
    post,
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
) -> Done<DictVo> {
    Ok(LR::of(dict_service.create_dict(&body).await?.into()))
}

#[utoipa::path(
    post,
    context_path = "/api/system/dict",
    request_body = DictUpdateReq,
    responses(
        (status = 200, description = "更新字典", body = DictVo)
    )
)]
#[post("/update")]
async fn update_dict(
    dict_service: Autowired<DictService>,
    body: RequestBody<DictUpdateReq>,
) -> Done<DictVo> {
    Ok(LR::of_raw(
        dict_service
            .update_dict(&body)
            .await?
            .map(|d| d.into())
            .or(None),
    ))
}

#[utoipa::path(
    post,
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
) -> Done<DictVo> {
    Ok(LR::of_raw(
        dict_service
            .delete_dict(&body)
            .await?
            .map(|d| d.into())
            .or(None),
    ))
}

#[utoipa::path(
    get,
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
) -> Done<Pagination<DictValueVo>> {
    let (page, size) = match &query.page {
        Some(p) => (p.page, p.size),
        _ => (1, 15),
    };
    Ok(LR::of(
        dict_service.page_values(&query, page, size).await?.to(),
    ))
}

#[utoipa::path(
    post,
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
) -> Done<DictValueVo> {
    Ok(LR::of(dict_service.create_value(&body).await?.into()))
}

#[utoipa::path(
    post,
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
) -> Done<DictValueVo> {
    Ok(LR::of_raw(
        dict_service
            .update_value(&body)
            .await?
            .map(|d| d.into())
            .or(None),
    ))
}

#[utoipa::path(
    post,
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
) -> Done<DictValueVo> {
    Ok(LR::of_raw(
        dict_service
            .delete_value(&body)
            .await?
            .map(|d| d.into())
            .or(None),
    ))
}
