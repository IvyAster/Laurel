use actix_web::{web};



pub mod route{
    pub use crate::error::AppError;
    pub type Result<T> = core::result::Result<crate::types::common::ApiResult<T>, AppError>;

    #[allow(non_snake_case)]
    #[macro_export]
    macro_rules! Data {
        ($value: expr) => {
            Ok(
                laurel_actix::types::common::ApiResult::of(
                    $value
                )
            )
        };
    }
}

pub mod service{
    pub type Result<T> = core::result::Result<T, anyhow::Error>;
}

pub mod repository{
    pub type Result<T> = core::result::Result<T, anyhow::Error>;
}

pub mod common{
    use actix_web::{HttpRequest, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};

    pub trait IntoOption<T>{
        fn into_option(self) -> Option<T>;
    }

    impl <T> IntoOption<T> for Option<T>{
        fn into_option(self) -> Option<T> {
            self
        }
    }

    impl<T> IntoOption<T> for T {
        fn into_option(self) -> Option<T> {
            Some(self)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApiResult<T>{
        /// 业务状态码
        pub code: u16,
        /// 响应消息
        pub message: String,
        /// 响应数据
        pub data: Option<T>,
    }

    static SUCCESS: &str = "success";
    static ERROR: &str = "error";

    static SUCCESS_CODE: u16 = 200;
    static ERROR_CODE: u16 = 500;

    impl <T> ApiResult<T> {
        pub fn of<U>(data: U) -> Self
        where
            U: IntoOption<T>, // 约束入参必须能转为 Option<T>
        {
            ApiResult {
                code: SUCCESS_CODE,
                message: SUCCESS.to_string(),
                data: data.into_option(),
            }
        }

        pub fn new(code: u16, message: &str, data: Option<T>) -> Self {
            Self {code, message: message.to_string(), data}
        }

        pub fn error_code(code: u16) -> Self {
            Self{code, message: ERROR.to_string(), data: None}
        }

        pub fn error_message(message: &str) -> Self {
            Self{code: ERROR_CODE, message: message.to_string(), data: None}
        }

        pub fn without_data(code: u16, message: &str) -> Self {
            Self{code, message: message.to_string(), data: None}
        }

        pub fn default() -> Self {
            Self {code: SUCCESS_CODE, message: SUCCESS.to_string(), data: None}
        }

        pub fn is_successful_with_code(&self, code: u16) -> bool {
            self.code == code
        }

        pub fn is_successful(&self) -> bool {
            self.is_successful_with_code(SUCCESS_CODE)
        }
    }

    impl <T: Serialize> Responder for ApiResult<T>{
        type Body = actix_web::body::BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            HttpResponse::Ok().json(self)
        }
    }
}

// on way data processing
//pub type Running<T, E = Error> =  anyhow::Result<T, E>;

// ending data processing
//pub type Done<T, E = AppError> = Result<LR<T>, E>;

//用于从应用程序状态中共享数据。通常用于在多个处理器之间共享数据库连接池、配置等。
//T 必须是实现了 Send + Sync 的类型，因为多个线程可能同时访问它。
//例如：数据库连接池、全局配置、缓存客户端等。
pub type Autowired<T> = web::Data<T>;
//pub type WebData<T> = web::Data<T>;

//用于从请求的查询字符串（URL 中的 ? 后面的部分）中提取数据。
//T 通常是一个实现了 serde::Deserialize 的结构体，用于反序列化查询参数。
//例如：分页参数、过滤条件等。
//pub type WebQuery<T> = web::Query<T>;
pub type RequestParam<T> = web::Query<T>;

//用于从请求的路径中提取动态片段（即路由中的变量）。
//T 可以是一个元组结构体或一个实现了 serde::Deserialize 的结构体，用于反序列化路径参数。
//例如：在路由 /users/{id} 中，可以提取 id。
//pub type WebPath<T> = web::Path<T>;
pub type RequestPath<T> = web::Path<T>;

//用于从请求的 body 中提取 JSON 数据，并反序列化为类型 T。
//T 必须实现 serde::Deserialize。
//例如：创建或更新资源时提交的 JSON 数据。
//pub type WebJson<T> = web::Json<T>;
pub type RequestBody<T> = web::Json<T>;

//用于从 application/x-www-form-urlencoded 格式的请求体中提取数据。通常用于处理 HTML 表单提交。
pub type RequestForm<T> = web::Form<T>;

//用于从请求的扩展（request extensions）中提取数据。这些数据通常是由中间件放入的。
//例如：用户认证后，中间件可以将用户信息放入请求扩展，然后在处理器中通过 WebReqData 提取。
//pub type WebReqData<T> = web::ReqData<T>;
pub type RequestExtension<T> = web::ReqData<T>;

//用于从 application/x-www-form-urlencoded 格式的请求体中提取数据。通常用于处理 HTML 表单提交。
pub type WebForm<T> = web::Form<T>;