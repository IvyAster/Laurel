use crate::model::account_model::{AccountEntity, AccountLoginVo};
use crate::repository::account_repository::AccountRepository;
use crate::repository::passport_repository::PassportRepository;
use crate::utils::passport_utils::Passports;
use crate::utils::token_utils;
use anyhow::Error;
use laurel_actix::handler::Token;
use laurel_actix::types::Running;
use laurel_redis::Redis;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug)]
pub struct AccountService {
    account_repository: Arc<AccountRepository>,
    passport_repository: Arc<PassportRepository>,
    redis: Redis,
}

impl AccountService {
    pub fn new(
        account_repository: Arc<AccountRepository>,
        passport_repository: Arc<PassportRepository>,
        redis: Redis,
    ) -> Self {
        Self {
            account_repository,
            passport_repository,
            redis,
        }
    }
    pub async fn login(&self, req: &AccountLoginVo) -> Running<(AccountEntity, String)> {
        let account = self
            .account_repository
            .find_by_name(req.account.as_str(), "name")
            .await?
            .expect("account not found");
        let passport = self
            .passport_repository
            .find(account.account_id.as_str())
            .await?
            .expect("account passport not init");
        let p = Passports::password(
            account.account_id.as_str(),
            req.password.as_str(),
            passport.salt.as_str(),
        )?;
        if p == passport.password {
            let token = token_utils::token();
            self.redis
                .set::<String>(
                    &token.as_str(),
                    json!(Token {
                        account_id: account.account_id.clone()
                    })
                    .to_string(),
                )
                .await?;
            Ok((account, token))
        } else {
            Err(Error::msg("account passport error"))
        }
    }

    pub async fn find_account_by_id(&self, account_id: &str) -> Running<Option<AccountEntity>> {
        let account = self
            .account_repository
            .find_by_account_id(account_id)
            .await?;
        Ok(account)
    }

    pub async fn find_account_by_account(
        &self,
        account_name: &str,
        account_type: &str,
    ) -> Running<Option<AccountEntity>> {
        let account = self
            .account_repository
            .find_by_name(account_name, account_type)
            .await?;
        Ok(account)
    }
}
