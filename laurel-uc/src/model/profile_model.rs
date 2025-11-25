use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use laurel_common::datetime_format;
use laurel_uc_api::profile_api::ProfileBo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::schema::profile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    /// 自增id
    pub id: i64,

    pub account_id: String,

    pub profile_key: String,

    pub profile_value: Option<String>,

    /// 创建时间
    pub cts: NaiveDateTime,

    /// 更新时间
    pub uts: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::schema::profile)]
pub struct InsertAbleProfile {
    pub account_id: String,

    pub profile_key: String,

    pub profile_value: Option<String>,

    /// 创建时间
    pub cts: NaiveDateTime,

    /// 更新时间
    pub uts: NaiveDateTime,
}

impl From<Profile> for ProfileBo {
    fn from(value: Profile) -> Self {
        ProfileBo {
            account_id: value.account_id,
            profile_key: value.profile_key,
            profile_value: value.profile_value,
            cts: datetime_format!(value.cts),
            uts: datetime_format!(value.uts),
        }
    }
}
