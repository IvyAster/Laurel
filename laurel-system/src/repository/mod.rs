pub mod dict_repository;
pub mod fe_micro_service_repository;
pub mod menu_repository;
pub mod role_repository;

use diesel::QueryableByName;
#[allow(unused)]
pub use diesel::RunQueryDsl as SyncDsl;
pub use diesel_async::RunQueryDsl as AsyncDsl;

#[derive(Debug, QueryableByName)]
pub struct StringRow {
    #[diesel(sql_type = diesel::sql_types::VarChar)]
    pub row_result: String,
}

#[derive(Debug, QueryableByName)]
pub struct IntRow {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub row_result: i64,
}
