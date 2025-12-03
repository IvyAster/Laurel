use crate::model::passport::PassportEntity;
use crate::schema::schema::passport::dsl as PassportDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use laurel_actix::types::{repository};
use laurel_pg::DbPool;

#[derive(Clone, Debug)]
pub struct PassportRepository {
    pool: DbPool,
}

impl PassportRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, account_id: &str) -> repository::Result<Option<PassportEntity>> {
        let mut conn = self.pool.get().await?;
        let passport = PassportDsl::passport
            .filter(PassportDsl::account_id.eq(account_id))
            .select(PassportEntity::as_select())
            .first(&mut conn)
            .await
            .optional()?;
        Ok(passport)
    }
}
