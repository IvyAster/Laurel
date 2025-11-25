use std::sync::Arc;
use anyhow::{anyhow, Error};
use bon::Builder;
use laurel_actix::handler::{Token, TokenHandler, TokenResult};
use laurel_uc_api::account_api::{AccountApi};

#[derive(Clone, Debug, Builder)]
pub struct TokenService {
    account_api: Arc<AccountApi>,
    exclude_paths: Vec<String>,
    exclude_start_path: Vec<String>,
}

impl TokenService {
    pub fn new(
        account_api: Arc<AccountApi>,
        exclude_paths: Vec<String>,
        exclude_start_path: Vec<String>,
    )-> Self{
        Self{
            account_api,
            exclude_paths,
            exclude_start_path,
        }
    }
}


impl TokenHandler for TokenService {
    fn parse(&self, token: &str) -> TokenResult<Option<Token>> {
        let token_string = token.to_string();
        let client = Arc::clone(&self.account_api);
        Box::pin(async move {
            let result = client.parse_token(token_string.as_str()).await?;
            match (result.is_successful(), &result.data){
                (true, Some(data)) => Ok(Some(Token { account_id: data.account_id.clone() })),
                (true, None) => Err(Box::from(Error::msg("token parse error: empty data"))),
                _ => Err(Box::from(anyhow!(result.message.clone())))
            }
        })
    }

    #[allow(deprecated)]
    fn exclude(&self, url: &str) -> TokenResult<bool> {
        let exclude = self.exclude_paths.iter().any(|path| path == url) ||
            self.exclude_start_path.iter().any(|path| url.starts_with(path));
        Box::pin(async move {
            Ok(exclude)
        })
    }
}
