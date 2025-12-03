pub mod reqwest_middle;

pub mod request{
    use std::sync::Arc;
    use reqwest_middleware::ClientWithMiddleware;

    #[derive(Clone, Debug)]
    pub struct Client{
        client: Arc<ClientWithMiddleware>,
        host: String,
        path: Option<String>,
    }

    impl Client{
        pub fn new(client: Arc<ClientWithMiddleware>, host: String, path: Option<String>) -> Self{
            Self{
                client,
                host,
                path,
            }
        }

        pub fn url(&self, uri: &str) -> String{
            match &self.path {
                Some(path) => format!("{}{}{}", self.host, path.as_str(), uri),
                _ => format!("{}{}", self.host, uri),
            }
        }

        pub fn client(&self) -> Arc<ClientWithMiddleware>{
            Arc::clone(&self.client)
        }
    }
}