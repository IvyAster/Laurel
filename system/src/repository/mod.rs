pub mod dict;
pub mod fe_micro_service;
pub mod menu;
pub mod role;

pub mod account;
pub mod passport;
pub mod profile;
pub mod ticket;

// use diesel::QueryableByName;
// #[allow(unused)]
// pub use diesel::RunQueryDsl as SyncDsl;
// pub use diesel_async::RunQueryDsl as AsyncDsl;
//
// #[derive(Debug, QueryableByName)]
// pub struct StringRow {
//     #[diesel(sql_type = diesel::sql_types::VarChar)]
//     pub row_result: String,
// }
//
// #[derive(Debug, QueryableByName)]
// pub struct IntRow {
//     #[diesel(sql_type = diesel::sql_types::BigInt)]
//     pub row_result: i64,
// }
