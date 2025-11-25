use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Selectable};
use laurel_uc_api::account_api::AccountBo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountEntity {
    /// 自增id
    pub id: i64,

    pub account_id: String,

    pub account_name: String,

    pub account_state: String,

    pub account_type: String,

    /// 创建时间
    pub cts: NaiveDateTime,

    /// 更新时间
    pub uts: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountLoginVo {
    pub account: String,

    pub password: String,

    pub device: Option<String>,

    pub host: Option<String>,

    pub endpoint: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountVo {
    pub account_id: String,

    pub account_name: String,

    pub account_state: String,

    pub account_type: String,

    /// 创建时间
    pub cts: String,

    /// 更新时间
    pub uts: String,
}

impl From<AccountEntity> for AccountVo {
    fn from(entity: AccountEntity) -> Self {
        let ts_formatter = "%Y-%m-%d %H:%M:%S";
        AccountVo {
            account_id: entity.account_id,
            account_name: entity.account_name,
            account_state: entity.account_state,
            account_type: entity.account_type,
            cts: entity.cts.format(ts_formatter).to_string(),
            uts: entity.uts.format(ts_formatter).to_string(),
        }
    }
}

// #[derive(Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AccountBo{
//     pub account_id: String,
//
//     pub account_name: String,
//
//     pub account_state: String,
//
//     pub account_type: String,
//
//     /// 创建时间
//     pub cts: String,
//
//     /// 更新时间
//     pub uts: String,
// }

impl From<AccountEntity> for AccountBo {
    fn from(entity: AccountEntity) -> Self {
        let ts_formatter = "%Y-%m-%d %H:%M:%S";
        AccountBo {
            account_id: entity.account_id,
            account_name: entity.account_name,
            account_state: entity.account_state,
            account_type: entity.account_type,
            cts: entity.cts.format(ts_formatter).to_string(),
            uts: entity.uts.format(ts_formatter).to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginVo {
    pub account: AccountVo,
    pub token: String,
}

impl From<(AccountEntity, String)> for LoginVo {
    fn from(value: (AccountEntity, String)) -> Self {
        LoginVo {
            account: value.0.into(),
            token: value.1,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountQuery {
    pub account_id: Option<String>,

    pub account_name: Option<String>,

    pub account_type: Option<String>,
}
