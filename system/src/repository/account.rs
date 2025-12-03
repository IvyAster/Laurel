use crate::model::account::AccountEntity;
use crate::schema::schema::account::dsl as AccountDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use laurel_actix::types::{repository};
use laurel_pg::DbPool; //::*;

#[derive(Clone, Debug)]
pub struct AccountRepository {
    pool: DbPool,
}

impl AccountRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_account_id(&self, account_id: &str) -> repository::Result<Option<AccountEntity>> {
        let mut conn = self.pool.get().await?;
        let account = AccountDsl::account
            .filter(AccountDsl::account_id.eq(account_id))
            .select(AccountEntity::as_select())
            .first(&mut conn)
            .await
            .optional()?;
        Ok(account)

        // let result = account
        //     .select(AccountEntity::as_select())
        //     .load(&mut conn)
        //     .await?;
    }

    pub async fn find_by_name(
        &self,
        account_name: &str,
        account_type: &str,
    ) -> repository::Result<Option<AccountEntity>> {
        let mut conn = self.pool.get().await?;
        let account = AccountDsl::account
            .filter(AccountDsl::account_name.eq(account_name))
            .filter(AccountDsl::account_type.eq(account_type))
            .select(AccountEntity::as_select())
            .first(&mut conn)
            .await
            .optional()?;
        Ok(account)
    }
}
