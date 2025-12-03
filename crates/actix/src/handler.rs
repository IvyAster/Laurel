use std::pin::Pin;
use std::sync::Arc;
use crate::types::{common};
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use tracing::error;
use crate::error::AppError;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token{
    pub account_id: String,
}

pub type TokenResult<T> = Pin<Box<dyn Future<Output = anyhow::Result<T, Box<dyn std::error::Error>>> + Send>>;

pub trait TokenHandler{
    fn parse(&self, token: &str) ->  TokenResult<Option<Token>>;

    fn exclude(&self, url: &str) -> TokenResult<bool>;
}

pub async fn validator(req: ServiceRequest, credentials: Option<BearerAuth>) ->Result<ServiceRequest, (Error, ServiceRequest)> {
    let path = req.path();
    let token_service: &web::Data<Arc<dyn TokenHandler>> = req.app_data::<web::Data<Arc<dyn TokenHandler>>>().clone().expect("token service component not fount");
    match token_service.exclude(path).await {
        Ok(exclude) => {
            if exclude{
                return Ok(req);
            }
        },
        Err(err) => {
            error!("token_service.exclude({}) error: {}", path, err);
            return Err((AppError::AuthError(format!("req valid error: {}", path).to_string()).into(), req))
        }
    }

    let token = match &credentials {
        Some(x) => x.token(),
        _ => return Err((AppError::AuthError("invalid token".to_string()).into(), req))
    };
    if token.is_empty(){
        return Err((AppError::AuthError("invalid token".to_string()).into(), req))
    }
    match token_service.parse(token).await{
        Ok(account) => {
            match account {
                Some(account) => {
                    req.extensions_mut().insert(account);
                    Ok(req)
                },
                _ => Err((AppError::AuthError("invalid token {}, payload not exists".to_string()).into(), req))
            }
        },
        Err(err) => {
            error!("parse token: {} error: {}", token, err);
            Err((AppError::AuthError(format!("invalid token: {}", token).to_string()).into(), req))
        }
    }
}


#[macro_export]
macro_rules! error_handler {
    ($req:ident, $srv:ident) => {
        |$req, $srv| {
            let fut = actix_web::dev::Service::call($srv, $req);
                //$srv.call($req);
            async move {
                let res = fut.await?;
                Ok(res)
            }
        }
    };
}

#[macro_export]
macro_rules! default_error_handler {
    ($err:ident, $_req:ident) => {
        (|err, _req| {
            let error_response =
                serde_json::json!(laurel_actix::types::common::ApiResult::<()>::error_message(err.to_string().as_str()));
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::Ok().json(error_response),
            )
            .into()
        })
    };
}

pub async fn default_handler(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound()
        .content_type("application/json")
        .json(serde_json::json!(common::ApiResult::<()>::without_data(
            404,
            format!("{} not found", req.path()).as_str()
        )))
}
