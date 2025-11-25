use std::sync::Arc;
use bon::Builder;
use reqwest_middleware::ClientWithMiddleware;
use laurel_common::types::LrApi;

#[derive(Builder, Debug)]
pub struct IdApi {
    pub(crate) client: Arc<ClientWithMiddleware>,
    pub host: String,
    pub path: Option<String>,
}

impl IdApi{
    pub fn new(client: Arc<ClientWithMiddleware>, host: String, path: Option<String>) -> Self{
        Self{
            client,
            host,
            path,
        }
    }

    pub async fn id(&self) -> Result<String, anyhow::Error>{
        let url = format!("{}{}", self.host, "/id");
        Ok(self.client.get(url).send().await?.text().await?)
    }

    pub async fn id_with_size(&self, size: usize) -> Result<Vec<String>, anyhow::Error>{
        let url = format!("{}/ids/{id}", self.host, id=size);
        Ok(
            Arc::clone(&self.client).get(url).send().await?.json::<Vec<String>>().await?
        )
    }

    pub async fn api_id(&self) -> Result<LrApi<String>, anyhow::Error>{
        let url = format!("{}{}", self.host, "/api/id");
        Ok(
            Arc::clone(&self.client).get(url).send().await?.json::<LrApi<String>>().await?
        )
    }

    pub async fn api_id_with_size(&self, size: usize) -> Result<LrApi<Vec<String>>, anyhow::Error>{
        let url = format!("{}/api/ids/{id}", self.host, id=size);
        Ok(
            Arc::clone(&self.client).get(url).send().await?.json::<LrApi<Vec<String>>>().await?
        )
    }
}