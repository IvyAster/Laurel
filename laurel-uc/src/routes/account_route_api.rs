use crate::model::account_model::AccountQuery;
use crate::service::account_service::AccountService;
use actix_web::{get, web};
use laurel_uc_api::account_api::{AccountBo, TokenParseQuery, TokenPayloadBo};
use tracing::error;
use laurel_actix::error::AppError;
use laurel_actix::handler::TokenHandler;
use laurel_actix::types::{Autowired, Done, RequestParam, LR};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/interface/uc/account")
            .service(find_account)
            .service(parse_token),
    );
}

#[get("")]
async fn find_account(
    account_service: Autowired<AccountService>,
    query: RequestParam<AccountQuery>,
) -> Done<AccountBo> {
    let account = if let Some(id) = &query.account_id {
        account_service.find_account_by_id(id.as_str()).await?
    } else if let (Some(account_name), Some(account_type)) =
        (&query.account_name, &query.account_type)
    {
        account_service
            .find_account_by_account(account_name.as_str(), account_type.as_str())
            .await?
    } else {
        None
    };
    Ok(LR::of_raw(
        account.map(|e| e.into()).or(None),
    ))
}

#[get("/token")]
async fn parse_token(
    token_service: Autowired<Box<dyn TokenHandler>>,
    token: RequestParam<TokenParseQuery>,
) -> Done<TokenPayloadBo> {
    match token_service.parse(token.token.as_str()).await {
        Ok(t) => match t {
            Some(t) => Ok(LR::of(TokenPayloadBo {
                account_id: t.account_id,
            })),
            _ => Ok(LR::of_raw(None)),
        },
        Err(err) => {
            error!("parse token: {} error: {}", token.token, err);
            Err(AppError::AuthError(format!("invalid token: {}", token.token).to_string()).into())
        }
    }
}
