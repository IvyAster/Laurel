use std::sync::Arc;
use bon::Builder;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use laurel_common::types::{ApiResult, LrApi};

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

#[derive(Clone, Debug, Builder)]
pub struct ProfileApi {
    pub(crate) client: Arc<ClientWithMiddleware>,
    pub host: String,
    pub path: Option<String>,
}


// impl UcApi for ProfileApi {
//     fn build(client: Arc<ClientWithMiddleware>, host: String, path: Option<String>)-> Self{
//         ProfileApi {
//             client,
//             host,
//             path,
//         }
//     }
// }

impl ProfileApi{
    pub async fn list_profiles(&self, query: &ProfileQuery) -> ApiResult<Vec<ProfileBo>>{
        let url = format!("{}{}", self.host, "/interface/uc/profile/profiles");
        let resp = Arc::clone(&self.client)
            .post(url)
            .json(&query)
            .send()
            .await?
            .json::<LrApi<Vec<ProfileBo>>>()
            .await?;
        Ok(resp)
    }
}