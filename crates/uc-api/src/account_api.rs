use std::sync::Arc;
use bon::Builder;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use laurel_common::types::{ApiResult, LrApi};

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


#[derive(Clone, Debug, Builder)]
pub struct AccountApi {
    pub client: Arc<ClientWithMiddleware>,
    pub host: String,
    pub path: Option<String>,
}


impl AccountApi{

    pub async fn find_with_id(&self, account_id: &str) -> ApiResult<AccountBo>{
        let url = format!("{}{}", self.host, "/interface/uc/account");
        let resp = Arc::clone(&self.client).get(url)
            .query(&[("accountId", account_id)])
            .send()
            .await?
            .json::<LrApi<AccountBo>>()
            .await?;
        Ok(resp)
    }

    pub async fn find_with_account(&self, account_name: &str, account_type: &str)-> ApiResult<AccountBo>{
        let url = format!("{}{}", self.host, "/interface/uc/account");
        let resp = Arc::clone(&self.client).get(url)
            .query(&[("accountName", account_name), ("accountType", account_type)])
            .send()
            .await?
            .json::<LrApi<AccountBo>>()
            .await?;
        Ok(resp)
    }

    pub async fn parse_token(&self, token: &str) -> ApiResult<TokenPayloadBo>{
        let url = format!("{}{}", self.host, "/interface/uc/account/token");
        let resp = Arc::clone(&self.client).get(url)
            .query(&[("token", token)])
            .send()
            .await?
            .json::<LrApi<TokenPayloadBo>>()
            .await?;
        Ok(resp)
    }
}


