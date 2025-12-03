pub mod login_log {
    use crate::model::login_log::{
        InsertableLoginLog, LoginLog, LoginLogQueryReq, QueryableLoginLog,
    };
    use crate::repository::login_log;
    use laurel_actix::types::service;
    use laurel_common::types::Pagination;
    use laurel_logs_api::logs::LoginLogCreateReqBo;
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    pub struct Service {
        repository: Arc<login_log::Repository>,
    }

    impl Service {
        pub fn new(repository: Arc<login_log::Repository>) -> Self {
            Self { repository }
        }

        pub async fn create(&self, req: &LoginLogCreateReqBo) -> service::Result<i64> {
            let insertable = InsertableLoginLog::from(req);
            Ok(self.repository.save(&insertable).await?)
        }

        pub async fn page(
            &self,
            req: &LoginLogQueryReq,
            (page, size): (u32, u32),
        ) -> service::Result<Pagination<LoginLog>> {
            let queryable = QueryableLoginLog::from(req);
            Ok(self.repository.page(&queryable, (page, size)).await?)
        }
    }
}


pub mod token{
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

    #[allow(deprecated)]
    impl TokenHandler for TokenService {
        fn parse(&'_ self, token: &str) -> TokenResult<Option<Token>> {
            let t = String::from(token);
            Box::pin(async move {
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
}