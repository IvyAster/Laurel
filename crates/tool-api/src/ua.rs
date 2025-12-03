use std::sync::Arc;
use serde::{Deserialize, Serialize};
use laurel_common::types::{api};

#[derive(Clone, Debug)]
pub struct UaApi(laurel_middleware::request::Client);


static UA_PARSE_URL: &'static str = "/interface/tools/ua/parse";


impl UaApi{
    pub fn new(client: laurel_middleware::request::Client) -> Self {
        UaApi(client)
    }

    pub fn build(client: Arc<reqwest_middleware::ClientWithMiddleware>, host: String, path: Option<String>) -> Self {
        UaApi(laurel_middleware::request::Client::new(client, host, path))
    }


    pub async fn parse(&self, ua: & str) -> api::Result<Ua>{
        let url = self.0.url(UA_PARSE_URL);
        let resp = self.0.client()
            .post(url)
            .json(&UaParseReqBo{ua: ua.to_string()})
            .send()
            .await?
            .json::<api::ApiResult<Ua>>()
            .await?;
        Ok(resp)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UaParseReqBo{
    pub ua: String
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ua{
    pub browser: Option<UaBrowser>,
    pub os: Option<UaOs>,
    pub device: Option<UaDevice>,
}

impl Ua{
    pub fn show(&self) -> (Option<String>, Option<String>, Option<String>){
        let browser = match &self.browser {
            Some(b) => Some(b.show()),
            _ => None
        };
        let os = match &self.os {
            Some(o) => Some(o.show()),
            _ => None
        };
        let device = match &self.device {
            Some(d) => Some(d.show()),
            _ => None
        };
        (browser, os, device)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UaBrowser {
    pub family: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UaOs{
    pub os: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UaDevice{
    pub device: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
}

pub trait Show{
    fn show(&self) -> String;
}

impl Show for UaBrowser{
    fn show(&self) -> String {
        let mut s = String::from("");
        match &self.family {
            Some(f) => s.push_str(f),
            _ => {},
        }
        match &self.major {
            Some(m) => s.push_str(format!(": {}", m.as_str()).as_str()),
            _ => {},
        }
        match &self.minor {
            Some(m) => s.push_str(format!(".{}", m.as_str()).as_str()),
            _ => {},
        }
        match &self.patch {
            Some(p) => s.push_str(format!(".{}", p.as_str()).as_str()),
            _ => {},
        }
        s
    }
}

impl Show for UaOs{
    fn show(&self) -> String {
        let mut s = String::from("");
        match &self.os {
            Some(f) => s.push_str(f),
            _ => {},
        }
        match &self.major {
            Some(m) => s.push_str(format!(": {}", m.as_str()).as_str()),
            _ => {},
        }
        match &self.minor {
            Some(m) => s.push_str(format!(".{}", m.as_str()).as_str()),
            _ => {},
        }
        match &self.patch {
            Some(p) => s.push_str(format!(".{}", p.as_str()).as_str()),
            _ => {},
        }
        match &self.patch_minor {
            Some(p) => s.push_str(format!(".{}", p.as_str()).as_str()),
            _ => {},
        }
        s
    }
}

impl Show for UaDevice{
    fn show(&self) -> String {
        let mut s = String::from("");
        match &self.device {
            Some(f) => s.push_str(f),
            _ => {},
        }
        match &self.brand {
            Some(m) => s.push_str(format!(": {}", m.as_str()).as_str()),
            _ => {},
        }
        match &self.model {
            Some(m) => s.push_str(format!(".{}", m.as_str()).as_str()),
            _ => {},
        }
        s
    }
}