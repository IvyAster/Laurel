use crate::model::menu_model::{
    Menu, MenuActionType, MenuCreateReq, MenuQueryReq, MenuStatus, MenuType, MenuUpdateReq, MenuVo,
};
use crate::service::menu_service::MenuService;
use actix_web::{get, post, web};
use laurel_actix::types::{Autowired, Done, LR, RequestBody, RequestParam};
use laurel_common::types::{HappyEnum, Pagination, SelectOption};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/system/menu")
            .service(create_menu)
            .service(list_menu_status_options)
            .service(tree_all_menus)
            .service(tree_used_menus)
            .service(page_menus)
            .service(list_menu_type_options)
            .service(list_menu_action_options)
            .service(list_menu_status_options)
            .service(update_menu)
            .service(find_menu),
    );
}

#[post("/create")]
async fn create_menu(
    menu_service: Autowired<MenuService>,
    req: RequestBody<MenuCreateReq>,
) -> Done<MenuVo> {
    Ok(LR::of(menu_service.create_menu(&req).await?.into()))
}

#[get("")]
async fn find_menu(
    menu_service: Autowired<MenuService>,
    req: RequestParam<MenuUpdateReq>,
) -> Done<MenuVo> {
    Ok(LR::of_raw(
        match menu_service
            .menu_repository
            .find_menu(req.menu_id.as_str())
            .await?
        {
            Some(menu) => Some(menu.into()),
            _ => None,
        },
    ))
}

#[post("/update")]
async fn update_menu(
    menu_service: web::Data<MenuService>,
    req: RequestBody<MenuUpdateReq>,
) -> Done<MenuVo> {
    let body = req.into_inner();
    Ok(LR::of(menu_service.update_menu(body).await?.into()))
}

#[get("/state/options")]
async fn list_menu_status_options() -> Done<Vec<SelectOption<&'static str, &'static str>>> {
    Ok(LR::of(MenuStatus::options()))
}

#[get("/action/options")]
async fn list_menu_action_options() -> Done<Vec<SelectOption<&'static str, &'static str>>> {
    Ok(LR::of(MenuActionType::options()))
}

#[get("/type/options")]
async fn list_menu_type_options() -> Done<Vec<SelectOption<&'static str, &'static str>>> {
    Ok(LR::of(MenuType::options()))
}

#[post("/list")]
async fn list_menus(
    menu_service: web::Data<MenuService>,
    query: RequestBody<MenuQueryReq>,
) -> Done<Vec<MenuVo>> {
    Ok(LR::of(
        menu_service
            .list_menus(&query)
            .await?
            .into_iter()
            .map(|menu| MenuVo::from(menu))
            .collect(),
    ))
}

#[get("/tree")]
async fn tree_used_menus(
    menu_service: web::Data<MenuService>,
    query: RequestParam<MenuQueryReq>,
) -> Done<Vec<MenuVo>> {
    Ok(LR::of(Menu::build_tree(
        menu_service.list_used_menus(query.app_id.as_str()).await?,
    )))
}

#[post("/tree/all")]
async fn tree_all_menus(
    menu_service: web::Data<MenuService>,
    req: RequestBody<MenuQueryReq>,
) -> Done<Vec<MenuVo>> {
    Ok(LR::of(Menu::build_tree(
        menu_service.list_menus(&req).await?,
    )))
}

#[post("/page")]
async fn page_menus(
    menu_service: web::Data<MenuService>,
    req: RequestBody<MenuQueryReq>,
) -> Done<Pagination<MenuVo>> {
    let (page, size) = if let Some(p) = &req.pagination {
        (p.page, p.size)
    } else {
        (1, 10)
    };
    Ok(LR::of(
        menu_service
            .page_menus(&req, page, size)
            .await?
            .to_with_index::<MenuVo>(),
    ))
}
