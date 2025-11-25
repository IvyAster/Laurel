use chrono::{Local, NaiveDateTime};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use laurel_common::datetime_format;
use laurel_common::types::PageQuery;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

pub enum DictType {
    DEFAULT(&'static str), // for d
    CUSTOM(&'static str),  // for c
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::schema::dict)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Dict {
    pub id: i64,
    pub dict_id: String,
    pub dict_name: String,
    pub dict_mark: Option<String>,
    pub weight: i32,
    pub dict_type: String,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schema::dict)]
pub struct InsertableDict<'a> {
    pub dict_id: &'a str,
    pub dict_name: &'a str,
    pub dict_mark: Option<String>,
    pub weight: i32,
    pub dict_type: &'a str,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

impl<'a> From<&'a DictCreateReq> for InsertableDict<'a> {
    fn from(value: &'a DictCreateReq) -> Self {
        InsertableDict {
            dict_id: value.dict_id.as_str(),
            dict_name: value.dict_name.as_str(),
            dict_mark: value.dict_mark.clone(),
            weight: if let Some(w) = value.weight { w } else { 0 },
            dict_type: "custom",
            cts: Local::now().naive_local(),
            uts: Local::now().naive_local(),
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::schema::dict)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatableDict {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_mark: Option<String>,
    pub weight: Option<i32>,
    pub uts: NaiveDateTime,
}

impl From<&DictUpdateReq> for UpdatableDict {
    fn from(value: &DictUpdateReq) -> Self {
        UpdatableDict {
            dict_id: value.dict_id.clone(),
            dict_name: value.dict_name.clone(),
            dict_mark: value.dict_mark.clone(),
            weight: value.weight.clone(),
            uts: Local::now().naive_local(),
        }
    }
}

#[derive(Debug)]
pub struct QueryableDict<'a> {
    pub dict_id: &'a Option<String>,
    pub dict_name: &'a Option<String>,
}

impl<'a> From<&'a DictQueryReq> for QueryableDict<'a> {
    fn from(value: &'a DictQueryReq) -> Self {
        QueryableDict {
            dict_id: &value.dict_id,
            dict_name: &value.dict_name,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct DictQueryReq {
    pub dict_id: Option<String>,
    pub dict_name: Option<String>,

    #[serde(flatten)]
    #[param(inline)]
    pub page: Option<PageQuery>,
}

#[derive(Debug, Default, Serialize, Deserialize, IntoParams, utoipa::ToSchema)]
#[into_params(style = Form, parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct DictCreateReq {
    /// 字典id
    pub dict_id: String,
    /// 字典名称
    pub dict_name: String,
    /// 字典标识
    pub dict_mark: Option<String>,
    /// 字典排序
    pub weight: Option<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize, IntoParams, utoipa::ToSchema)]
#[into_params(style = Form, parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct DictDeleteReq {
    /// 字典自增id
    pub id: i64,
    /// 字典id
    pub dict_id: String,
}

#[derive(Debug, Default, Serialize, Deserialize, IntoParams, utoipa::ToSchema)]
#[into_params(style = Form, parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct DictUpdateReq {
    /// 字典自增id
    pub id: i64,
    /// 字典id
    pub dict_id: Option<String>,
    /// 字典名称
    pub dict_name: Option<String>,
    /// 字典标识
    pub dict_mark: Option<String>,
    /// 字典排序
    pub weight: Option<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DictVo {
    /// 字典自增id
    pub id: i64,
    /// 字典id
    pub dict_id: String,
    /// 字典名称
    pub dict_name: String,
    /// 字典标识
    pub dict_mark: Option<String>,
    /// 字典类型
    pub dict_type: String,
    /// 字典排序
    pub weight: i32,
    /// 创建时间
    pub cts: String,
    /// 更新时间
    pub uts: String,
}

impl From<Dict> for DictVo {
    fn from(value: Dict) -> Self {
        DictVo {
            id: value.id,
            dict_id: value.dict_id,
            dict_name: value.dict_name,
            dict_mark: value.dict_mark,
            dict_type: value.dict_type,
            weight: value.weight,
            cts: datetime_format!(&value.cts),
            uts: datetime_format!(&value.uts),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::schema::dict_value)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DictValue {
    pub id: i64,
    pub dict_id: String,
    pub value_id: String,
    pub value_name: String,
    pub value_mark: Option<String>,
    pub weight: i32,
    pub dict_type: String,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schema::dict_value)]
pub struct InsertableDictValue<'a> {
    pub dict_id: &'a str,
    pub value_id: &'a str,
    pub value_name: &'a str,
    pub value_mark: Option<String>,
    pub weight: i32,
    pub dict_type: &'a str,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

impl<'a> From<&'a DictValueCreateReq> for InsertableDictValue<'a> {
    fn from(value: &'a DictValueCreateReq) -> Self {
        InsertableDictValue {
            dict_id: value.dict_id.as_str(),
            value_id: value.value_id.as_str(),
            value_name: value.value_name.as_str(),
            value_mark: value.value_mark.clone(),
            weight: value.weight,
            dict_type: "custom",
            cts: Local::now().naive_local(),
            uts: Local::now().naive_local(),
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::schema::dict_value)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatableDictValue {
    pub value_name: Option<String>,
    pub value_mark: Option<String>,
    pub weight: Option<i32>,
    pub uts: NaiveDateTime,
}

impl From<&DictValueUpdateReq> for UpdatableDictValue {
    fn from(value: &DictValueUpdateReq) -> Self {
        UpdatableDictValue {
            value_name: value.value_name.clone(),
            value_mark: value.value_mark.clone(),
            weight: value.weight.clone(),
            uts: Local::now().naive_local(),
        }
    }
}

#[derive(Debug)]
pub struct QueryableDictValue<'a> {
    pub dict_id: &'a str,
    pub value_id: &'a Option<String>,
    pub value_name: &'a Option<String>,
}

impl<'a> From<&'a DictValueQueryReq> for QueryableDictValue<'a> {
    fn from(value: &'a DictValueQueryReq) -> Self {
        QueryableDictValue {
            dict_id: &value.dict_id.as_str(),
            value_id: &value.value_id,
            value_name: &value.value_name,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct DictValueQueryReq {
    /// 字典id
    pub dict_id: String,
    /// 字典值id
    pub value_id: Option<String>,
    /// 字典值名称
    pub value_name: Option<String>,
    /// 分页参数
    #[serde(flatten)]
    pub page: Option<PageQuery>,
}

#[derive(Debug, Default, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DictValueCreateReq {
    /// 字典id
    pub dict_id: String,
    /// 字典值id
    pub value_id: String,
    /// 字典值名称
    pub value_name: String,
    /// 字典值标识
    pub value_mark: Option<String>,
    /// 字典排序
    pub weight: i32,
}

#[derive(Debug, Default, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DictValueUpdateReq {
    pub id: i64,
    pub value_id: Option<String>,
    pub value_name: Option<String>,
    pub value_mark: Option<String>,
    pub weight: Option<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DictValueDeleteReq {
    pub id: i64,
    pub value_id: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DictValueVo {
    /// 字典值自增id
    pub id: i64,
    /// 字典id
    pub dict_id: String,
    /// 字典值id
    pub value_id: String,
    /// 字典值名称
    pub value_name: String,
    /// 字典值标识
    pub value_mark: Option<String>,
    /// 字典类型
    pub dict_type: String,
    /// 字典排序
    pub weight: i32,
    /// 创建时间
    pub cts: String,
    /// 更新时间
    pub uts: String,
}

impl From<DictValue> for DictValueVo {
    fn from(value: DictValue) -> Self {
        DictValueVo {
            id: value.id,
            dict_id: value.dict_id,
            value_id: value.value_id,
            value_name: value.value_name,
            value_mark: value.value_mark,
            dict_type: value.dict_type,
            weight: value.weight,
            cts: datetime_format!(&value.cts),
            uts: datetime_format!(&value.uts),
        }
    }
}
