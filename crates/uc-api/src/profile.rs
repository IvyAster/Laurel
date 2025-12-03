use std::sync::Arc;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use laurel_common::types::{api};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileBo{
    pub account_id: String,

    pub profile_key: String,

    pub profile_value: Option<String>,

    pub cts: String,

    pub uts: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileQuery{
    pub account_id: String,
    pub keys: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct ProfileApi (laurel_middleware::request::Client);

impl ProfileApi{

    pub fn new(client: laurel_middleware::request::Client) -> Self{
        Self(client)
    }

    pub fn build(client: Arc<ClientWithMiddleware>, host: String, path: Option<String>) -> Self{
        Self(laurel_middleware::request::Client::new(client, host, path))
    }

    pub async fn list_profiles(&self, query: &ProfileQuery) -> api::Result<Vec<ProfileBo>>{
        let url = self.0.url("/interface/uc/profile/profiles");
        let resp = self.0.client()
            .post(url)
            .json(&query)
            .send()
            .await?
            .json::<api::ApiResult<Vec<ProfileBo>>>()
            .await?;
        Ok(resp)
    }
}