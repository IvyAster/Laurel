use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use thiserror::Error;
use tracing::log::error;
use crate::types::LR;

// 自定义应用错误枚举
#[derive(Debug, Error)]
pub enum AppError {
    // 用于包装 Actix-web 的内置错误，如 404, 405
    #[error("Server Error: {0}")]
    ActixWeb(#[from] actix_web::Error),

    // 用于包装 JWT 认证错误
    #[error("Authorization Error: {0}")]
    AuthError(String),

    // 其他自定义业务错误
    #[error("Resource Not Found")]
    NotFound,

    #[error("Server Error: ")]
    InternalServerError,

    // 包装 anyhow::Error 的变体
    // 使用 thiserror 的 #[from] 属性自动实现 From<anyhow::Error>
    #[error("An unexpected error occurred: {0}")]
    AnyhowError(#[from] anyhow::Error),
}


// 为 AppError 实现 ResponseError，这是与 Actix-web 集成的关键
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let result = match self {
            AppError::ActixWeb(err) => {
                error!("error: {}", err);
                LR::<()>::error_message(&err.to_string())
            },
            AppError::AuthError(err) => LR::<()>::without_data(401, &err.to_string()),
            AppError::NotFound => LR::<()>::without_data(404, "resource not found"),
            AppError::InternalServerError => LR::<()>::without_data(500, "server error"),
            AppError::AnyhowError(err) => {
                error!("error: {}", err);
                LR::<()>::error_message(&err.to_string())
            },
        };
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .json(result)
    }
}