use crate::model::menu::{
    Menu, MenuActionType, MenuCreateReq, MenuQueryReq, MenuStatus, MenuType, MenuUpdateReq, MenuVo,
};
use crate::service::menu::MenuService;
use actix_web::{get, post, web};
use laurel_actix::Data;
use laurel_actix::types::{Autowired, RequestBody, RequestParam, route};
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
) -> route::Result<MenuVo> {
    let menu: MenuVo = menu_service.create_menu(&req).await?.into();
    Data!(menu)
}

#[get("")]
async fn find_menu(
    menu_service: Autowired<MenuService>,
    req: RequestParam<MenuUpdateReq>,
) -> route::Result<MenuVo> {
    let menu = match menu_service
        .menu_repository
        .find_menu(req.menu_id.as_str())
        .await?
    {
        Some(menu) => Some(menu.into()),
        _ => None,
    };
    Data!(menu)
}

#[post("/update")]
async fn update_menu(
    menu_service: web::Data<MenuService>,
    req: RequestBody<MenuUpdateReq>,
) -> route::Result<MenuVo> {
    let body = req.into_inner();
    let menu: MenuVo = menu_service.update_menu(body).await?.into();
    Data!(menu)
}

#[get("/state/options")]
async fn list_menu_status_options() -> route::Result<Vec<SelectOption<&'static str, &'static str>>>
{
    Data!(MenuStatus::options())
}

#[get("/action/options")]
async fn list_menu_action_options() -> route::Result<Vec<SelectOption<&'static str, &'static str>>>
{
    Data!(MenuActionType::options())
}

#[get("/type/options")]
async fn list_menu_type_options() -> route::Result<Vec<SelectOption<&'static str, &'static str>>> {
    Data!(MenuType::options())
}

#[post("/list")]
async fn list_menus(
    menu_service: web::Data<MenuService>,
    query: RequestBody<MenuQueryReq>,
) -> route::Result<Vec<MenuVo>> {
    let menus: Vec<MenuVo> = menu_service
        .list_menus(&query)
        .await?
        .into_iter()
        .map(|menu| MenuVo::from(menu))
        .collect();
    Data!(
        menus
    )
}

#[get("/tree")]
async fn tree_used_menus(
    menu_service: web::Data<MenuService>,
    query: RequestParam<MenuQueryReq>,
) -> route::Result<Vec<MenuVo>> {
    Data!(Menu::build_tree(
        menu_service.list_used_menus(query.app_id.as_str()).await?
    ))
}

#[post("/tree/all")]
async fn tree_all_menus(
    menu_service: web::Data<MenuService>,
    req: RequestBody<MenuQueryReq>,
) -> route::Result<Vec<MenuVo>> {
    Data!(Menu::build_tree(menu_service.list_menus(&req).await?,))
}

#[post("/page")]
async fn page_menus(
    menu_service: web::Data<MenuService>,
    req: RequestBody<MenuQueryReq>,
) -> route::Result<Pagination<MenuVo>> {
    let (page, size) = if let Some(p) = &req.pagination {
        (p.page, p.size)
    } else {
        (1, 10)
    };
    Data!(
        menu_service
            .page_menus(&req, page, size)
            .await?
            .to_with_index::<MenuVo>()
    )
}
