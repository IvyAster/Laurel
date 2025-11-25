use crate::model::menu_model::{InsertAbleMenu, Menu, MenuQuery, UpdatableMenu};
use crate::repository::{AsyncDsl, IntRow, StringRow};
use crate::schema::schema::menu as MenuSchema;
use crate::schema::schema::menu::dsl as MenuDsl;
use diesel::associations::HasTable;
use diesel::{
    ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl,
    SelectableHelper,
};
use diesel_async::*;
use laurel_actix::types::Running;
use laurel_common::types::Pagination;
use laurel_pg::DbPool;

#[derive(Clone, Debug)]
pub struct MenuRepository {
    pool: DbPool,
}

impl MenuRepository {
    pub fn new(pool: DbPool) -> Self {
        MenuRepository { pool }
    }

    pub async fn find_menu(&self, menu_id: &str) -> Running<Option<Menu>> {
        let mut conn = self.pool.get().await?;
        let menu = AsyncDsl::first(
            MenuDsl::menu::table()
                .filter(MenuDsl::menu_id.eq(menu_id))
                .select(Menu::as_select()),
            &mut conn,
        )
        .await
        .optional()?;
        Ok(menu)
    }

    pub async fn list_menus<'a>(&self, query: &MenuQuery<'a>) -> Running<Vec<Menu>> {
        let mut conn = self.pool.get().await?;
        let menus = AsyncDsl::load(
            self.apply_filters(query, MenuDsl::menu.into_boxed())
                .select(Menu::as_select())
                .order_by(MenuSchema::weight.asc())
                .then_order_by(MenuSchema::id.asc()),
            &mut conn,
        )
        .await?;
        Ok(menus)
    }

    pub async fn page_menus<'a>(
        &self,
        query: &MenuQuery<'a>,
        page: u32,
        size: u32,
    ) -> Running<Pagination<Menu>> {
        let mut conn = self.pool.get().await?;
        let total = AsyncDsl::get_result::<i64>(
            self.apply_filters(query, MenuDsl::menu.into_boxed())
                .select(diesel::dsl::count_star()),
            &mut conn,
        )
        .await?;
        let offset = (page - 1) * size;
        if total <= 0 {
            return Ok(Pagination {
                page,
                size,
                pages: 0,
                total: 0,
                data: Some(vec![]),
            });
        }
        let pages = (total as f64 / size as f64).ceil() as u64;
        let menus = AsyncDsl::load(
            self.apply_filters(query, MenuDsl::menu.into_boxed())
                .order_by(MenuSchema::id.desc())
                .offset(offset as i64)
                .limit(size as i64)
                .select(Menu::as_returning()),
            &mut conn,
        )
        .await?;
        Ok(Pagination {
            page,
            size,
            pages,
            total: total as u64,
            data: Some(menus),
        })
    }

    pub async fn list_menus_recursive(&self, app_id: &str) -> Running<Vec<Menu>> {
        let mut conn = self.pool.get().await?;
        let query = diesel::sql_query(
            r#"
                WITH RECURSIVE temp_menu AS(
                    SELECT * FROM menu WHERE app_id = $1 AND menu_id = parent_id AND menu_status = 'open'
                    UNION ALL
                    SELECT m.* FROM menu AS m INNER JOIN temp_menu AS tm ON m.parent_id = tm.menu_id AND m.menu_id != m.parent_id AND m.menu_status = 'open'
                )
                SELECT * FROM temp_menu ORDER BY weight ASC, id ASC
            "#
        ).bind::<diesel::sql_types::VarChar, _>(app_id);
        let menus = AsyncDsl::load::<Menu>(query, &mut conn).await?;
        Ok(menus)
    }

    pub async fn check_has_child(&self, menu_id: &str, child_id: &str) -> Running<bool> {
        let mut conn = self.pool.get().await?;
        let query = diesel::sql_query(
            r#"
            WITH RECURSIVE temp_menu AS(
                SELECT menu_id FROM menu WHERE menu_id != parent_id AND menu_status = 'open' AND parent_id = $1
                UNION ALL
                SELECT m.menu_id FROM menu AS m
                INNER JOIN temp_menu AS tm ON m.parent_id = tm.menu_id AND m.menu_id != m.parent_id AND m.menu_status = 'open'
            )
            SELECT COUNT(DISTINCT menu_id) AS row_result FROM temp_menu WHERE menu_id = $2
            "#
        )
            .bind::<diesel::sql_types::VarChar, _>(menu_id)
            .bind::<diesel::sql_types::VarChar, _>(child_id);
        let count = AsyncDsl::get_result::<IntRow>(query, &mut conn)
            .await?
            .row_result;
        Ok(count > 0)
    }

    pub async fn list_child_menu_ids(&self, menu_id: &str) -> Running<Vec<String>> {
        let mut conn = self.pool.get().await?;
        let query = diesel::sql_query(
            r#"
            WITH RECURSIVE temp_menu AS(
                SELECT menu_id FROM menu WHERE menu_id != parent_id AND menu_status = 'open' AND parent_id = $1
                UNION ALL
                SELECT m.menu_id FROM menu AS m
                INNER JOIN temp_menu AS tm ON m.parent_id = tm.menu_id AND m.menu_id != m.parent_id AND m.menu_status = 'open'
            )
            SELECT DISTINCT menu_id AS row_result FROM temp_menu
            "#
        )
            .bind::<diesel::sql_types::VarChar, _>(menu_id);
        let ids: Vec<String> = AsyncDsl::load::<StringRow>(query, &mut conn)
            .await?
            .into_iter()
            .map(|row| row.row_result)
            .collect();
        Ok(ids)
    }

    pub async fn check_has_parent(&self, menu_id: &str, parent_id: &str) -> Running<bool> {
        let mut conn = self.pool.get().await?;
        let query = diesel::sql_query(
            r#"
            WITH RECURSIVE temp_menu AS(
                SELECT parent_id FROM menu WHERE menu_id != parent_id AND menu_status = 'open' AND menu_id = $1
                UNION ALL
                SELECT m.parent_id FROM menu AS m
                INNER JOIN temp_menu AS tm ON m.menu_id = tm.parent_id AND m.menu_id != m.parent_id AND m.menu_status = 'open'
            )
            SELECT COUNT(DISTINCT parent_id) AS row_result FROM temp_menu WHERE parent_id = $2
            "#
        )
            .bind::<diesel::sql_types::VarChar, _>(menu_id)
            .bind::<diesel::sql_types::VarChar, _>(parent_id);
        let count = AsyncDsl::get_result::<IntRow>(query, &mut conn)
            .await?
            .row_result;
        Ok(count > 0)
    }

    pub async fn list_parent_menu_ids(&self, menu_id: &str) -> Running<Vec<String>> {
        let mut conn = self.pool.get().await?;
        let query = diesel::sql_query(
            r#"
            WITH RECURSIVE temp_menu AS(
                SELECT parent_id FROM menu WHERE menu_id != parent_id AND menu_status = 'open' AND menu_id = $1
                UNION ALL
                SELECT m.parent_id FROM menu AS m
                INNER JOIN temp_menu AS tm ON m.menu_id = tm.parent_id AND m.menu_id != m.parent_id AND m.menu_status = 'open'
            )
            SELECT DISTINCT parent_id AS row_result FROM temp_menu
            "#
        )
            .bind::<diesel::sql_types::VarChar, _>(menu_id);
        let ids: Vec<String> = AsyncDsl::load::<StringRow>(query, &mut conn)
            .await?
            .into_iter()
            .map(|row| row.row_result)
            .collect();
        Ok(ids)
    }

    pub async fn save_menu<'a>(&self, insertable: &InsertAbleMenu<'a>) -> Running<Menu> {
        let mut conn = self.pool.get().await?;
        let menu = conn
            .transaction::<Menu, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let menu = AsyncDsl::get_result(
                        diesel::insert_into(MenuDsl::menu)
                            .values(insertable)
                            .returning(Menu::as_returning()),
                        &mut tx,
                    )
                    .await?;
                    Ok(menu)
                })
            })
            .await?;
        Ok(menu)
    }

    pub async fn update_menu(
        &self,
        menu_id: &str,
        updatable: &UpdatableMenu,
    ) -> Running<Option<Menu>> {
        let mut conn = self.pool.get().await?;
        let menu = conn
            .transaction::<Option<Menu>, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let menu = AsyncDsl::get_result(
                        diesel::update(MenuDsl::menu)
                            .filter(MenuDsl::menu_id.eq(menu_id))
                            .set(updatable)
                            .returning(Menu::as_returning()),
                        &mut tx,
                    )
                    .await
                    .optional()?;
                    Ok(menu)
                })
            })
            .await?;

        Ok(menu)
    }

    fn apply_filters<'a>(
        &self,
        params: &'a MenuQuery,
        mut query: MenuSchema::BoxedQuery<'a, diesel::pg::Pg>,
    ) -> MenuSchema::BoxedQuery<'a, diesel::pg::Pg> {
        query = query.filter(MenuDsl::app_id.eq(params.app_id));
        if let Some(param) = params.menu_id {
            query = query.filter(MenuDsl::menu_id.eq(param));
        }
        if let Some(param) = params.menu_ids {
            query = query.filter(MenuDsl::menu_id.eq_any(param));
        }
        if let Some(param) = params.menu_name
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::menu_name.eq(param));
        }
        if let Some(param) = params.menu_type
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::menu_type.eq(param));
        }
        if let Some(param) = params.menu_types
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::menu_type.eq_any(param));
        }
        if let Some(param) = params.menu_action_type
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::menu_action_type.eq(param));
        }
        if let Some(param) = params.menu_status
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::menu_status.eq(param));
        }
        if let Some(param) = params.menu_statuses
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::menu_status.eq_any(param));
        }
        if let Some(param) = params.menu_route
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::menu_route.ilike(format!("%{}%", param)));
        }
        if let Some(param) = params.parent_id
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::parent_id.eq(param));
        }
        if let Some(param) = params.parent_ids
            && !param.is_empty()
        {
            query = query.filter(MenuDsl::parent_id.eq_any(param));
        }
        if let Some(param) = params.authority
            && !param.trim().is_empty()
        {
            query = query.filter(MenuDsl::authority.ilike(format!("%{}%", param)));
        }
        query
    }
}
