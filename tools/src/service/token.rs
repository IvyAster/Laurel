use laurel_actix::handler::{Token, TokenHandler, TokenResult};

#[derive(Clone, Debug)]
pub struct TokenService {
    exclude_paths: Vec<String>,
    exclude_start_path: Vec<String>,
}

impl TokenService {
    pub fn new(
        exclude_paths: Vec<String>,
        exclude_start_path: Vec<String>,
    ) -> Self {
        Self {
            exclude_paths,
            exclude_start_path,
        }
    }
}

impl TokenHandler for TokenService {
    fn parse(&'_ self, token: &str) -> TokenResult<Option<Token>> {
        let t = String::from(token);
        Box::pin(async move {
            // let cache = redis.get::<Option<String>>(t.as_str()).await?;
            // let payload = match cache {
            //     Some(c) => {
            //         if c.is_empty() {
            //             None
            //         } else {
            //             Some(serde_json::from_str::<Token>(c.as_str())?)
            //         }
            //     }
            //     None => None,
            // };
            // Ok(payload)
            Ok(
                Some(
                    Token{
                        account_id: "123".to_string()
                    }
                )
            )
        })
    }

    fn exclude(&'_ self, url: &str) -> TokenResult<bool> {
        let exclude = self.exclude_paths.iter().any(|path| path == url)
            || self
            .exclude_start_path
            .iter()
            .any(|path| url.starts_with(path));
        Box::pin(async move { Ok(exclude) })
    }
}