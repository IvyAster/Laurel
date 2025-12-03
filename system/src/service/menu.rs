use crate::model::menu::{
    InsertAbleMenu, Menu, MenuCreateReq, MenuQueryReq, MenuUpdateReq, UpdatableMenu,
};
use crate::repository::menu::MenuRepository;
use anyhow::Error;
use bon::Builder;
use chrono::Local;
use laurel_common::types::Pagination;
use laurel_id_api::id::IdApi;
use std::sync::Arc;
use laurel_actix::types::service;

#[derive(Debug, Builder)]
pub struct MenuService {
    pub menu_repository: Arc<MenuRepository>,
    pub id_api: IdApi,
}

impl MenuService {
    pub async fn list_menus(&self, req: &MenuQueryReq) -> service::Result<Vec<Menu>> {
        let query = req.into();
        self.menu_repository.list_menus(&query).await
    }

    pub async fn find_menu(&self, menu_id: &str) -> service::Result<Option<Menu>> {
        self.menu_repository.find_menu(menu_id).await
    }

    pub async fn list_used_menus(&self, app_id: &str) -> service::Result<Vec<Menu>> {
        self.menu_repository.list_menus_recursive(app_id).await
    }

    pub async fn page_menus(
        &self,
        req: &MenuQueryReq,
        page: u32,
        size: u32,
    ) -> service::Result<Pagination<Menu>> {
        let query = req.into();
        self.menu_repository.page_menus(&query, page, size).await
    }

    pub async fn create_menu(&self, req: &MenuCreateReq) -> service::Result<Menu> {
        let id = self.id_api.id().await?;
        let now = Local::now().naive_local();
        let insertable_menu = InsertAbleMenu {
            app_id: req.app_id.as_str(),
            menu_id: id.as_str(),
            menu_name: req.menu_name.as_str(),
            menu_type: req.menu_type.as_str(),
            menu_action_type: req.menu_action_type.as_str(),
            menu_icon: req.menu_icon.clone(),
            menu_route: req.menu_route.clone(),
            route_param: req.route_param.clone(),
            weight: req.weight,
            parent_id: if let Some(x) = &req.parent_id
                && !x.is_empty()
            {
                x.as_str()
            } else {
                id.as_str()
            },
            authority: req.authority.clone(),
            menu_status: if let Some(x) = &req.menu_status
                && !x.is_empty()
            {
                x.as_str()
            } else {
                "open"
            },
            cts: &now,
            uts: &now,
        };
        self.menu_repository.save_menu(&insertable_menu).await
    }

    pub async fn update_menu(&self, req: MenuUpdateReq) -> service::Result<Menu> {
        if req.menu_id.trim().is_empty() {
            return Err(Error::msg("菜单id不能为空"));
        }
        let check = self.menu_repository.find_menu(req.menu_id.as_str()).await?;
        match check {
            Some(menu) => {
                if menu.menu_status == "deleted" {
                    return Err(Error::msg("当前菜单已删除"));
                }
            }
            None => {
                return Err(Error::msg("当前菜单不存在"));
            }
        }
        match &req.parent_id {
            Some(parent_id) => {
                if !(parent_id == &req.menu_id) {
                    if self
                        .menu_repository
                        .check_has_child(req.menu_id.as_str(), parent_id.as_str())
                        .await?
                    {
                        return Err(Error::msg("无法将菜单移动到其子菜单下"));
                    }
                }
            }
            None => {
                // 将其转移至顶级
                // do nothing
                //req.parent_id = Some(req.menu_id.clone());
            }
        }

        let updatable = UpdatableMenu {
            menu_name: req.menu_name,
            menu_action_type: req.menu_action_type,
            menu_icon: req.menu_icon.clone(),
            menu_route: req.menu_route,
            route_param: req.route_param,
            weight: req.weight,
            parent_id: req.parent_id,
            authority: req.authority,
            menu_status: req.menu_status,
            uts: Local::now().naive_local(),
        };
        Ok(self
            .menu_repository
            .update_menu(req.menu_id.as_str(), &updatable)
            .await?
            .expect("当前菜单不存在, 更新失败"))
    }
}
