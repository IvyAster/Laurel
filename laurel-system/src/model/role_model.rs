use bon::Builder;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use laurel_common::enum_options;
use laurel_common::types::{HappyEnum, SelectOption};
use serde::{Deserialize, Serialize};

pub enum RoleStatus {
    OPEN(&'static str, &'static str),
    CLOSED(&'static str, &'static str),
    DELETED(&'static str, &'static str),
}

static ROLE_STATES: [RoleStatus; 3] = [
    RoleStatus::OPEN("open", "已开启"),
    RoleStatus::CLOSED("closed", "已关闭"),
    RoleStatus::DELETED("deleted", "已删除"),
];

impl HappyEnum<&'static str> for RoleStatus {
    fn take(&self) -> (&'static str, &'static str) {
        match self {
            RoleStatus::OPEN(x, y) | RoleStatus::CLOSED(x, y) | RoleStatus::DELETED(x, y) => (x, y),
        }
    }

    fn valid(key: &str) -> bool {
        Self::find_self(key).is_some()
    }

    fn find(key: &str) -> Option<&'static str> {
        Self::find_self(key).map(|t| t.take().1).or(None)
    }

    fn find_self(key: &str) -> Option<&'static Self> {
        for item in &ROLE_STATES {
            if let Some(y) = match item {
                &RoleStatus::OPEN(x, _) => {
                    if x == key {
                        Some(item)
                    } else {
                        None
                    }
                }
                &RoleStatus::CLOSED(x, _) => {
                    if x == key {
                        Some(item)
                    } else {
                        None
                    }
                }
                &RoleStatus::DELETED(x, _) => {
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
        enum_options!(ROLE_STATES)
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::schema::role)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i64,
    pub role_id: String,
    pub role_name: String,
    pub role_type: String,
    pub weight: i32,
    pub role_status: String,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::schema::role)]
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct UpdatableRole {
    pub role_name: Option<String>,
    pub role_type: Option<String>,
    pub weight: Option<i32>,
    pub role_status: Option<String>,
    pub uts: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schema::role)]
pub struct InsertableRole<'a> {
    pub role_name: &'a str,
    pub role_type: &'a str,
    pub weight: i32,
    pub role_status: &'a str,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
}

pub struct QueryableRole {}
