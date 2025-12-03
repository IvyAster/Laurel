use crate::model::fe_micro_service as FeMicroServiceModel;
use crate::model::fe_micro_service::FeMicroService;
use crate::schema::schema::fe_micro_service as FeMicroServiceSchema;
use crate::schema::schema::fe_micro_service::dsl as MicroServiceDsl;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel_async::*;
use laurel_actix::types::repository;
use laurel_common::types::Pagination;
use laurel_pg::{AsyncDsl, DbPool};

#[derive(Clone, Debug)]
pub struct FeMicroServiceRepository {
    pool: DbPool,
}

impl FeMicroServiceRepository {
    pub fn new(pool: DbPool) -> Self {
        FeMicroServiceRepository { pool }
    }

    fn apply_filters<'a>(
        &self,
        params: &'a FeMicroServiceModel::FeMicroServiceQuery,
        mut query: FeMicroServiceSchema::BoxedQuery<'a, diesel::pg::Pg>,
    ) -> FeMicroServiceSchema::BoxedQuery<'a, diesel::pg::Pg> {
        query = query.filter(FeMicroServiceSchema::dsl::app_id.eq(params.app_id.as_str()));
        if let Some(name) = &params.service_name
            && !name.is_empty()
        {
            query = query.filter(
                FeMicroServiceSchema::dsl::service_name.ilike(format!("%{}%", name.as_str())),
            );
        }
        if let Some(state) = &params.service_status
            && !state.is_empty()
        {
            query = query.filter(FeMicroServiceSchema::dsl::service_status.eq(state.as_str()));
        }
        query
    }

    pub async fn find_service(&self, service_id: &str) -> repository::Result<Option<FeMicroService>> {
        let mut conn = self.pool.get().await?;
        let service = AsyncDsl::first(
            MicroServiceDsl::fe_micro_service::table()
                .filter(MicroServiceDsl::service_id.eq(service_id))
                .select(FeMicroService::as_select()),
            &mut conn,
        )
        .await
        .optional()?;
        Ok(service)
    }

    pub async fn list_services(
        &self,
        app_id: &str,
    ) -> repository::Result<Vec<FeMicroServiceModel::FeMicroService>> {
        let mut conn = self.pool.get().await?;
        let apps = AsyncDsl::load(
            FeMicroServiceSchema::dsl::fe_micro_service
                .filter(FeMicroServiceSchema::dsl::app_id.eq(app_id))
                .order_by(FeMicroServiceSchema::dsl::id.desc())
                .select(FeMicroServiceModel::FeMicroService::as_select()),
            &mut conn,
        )
        .await?;
        Ok(apps)
    }

    pub async fn list_services_with_state(
        &self,
        app_id: &str,
        service_status: &str,
    ) -> repository::Result<Vec<FeMicroServiceModel::FeMicroService>> {
        let mut conn = self.pool.get().await?;
        let apps = AsyncDsl::load(
            FeMicroServiceSchema::dsl::fe_micro_service
                .filter(FeMicroServiceSchema::dsl::app_id.eq(app_id))
                .filter(FeMicroServiceSchema::dsl::service_status.eq(service_status))
                .order_by(FeMicroServiceSchema::dsl::id.desc())
                .select(FeMicroServiceModel::FeMicroService::as_select()),
            &mut conn,
        )
        .await?;
        Ok(apps)
    }

    pub async fn page_micro_apps(
        &self,
        query: &FeMicroServiceModel::FeMicroServiceQuery,
        page: u32,
        size: u32,
    ) -> repository::Result<Pagination<FeMicroServiceModel::FeMicroService>> {
        let mut conn = self.pool.get().await?;
        let total = AsyncDsl::get_result::<i64>(
            self.apply_filters(
                query,
                FeMicroServiceSchema::dsl::fe_micro_service.into_boxed(),
            )
            .select(diesel::dsl::count_star()),
            &mut conn,
        )
        .await?;
        //let page = query.page.page;
        //let size = query.page.size;
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
        let apps = AsyncDsl::load(
            self.apply_filters(
                query,
                FeMicroServiceSchema::dsl::fe_micro_service.into_boxed(),
            )
            .order_by(FeMicroServiceSchema::dsl::id.desc())
            .select(FeMicroServiceModel::FeMicroService::as_select())
            .limit(size as i64)
            .offset(((page - 1) * size) as i64),
            &mut conn,
        )
        .await?;
        Ok(Pagination {
            page,
            size,
            pages,
            total: total as u64,
            data: Some(apps),
        })
    }

    pub async fn update_micro_service(
        &self,
        service_id: &str,
        app: &FeMicroServiceModel::UpdatableFeMicroService,
    ) -> repository::Result<Option<FeMicroServiceModel::FeMicroService>> {
        let mut conn = self.pool.get().await?;
        let app = conn
            .transaction::<Option<FeMicroServiceModel::FeMicroService>, anyhow::Error, _>(
                |mut tx| {
                    Box::pin(async move {
                        let updated_app = AsyncDsl::get_result(
                            diesel::update(FeMicroServiceSchema::dsl::fe_micro_service)
                                .filter(FeMicroServiceSchema::dsl::service_id.eq(service_id))
                                .set(app)
                                .returning(FeMicroServiceModel::FeMicroService::as_returning()),
                            &mut tx,
                        )
                        .await
                        .optional()?;
                        Ok(updated_app)
                    })
                },
            )
            .await?;
        Ok(app)
    }

    pub async fn save(
        &self,
        app: &FeMicroServiceModel::InsertAbleFeMicroService<'_>,
    ) -> repository::Result<FeMicroServiceModel::FeMicroService> {
        let mut conn = self.pool.get().await?;
        let saved_app = conn
            .transaction::<FeMicroServiceModel::FeMicroService, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let saved_app = AsyncDsl::get_result(
                        diesel::insert_into(FeMicroServiceSchema::dsl::fe_micro_service)
                            .values(app)
                            .returning(FeMicroServiceModel::FeMicroService::as_returning()),
                        &mut tx,
                    )
                    .await?;
                    Ok(saved_app)
                })
            })
            .await?;
        Ok(saved_app)
    }

    pub async fn save_micro_apps(
        &self,
        apps: &Vec<FeMicroServiceModel::InsertAbleFeMicroService<'_>>,
    ) -> repository::Result<usize> {
        let size = apps.len();
        if size == 0 {
            return Ok(0);
        }
        let mut conn = self.pool.get().await?;
        let size = conn
            .transaction::<usize, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let size = AsyncDsl::execute(
                        diesel::insert_into(FeMicroServiceSchema::dsl::fe_micro_service)
                            .values(apps),
                        &mut tx,
                    )
                    .await?;
                    Ok(size)
                })
            })
            .await?;
        Ok(size)
    }
}
