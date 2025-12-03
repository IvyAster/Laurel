use tracing::log::info;
use laurel_actix::handler::{Token, TokenHandler, TokenResult};
use laurel_actix::types::service;
use laurel_redis::Redis;
use crate::model::ticket::{JwtPayload};

#[derive(Clone, Debug)]
pub struct TokenService {
    redis: Redis,
    exclude_paths: Vec<String>,
    exclude_start_path: Vec<String>,
    secret: String,
}

impl TokenService {
    pub fn new(
        redis: Redis,
        exclude_paths: Vec<String>,
        exclude_start_path: Vec<String>,
        secret: String,
    ) -> Self {
        Self {
            redis,
            exclude_paths,
            exclude_start_path,
            secret,
        }
    }

    pub fn make(&self, ticket_id: &str) -> service::Result<String>{
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &JwtPayload {
                ticket_id: ticket_id.to_string(),
            },
            &jsonwebtoken::EncodingKey::from_secret(self.secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub async fn parse(&self, token: &str) -> service::Result<String>{
        let ticket_id = jsonwebtoken::decode::<JwtPayload>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )?
        .claims
        .ticket_id;
        Ok(ticket_id)
    }
}

impl TokenHandler for TokenService {
    fn parse(&'_ self, token: &str) -> TokenResult<Option<Token>> {
        let redis = self.redis.clone();
        let t = String::from(token);
        Box::pin(async move {
            let cache = redis.get::<Option<String>>(t.as_str()).await?;
            let payload = match cache {
                Some(c) => {
                    if c.is_empty() {
                        None
                    } else {
                        Some(serde_json::from_str::<Token>(c.as_str())?)
                    }
                }
                None => None,
            };
            Ok(payload)
        })
    }

    fn exclude(&'_ self, url: &str) -> TokenResult<bool> {
        info!("exclude: {}", url);
        let exclude = self.exclude_paths.iter().any(|path| path == url)
            || self
            .exclude_start_path
            .iter()
            .any(|path| url.starts_with(path));
        Box::pin(async move { Ok(exclude) })
    }
}
