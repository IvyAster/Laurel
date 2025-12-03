use crate::model::account::{AccountLoginVo, LoginVo};
use crate::service::account::AccountService;
use actix_web::{HttpRequest, get, post, web};
use tracing::error;
use laurel_actix::Data;
use laurel_actix::handler::Token;
use laurel_actix::types::{Autowired, RequestBody, RequestExtension, RequestParam, route};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/system/account").service(login).service(test));
}

#[post("/login")]
async fn login(
    _req : HttpRequest,
    account_service: Autowired<AccountService>,
    req: RequestBody<AccountLoginVo>,
) -> route::Result<LoginVo> {
    let ua = match _req.headers().get(actix_web::http::header::USER_AGENT){
        Some(ua) => {
            match ua.to_str(){
                Ok(ua) => Some(ua),
                Err(err) => {
                    error!("解析UA: {:?} 失败: {}", ua, err);
                    None
                },
            }
        },
        None => None,
    };
    let ip = laurel_actix::utils::ip(&_req);
    let login_vo = req.into_inner();
    Data!(
        LoginVo::from(account_service.login(&login_vo, ua, ip).await?)
    )
}

#[get("/test")]
async fn test(token: RequestExtension<Token>) -> route::Result<Token> {
    Data!(
        Token {
            account_id: token.account_id.clone(),
        }
    )
}

#[get("/test1")]
async fn test1(account_id: RequestParam<(String,)>) -> route::Result<String> {
    Data!(account_id.0.0)
}
