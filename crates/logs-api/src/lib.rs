
pub mod logs{
    use std::sync::Arc;
    use reqwest_middleware::ClientWithMiddleware;
    use serde::{Deserialize, Serialize};
    use laurel_common::types::{api};

    #[derive(Clone, Debug)]
    pub struct LogApi(laurel_middleware::request::Client);
    static LOGIN_LOG_URI: &'static str = "/interface/logs/login/create";

    impl LogApi{
        pub fn build(client: Arc<ClientWithMiddleware>, host: String, path: Option<String>) -> Self{
            Self(laurel_middleware::request::Client::new(client, host, path))
        }

        pub fn new(client: laurel_middleware::request::Client)->Self{
            Self( client)
        }


        pub async fn save_login_log(&self, req: &LoginLogCreateReqBo) -> api::Result<i64>{
            let url = self.0.url(LOGIN_LOG_URI);
            let resp = self.0.client()
                .post(url)
                .json(req)
                .send()
                .await?
                .json::<api::ApiResult<i64>>()
                .await?;
            Ok(resp)
        }
    }








    #[derive(Deserialize, Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginLogCreateReqBo{
        pub ticket_id: String,
        pub account: String,
        pub login_type: String,
        pub login_state: String,
        pub login_result: Option<String>,
        pub ip: Option<String>,
        pub location: Option<String>,
        pub browser: Option<String>,
        pub os: Option<String>,
        pub device: Option<String>,
        // yyyy-MM-dd HH:mm:ss
        pub login_cts: String,
    }


}