use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};


#[allow(dead_code, unused)]
pub mod api{
    use serde::{Deserialize, Serialize};

    pub type Result<T> = core::result::Result<ApiResult<T>, anyhow::Error>;
    static SUCCESS: &str = "success";
    static ERROR: &str = "error";

    static SUCCESS_CODE: u16 = 200;
    static ERROR_CODE: u16 = 500;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApiResult<T>{
        /// 业务状态码
        pub code: u16,
        /// 响应消息
        pub message: String,
        /// 响应数据
        pub data: Option<T>,
    }

    impl <T> From<anyhow::Error> for ApiResult<T> {
        fn from(value: anyhow::Error) -> Self {
            ApiResult{
                code: ERROR_CODE,
                message: value.to_string(),
                data: None,
            }
        }
    }

    impl <T> ApiResult<T>{
        pub fn is_successful_with_code(&self, code: u16) -> bool {
            self.code == code
        }

        pub fn is_successful(&self) -> bool {
            self.is_successful_with_code(SUCCESS_CODE)
        }

        pub fn of(code: u16, message: String, data: Option<T>) -> Self{
            Self{code, message, data}
        }
    }

    pub trait ApiUrl{
        fn url(&self, uri: &str);
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all="camelCase")]
pub struct Pagination<T> {
    pub page: u32,
    pub size: u32,
    pub pages: u64,
    pub total: u64,
    pub data: Option<Vec<T>>,
}

impl <T> Pagination<T>{

    pub fn to_with_index<R>(self) -> Pagination<R>
    where R: IndexAble + From<T>
    {
        let index = (self.page - 1) * self.size + 1;
        let data = match self.data {
            Some(data) => data.into_iter().map(|item| {
                let mut r : R= item.into();
                r.set_index(index);
                r
            }).collect(),
            None => Vec::new()
        };
        Pagination{
            page: self.page,
            size: self.size,
            pages: self.pages,
            total: self.total,
            data: Some(data),
        }
    }

    pub fn to<R>(self) -> Pagination<R>
    where R: From<T>{
        Pagination{
            page: self.page,
            size: self.size,
            pages: self.pages,
            total: self.total,
            data: match self.data {
                Some(data) => Some(data.into_iter().map(|item| {
                    item.into()
                }).collect()),
                None => Some(vec![])
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
#[into_params(style = Form, parameter_in = Query)]
#[serde(rename_all="camelCase")]
pub struct PageQuery{
    pub page: u32,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectOption<K, V>{
    pub key: K,
    pub value: V,
}

#[macro_export]
macro_rules! enum_options {
    ($ident: ident) => {
        (&$ident).iter().map(|item| {
            SelectOption{
                key: item.take().1,
                value: item.take().0,
            }
        }).collect()
    };
}


pub trait HappyEnum<T>{
    fn take(&self) -> (&'static str, &'static str);

    fn valid(key: &str) -> bool;

    fn find(key: &str) -> Option<&'static str>;

    fn find_self(key: &str)-> Option<&'static Self>;

    fn options() -> Vec<SelectOption<&'static str,&'static str>>;
}

pub trait IndexAble {
    fn set_index(&mut self, index: u32) -> &mut Self;
}

