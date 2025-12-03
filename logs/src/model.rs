pub mod login_log {
    use chrono::{Local, NaiveDateTime};
    use diesel::{Identifiable, Insertable, Queryable, Selectable};
    use laurel_common::date_time;
    use laurel_common::types::{IndexAble, PageQuery};
    use laurel_logs_api::logs::LoginLogCreateReqBo;
    use serde::{Deserialize, Serialize};
    use tracing::error;

    #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
    #[diesel(table_name = crate::schema::login_log)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct LoginLog {
        pub id: i64,
        pub ticket_id: String,
        pub account: String,
        pub login_type: String,
        pub login_state: String,
        pub login_result: Option<String>,
        pub ip: Option<String>,
        pub location: Option<String>,
        pub browser: Option<String>,
        pub os: Option<String>,
        pub device: Option<String>,
        pub cts: NaiveDateTime,
        pub login_cts: NaiveDateTime,
    }

    #[derive(Debug, Insertable)]
    #[diesel(table_name = crate::schema::login_log)]
    pub struct InsertableLoginLog<'a> {
        pub ticket_id: &'a str,
        pub account: &'a str,
        pub login_type: &'a str,
        pub login_state: &'a str,
        pub login_result: Option<String>,
        pub ip: Option<String>,
        pub location: Option<String>,
        pub browser: Option<String>,
        pub os: Option<String>,
        pub device: Option<String>,
        pub cts: NaiveDateTime,
        pub login_cts: NaiveDateTime,
    }

    impl<'a> From<&'a LoginLogCreateReqBo> for InsertableLoginLog<'a> {
        fn from(bo: &'a LoginLogCreateReqBo) -> Self {
            InsertableLoginLog {
                ticket_id: bo.ticket_id.as_str(),
                account: bo.account.as_str(),
                login_type: bo.login_type.as_str(),
                login_state: bo.login_state.as_str(),
                login_result: bo.login_result.clone(),
                ip: bo.ip.clone(),
                location: bo.location.clone(),
                browser: bo.browser.clone(),
                os: bo.os.clone(),
                device: bo.device.clone(),
                cts: Local::now().naive_local(),
                login_cts: match NaiveDateTime::parse_from_str(
                    bo.login_cts.as_str(),
                    date_time::DTF,
                ) {
                    Ok(cts) => cts,
                    Err(err) => {
                        error!(
                            "Invalid date time for LoginLogCreateReqBo({:?}.login_cts, %Y-%m-%d %H:%M:%S): {}",
                            bo, err
                        );
                        Local::now().naive_local()
                    }
                },
            }
        }
    }

    #[derive(Debug)]
    pub struct QueryableLoginLog<'a> {
        pub account: &'a Option<String>,
        pub ip: &'a Option<String>,
        pub login_state: &'a Option<String>,
        pub login_cts_start: Option<NaiveDateTime>,
        pub login_cts_end: Option<NaiveDateTime>,
    }

    impl<'a> From<&'a LoginLogQueryReq> for QueryableLoginLog<'a> {
        fn from(req: &'a LoginLogQueryReq) -> Self {
            QueryableLoginLog {
                account: &req.account,
                ip: &req.ip,
                login_state: &req.login_state,
                login_cts_start: match &req.login_cts_start {
                    Some(cs) => Some(
                        NaiveDateTime::parse_from_str(cs.as_str(), date_time::DTF).expect("Invalid date time for LoginLogQueryReq(login_cts_start, %Y-%m-%d %H:%M:%S)")
                    ),
                    None => None,
                },
                login_cts_end: match &req.login_cts_end {
                    Some(cs) => Some(
                        NaiveDateTime::parse_from_str(cs.as_str(), date_time::DTF).expect("Invalid date time for LoginLogQueryReq(login_cts_end, %Y-%m-%d %H:%M:%S)")
                    ),
                    None => None,
                },
            }
        }
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginLogVo {
        pub index: u32,
        pub ticket_id: String,
        pub account: String,
        pub login_type: String,
        pub login_state: String,
        pub login_result: Option<String>,
        pub ip: Option<String>,
        pub location: Option<String>,
        pub browser: Option<String>,
        pub os: Option<String>,
        pub device: Option<String>,
        pub cts: String,
        pub login_cts: String,
    }

    impl IndexAble for LoginLogVo {
        fn set_index(&mut self, index: u32) -> &mut Self {
            self.index = index;
            self
        }
    }

    impl From<LoginLog> for LoginLogVo {
        fn from(value: LoginLog) -> Self {
            LoginLogVo {
                index: 0u32,
                ticket_id: value.ticket_id,
                account: value.account,
                login_type: value.login_type,
                login_state: value.login_state,
                login_result: value.login_result,
                ip: value.ip,
                location: value.location,
                browser: value.browser,
                os: value.os,
                device: value.device,
                cts: value.cts.format(date_time::DTF).to_string(),
                login_cts: value.login_cts.format(date_time::DTF).to_string(),
            }
        }
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginLogQueryReq {
        pub account: Option<String>,
        pub ip: Option<String>,
        pub login_state: Option<String>,
        pub login_cts_start: Option<String>,
        pub login_cts_end: Option<String>,
        #[serde(flatten)]
        pub page: Option<PageQuery>,
    }
}
