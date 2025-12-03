use crate::model::role::{InsertableRole, Role, UpdatableRole};
//use crate::schema::schema::role as RoleSchema;
use crate::schema::schema::role::dsl as RoleDsl;
use diesel::ExpressionMethods;
use diesel::associations::HasTable;
use diesel::{OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::*;
use laurel_actix::types::repository;
use laurel_pg::{AsyncDsl, DbPool};

#[derive(Clone, Debug)]
pub struct RoleRepository {
    pool: DbPool,
}

impl RoleRepository {
    pub fn new(pool: DbPool) -> Self {
        RoleRepository { pool }
    }

    pub async fn find(&self, role_id: &str) -> repository::Result<Option<Role>> {
        let mut conn = self.pool.get().await?;
        let role = AsyncDsl::first(
            RoleDsl::role::table()
                .filter(RoleDsl::role_id.eq(role_id))
                .select(Role::as_select()),
            &mut conn,
        )
        .await
        .optional()?;
        Ok(role)
    }

    pub async fn save<'a>(&self, insertable: &InsertableRole<'a>) -> repository::Result<Role> {
        let mut conn = self.pool.get().await?;
        let role = conn
            .transaction::<Role, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let role = AsyncDsl::get_result(
                        diesel::insert_into(RoleDsl::role)
                            .values(insertable)
                            .returning(Role::as_returning()),
                        &mut tx,
                    )
                    .await?;
                    Ok(role)
                })
            })
            .await?;
        Ok(role)
    }

    pub async fn update(&self, role_id: &str, updatable: &UpdatableRole) -> repository::Result<Option<Role>> {
        let mut conn = self.pool.get().await?;
        let role = conn
            .transaction::<Option<Role>, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let role = AsyncDsl::get_result(
                        diesel::update(RoleDsl::role)
                            .filter(RoleDsl::role_id.eq(role_id))
                            .set(updatable)
                            .returning(Role::as_returning()),
                        &mut tx,
                    )
                    .await
                    .optional()?;
                    Ok(role)
                })
            })
            .await?;

        Ok(role)
    }
}
