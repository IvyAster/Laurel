pub mod login_log {
    use crate::model::login_log::{InsertableLoginLog, LoginLog, QueryableLoginLog};
    use crate::schema::login_log as LoginLogSchema;
    use crate::schema::login_log::dsl as LoginLogDsl;
    use diesel::QueryDsl;
    use diesel::pg::Pg;
    use diesel::{ExpressionMethods, PgTextExpressionMethods, SelectableHelper};
    use laurel_actix::types::repository;
    use laurel_common::types::Pagination;
    use laurel_pg::{AsyncDsl, DbPool};

    #[derive(Clone, Debug)]
    pub struct Repository {
        pool: DbPool,
    }
    impl Repository {
        pub fn new(pool: DbPool) -> Self {
            Self { pool }
        }

        pub async fn save<'a>(
            &self,
            insertable: &InsertableLoginLog<'a>,
        ) -> repository::Result<i64> {
            let mut conn = self.pool.get().await?;
            let id = AsyncDsl::get_result(
                diesel::insert_into(LoginLogDsl::login_log)
                    .values(insertable)
                    .returning(LoginLogDsl::id),
                &mut conn,
            )
            .await?;
            Ok(id)
        }

        pub async fn page<'a>(
            &self,
            queryable: &'a QueryableLoginLog<'a>,
            (page, size): (u32, u32),
        ) -> repository::Result<Pagination<LoginLog>> {
            let mut conn = self.pool.get().await?;
            let total = AsyncDsl::get_result::<i64>(
                self.apply_filters(queryable, LoginLogDsl::login_log.into_boxed())
                    .select(diesel::dsl::count_star()),
                &mut conn,
            )
            .await?;
            let offset = (page - 1) * size;
            if total <= 0 {
                return Ok(Pagination {
                    page,
                    size,
                    pages: 0,
                    total: 0,
                    data: Some(vec![]),
                });
            }
            let pages = (total as f64 / size as f64).ceil() as u64;
            let list = AsyncDsl::load(
                self.apply_filters(queryable, LoginLogDsl::login_log.into_boxed())
                    .order_by(LoginLogDsl::id.desc())
                    .offset(offset as i64)
                    .limit(size as i64)
                    .select(LoginLog::as_returning()),
                &mut conn,
            )
            .await?;
            Ok(Pagination {
                page,
                size,
                pages,
                total: total as u64,
                data: Some(list),
            })
        }

        fn apply_filters<'a>(
            &self,
            queryable: &QueryableLoginLog<'a>,
            mut query: LoginLogSchema::BoxedQuery<'a, Pg>,
        ) -> LoginLogSchema::BoxedQuery<'a, Pg> {
            if let Some(param) = queryable.account
                && !param.is_empty()
            {
                query = query.filter(LoginLogDsl::account.ilike(format!("%{}%", param)))
            }
            if let Some(param) = queryable.ip
                && !param.is_empty()
            {
                query = query.filter(LoginLogDsl::ip.ilike(format!("%{}%", param)))
            }
            if let Some(param) = queryable.login_state
                && !param.is_empty()
            {
                query = query.filter(LoginLogDsl::login_state.eq(param.as_str()))
            }

            if let (Some(cs), Some(ce)) = (queryable.login_cts_start, queryable.login_cts_end) {
                query = query.filter(LoginLogDsl::login_cts.between(cs, ce))
            } else {
                if let Some(cs) = queryable.login_cts_start {
                    query = query.filter(LoginLogDsl::login_cts.ge(cs))
                }
                if let Some(ce) = queryable.login_cts_end {
                    query = query.filter(LoginLogDsl::login_cts.le(ce))
                }
            }
            query
        }
    }
}
