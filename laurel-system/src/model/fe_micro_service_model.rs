use bon::Builder;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use laurel_common::types::{HappyEnum, IndexAble, PageQuery, SelectOption};
use laurel_common::{datetime_format, enum_options};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::schema::fe_micro_service)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FeMicroService {
    /// 自增id
    pub id: i64,
    pub app_id: String,
    pub service_id: String,
    pub service_name: String,
    pub service_entry: String,
    pub mount_point: String,
    pub route_pattern: String,
    pub service_status: String,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::schema::fe_micro_service)]
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct UpdatableFeMicroService {
    pub service_name: Option<String>,
    pub service_entry: Option<String>,
    pub mount_point: Option<String>,
    pub route_pattern: Option<String>,
    pub service_status: Option<String>,
    pub uts: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::schema::fe_micro_service)]
pub struct InsertAbleFeMicroService<'a> {
    pub app_id: &'a str,
    pub service_id: &'a str,
    pub service_name: &'a str,
    pub service_entry: &'a str,
    pub mount_point: &'a str,
    pub route_pattern: &'a str,
    pub service_status: &'a str,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

#[derive(Debug)]
pub enum MicroServiceStatus {
    OPEN(&'static str, &'static str),
    CLOSED(&'static str, &'static str),
    DELETED(&'static str, &'static str),
}

static MICRO_SERVICE_STATES: [MicroServiceStatus; 3] = [
    MicroServiceStatus::OPEN("open", "已开启"),
    MicroServiceStatus::CLOSED("closed", "已关闭"),
    MicroServiceStatus::DELETED("deleted", "已删除"),
];

impl HappyEnum<&'static str> for MicroServiceStatus {
    fn take(&self) -> (&'static str, &'static str) {
        match self {
            MicroServiceStatus::OPEN(x, y)
            | MicroServiceStatus::CLOSED(x, y)
            | MicroServiceStatus::DELETED(x, y) => (x, y),
        }
    }

    fn valid(key: &str) -> bool {
        MicroServiceStatus::find_self(key).is_some()
    }

    fn find(key: &str) -> Option<&'static str> {
        MicroServiceStatus::find_self(key)
            .map(|t| t.take().1)
            .or(None)
    }

    fn find_self(key: &str) -> Option<&'static MicroServiceStatus> {
        for item in &MICRO_SERVICE_STATES {
            if let Some(y) = match item {
                &MicroServiceStatus::OPEN(x, _) => {
                    if x == key {
                        Some(item)
                    } else {
                        None
                    }
                }
                &MicroServiceStatus::CLOSED(x, _) => {
                    if x == key {
                        Some(item)
                    } else {
                        None
                    }
                }
                &MicroServiceStatus::DELETED(x, _) => {
                    if x == key {
                        Some(item)
                    } else {
                        None
                    }
                }
            } {
                return Some(y);
            }
        }
        None
    }

    fn options() -> Vec<SelectOption<&'static str, &'static str>> {
        enum_options!(MICRO_SERVICE_STATES)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeMicroServiceVo {
    /// 自增id
    pub index: u32,
    pub app_id: String,
    pub service_id: String,
    pub service_name: String,
    pub service_entry: String,
    pub mount_point: String,
    pub route_pattern: String,
    pub service_status: String,
    pub service_status_name: Option<&'static str>,
    cts: String,
    uts: String,
}

impl IndexAble for FeMicroServiceVo {
    fn set_index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }
}

impl From<FeMicroService> for FeMicroServiceVo {
    fn from(value: FeMicroService) -> Self {
        Self {
            index: 0,
            app_id: value.app_id,
            service_id: value.service_id,
            service_name: value.service_name,
            service_entry: value.service_entry,
            mount_point: value.mount_point,
            service_status_name: MicroServiceStatus::find(&value.service_status),
            service_status: value.service_status,
            cts: datetime_format!(value.cts),
            uts: datetime_format!(value.uts),
            route_pattern: value.route_pattern,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeMicroServiceQuery {
    pub app_id: String,

    pub service_name: Option<String>,

    pub service_status: Option<String>,

    #[serde(flatten)]
    pub page: Option<PageQuery>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeMicroServiceCreateReq {
    pub app_id: String,
    pub service_name: String,
    pub service_entry: String,
    pub mount_point: String,
    pub route_pattern: String,
    pub service_status: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MicroServiceUpdateReq {
    pub service_id: String,
    pub service_name: Option<String>,
    pub service_entry: Option<String>,
    pub mount_point: Option<String>,
    pub route_pattern: Option<String>,
    pub service_status: Option<String>,
}

#[test]
fn test_find() {
    println!("{:?}", MicroServiceStatus::find_self(&"open".to_string()))
}
