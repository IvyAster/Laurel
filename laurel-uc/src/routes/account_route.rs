use crate::model::account_model::{AccountLoginVo, LoginVo};
use crate::service::account_service::AccountService;
use actix_web::{get, post, web};
use laurel_actix::handler::Token;
use laurel_actix::types::{Autowired, Done, LR, RequestBody, RequestExtension, RequestParam};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/uc/account").service(login).service(test));
}

#[post("/login")]
async fn login(
    account_service: Autowired<AccountService>,
    req: RequestBody<AccountLoginVo>,
) -> Done<LoginVo> {
    let login_vo = req.into_inner();
    Ok(LR::of(account_service.login(&login_vo).await?.into()))
}

#[get("/test")]
async fn test(token: RequestExtension<Token>) -> Done<Token> {
    Ok(LR::of(Token {
        account_id: token.account_id.clone(),
    }))
}

#[get("/test1")]
async fn test1(account_id: RequestParam<(String,)>) -> Done<String> {
    Ok(LR::of(account_id.0.0))
}
