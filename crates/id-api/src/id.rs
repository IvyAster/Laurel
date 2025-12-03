use std::sync::Arc;
use reqwest_middleware::ClientWithMiddleware;
use laurel_common::types::api::ApiResult;

#[derive(Clone, Debug)]
pub struct IdApi(laurel_middleware::request::Client);

impl IdApi{
    pub fn new(client: laurel_middleware::request::Client) -> Self{
        Self(client)
    }
    pub fn build(client: Arc<ClientWithMiddleware>, host: String, path: Option<String>) -> Self{
        Self(laurel_middleware::request::Client::new( client, host, path))
    }

    pub async fn id(&self) -> Result<String, anyhow::Error>{
        let url = self.0.url("/id");
        Ok(self.0.client().get(url).send().await?.text().await?)
    }

    pub async fn id_with_size(&self, size: usize) -> Result<Vec<String>, anyhow::Error>{
        let url = format!("{}/ids/{id}", self.0.url( ""), id=size);
        Ok(
            self.0.client().get(url).send().await?.json::<Vec<String>>().await?
        )
    }

    pub async fn api_id(&self) -> Result<ApiResult<String>, anyhow::Error>{
        let url = self.0.url("/api/id");
        Ok(
            self.0.client().get(url).send().await?.json::<ApiResult<String>>().await?
        )
    }

    pub async fn api_id_with_size(&self, size: usize) -> Result<ApiResult<Vec<String>>, anyhow::Error>{
        let url = format!("{}/api/ids/{id}", self.0.url( ""), id=size);
        Ok(
            self.0.client().get(url).send().await?.json::<ApiResult<Vec<String>>>().await?
        )
    }
}