use std::sync::Arc;
use serde::{Deserialize, Serialize};
use laurel_common::types::{api};




#[derive(Clone, Debug)]
pub struct IpApi(laurel_middleware::request::Client);

static LOCATION_API:&'static str = "/interface/tools/ip/location";

impl IpApi {

    pub fn new(client: laurel_middleware::request::Client) -> Self {
        IpApi(client)
    }

    pub fn build(client: Arc<reqwest_middleware::ClientWithMiddleware>, host: String, path: Option<String>) -> Self {
        IpApi(laurel_middleware::request::Client::new(client, host, path))
    }

    pub async fn location(&self, ip: &str) -> api::Result<IpLocationBo>{
        let url = self.0.url(LOCATION_API);

        let resp = self.0.client()
            .post(url)
            .json(&IpReqBo{
                ip: ip.to_string(),
            })
            .send()
            .await?
            .json::<api::ApiResult<IpLocationBo>>()
            .await?;
        Ok(resp)
    }
}

#[derive(Deserialize, Serialize)]
pub struct IpReqBo{
    pub ip: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IpLocationBo{
    pub contry: Option<String>,
    pub region: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub isp: Option<String>,
}

pub static LOCAL_IP: &'static str = "内网IP";

impl IpLocationBo {
    pub fn show(&self) -> String {
        if self.is_local_ip(){
            return LOCAL_IP.to_string();
        }
        format!(
            "{}/{}/{}",
            self.contry.as_ref().unwrap_or(&"".to_string()).as_str(),
            self.region.as_ref().unwrap_or(&"".to_string()).as_str(),
            self.city.as_ref().unwrap_or(&"".to_string()).as_str()
        )
    }

    pub fn is_local_ip(&self) -> bool{
        let mut local = match &self.province {
            Some(p) => p.as_str() == LOCAL_IP,
            _ => false,
        };
        if local{
            return local;
        }
        local = match &self.city {
            Some(c) => c.as_str() == LOCAL_IP,
            _ => false,
        };
        local
    }

    pub fn full_show(&self) -> String{
        if self.is_local_ip(){
            return LOCAL_IP.to_string();
        }
        format!(
            "{}/{}/{}/{}: {}",
            self.contry.as_ref().unwrap_or(&"".to_string()).as_str(),
            self.region.as_ref().unwrap_or(&"".to_string()).as_str(),
            self.province.as_ref().unwrap_or(&"".to_string()).as_str(),
            self.city.as_ref().unwrap_or(&"".to_string()).as_str(),
            self.isp.as_ref().unwrap_or(&"".to_string()).as_str()
        )
    }
}
