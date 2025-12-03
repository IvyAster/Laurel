use std::sync::Arc;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use laurel_common::types::{api};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBo{
    pub account_id: String,

    pub account_name: String,

    pub account_state: String,

    pub account_type: String,

    /// 创建时间
    pub cts: String,

    /// 更新时间
    pub uts: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPayloadBo{
    pub account_id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenParseQuery{
    pub token: String,
}


#[derive(Clone, Debug)]
pub struct AccountApi(laurel_middleware::request::Client);


impl AccountApi{

    pub fn new(client: laurel_middleware::request::Client) -> Self{
        Self(client)
    }

    pub fn build(client: Arc<ClientWithMiddleware>, host: String, path: Option<String>) -> Self{
        Self(laurel_middleware::request::Client::new(client, host, path))
    }

    pub async fn find_with_id(&self, account_id: &str) -> api::Result<AccountBo>{
        let url = self.0.url("/interface/uc/account");
        let resp = self.0.client()
            .get(url)
            .query(&[("accountId", account_id)])
            .send()
            .await?
            .json::<api::ApiResult<AccountBo>>()
            .await?;
        Ok(resp)
    }

    pub async fn find_with_account(&self, account_name: &str, account_type: &str)-> api::Result<AccountBo>{
        let url = self.0.url("/interface/uc/account");
        let resp = self.0.client()
            .get(url)
            .query(&[("accountName", account_name), ("accountType", account_type)])
            .send()
            .await?
            .json::<api::ApiResult<AccountBo>>()
            .await?;
        Ok(resp)
    }

    pub async fn parse_token(&self, token: &str) -> api::Result<TokenPayloadBo>{
        let url = self.0.url("/interface/uc/account/token");
        let resp = self.0.client()
            .get(url)
            .query(&[("token", token)])
            .send()
            .await?
            .json::<api::ApiResult<TokenPayloadBo>>()
            .await?;
        Ok(resp)
    }
}


