use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::schema::passport)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PassportEntity {
    /// 自增id
    pub id: i64,

    pub account_id: String,

    pub salt: String,

    pub password: String,

    /// 创建时间
    pub cts: NaiveDateTime,

    /// 更新时间
    pub uts: NaiveDateTime,
}
