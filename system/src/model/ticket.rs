use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::schema::ticket)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ticket{
    pub id: i64,
    pub ticket_id: String,
    pub token: String,
    pub account_id: String,
    pub login_type: String,
    pub ticket_state: String,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
    pub ets: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::schema::ticket)]
pub struct InsertableTicket<'a>{
    pub ticket_id: &'a str,
    pub token: &'a str,
    pub account_id: &'a str,
    pub login_type: &'a str,
    pub ticket_state: &'a str,
    pub cts: NaiveDateTime,
    pub uts: NaiveDateTime,
    pub ets: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtPayload{
    pub ticket_id: String,
}