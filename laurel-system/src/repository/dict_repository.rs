use crate::model::dict_model::{
    Dict, DictValue, InsertableDict, InsertableDictValue, QueryableDict, QueryableDictValue,
    UpdatableDict, UpdatableDictValue,
};
use crate::repository::AsyncDsl;
use crate::schema::schema::dict as DictSchema;
use crate::schema::schema::dict::dsl as DictDsl;
use diesel::ExpressionMethods;
use diesel::pg::Pg;
use diesel::{OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::AsyncConnection;

use crate::schema::schema::dict_value as DictValueSchema;
use DictValueSchema::dsl as DictValueDsl;
use laurel_actix::types::Running;
use laurel_common::types::Pagination;
use laurel_pg::DbPool;

pub struct DictRepository {
    pool: DbPool,
}

impl DictRepository {
    pub fn new(pool: DbPool) -> Self {
        DictRepository { pool }
    }

    pub async fn count_dict(&self, dict_id: &str) -> Running<i64> {
        let mut conn = self.pool.get().await?;
        let total = AsyncDsl::get_result::<i64>(
            DictDsl::dict.filter(DictDsl::dict_id.eq(dict_id)).count(),
            &mut conn,
        )
        .await?;
        Ok(total)
    }

    pub async fn find_dict(&self, id: i64) -> Running<Option<Dict>> {
        let mut conn = self.pool.get().await?;
        let dict = AsyncDsl::first(
            DictDsl::dict
                .filter(DictDsl::id.eq(id))
                .select(Dict::as_returning()),
            &mut conn,
        )
        .await
        .optional()?;
        Ok(dict)
    }

    pub async fn page_dict<'a>(
        &self,
        queryable: &QueryableDict<'a>,
        page: u32,
        size: u32,
    ) -> Running<Pagination<Dict>> {
        let mut conn = self.pool.get().await?;
        let total = AsyncDsl::get_result::<i64>(
            self.apply_filters(queryable, DictDsl::dict.into_boxed())
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
            self.apply_filters(queryable, DictDsl::dict.into_boxed())
                .order_by(DictSchema::id.desc())
                .offset(offset as i64)
                .limit(size as i64)
                .select(Dict::as_returning()),
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

    pub async fn update(&self, id: i64, updatable: &UpdatableDict) -> Running<Option<Dict>> {
        let mut conn = self.pool.get().await?;
        let dict = conn
            .transaction::<Option<Dict>, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let dict = AsyncDsl::get_result(
                        diesel::update(DictDsl::dict)
                            .filter(DictSchema::id.eq(id))
                            .set(updatable)
                            .returning(Dict::as_returning()),
                        &mut tx,
                    )
                    .await
                    .optional()?;
                    Ok(dict)
                })
            })
            .await?;
        Ok(dict)
    }

    pub async fn save<'a>(&self, insertable: &InsertableDict<'a>) -> Running<Dict> {
        let mut conn = self.pool.get().await?;
        let dict = conn
            .transaction::<Dict, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let dict = AsyncDsl::get_result(
                        diesel::insert_into(DictDsl::dict)
                            .values(insertable)
                            .returning(Dict::as_returning()),
                        &mut tx,
                    )
                    .await?;
                    Ok(dict)
                })
            })
            .await?;
        Ok(dict)
    }

    pub async fn delete(&self, id: i64) -> Running<bool> {
        let mut conn = self.pool.get().await?;
        let result = conn
            .transaction::<bool, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let result = AsyncDsl::get_result(
                        diesel::delete(DictDsl::dict)
                            .filter(DictSchema::id.eq(id))
                            .returning(diesel::dsl::sql::<diesel::sql_types::Bool>("true")),
                        &mut tx,
                    )
                    .await?;
                    Ok(result)
                })
            })
            .await?;
        Ok(result)
    }

    pub async fn count_value(&self, dict_id: &str, value_id: &str) -> Running<i64> {
        let mut conn = self.pool.get().await?;
        let total = AsyncDsl::get_result::<i64>(
            DictValueDsl::dict_value
                .filter(DictValueDsl::dict_id.eq(dict_id))
                .filter(DictValueDsl::value_id.eq(value_id))
                .select(diesel::dsl::count_star()),
            &mut conn,
        )
        .await?;
        Ok(total)
    }

    pub async fn find_value_by_id(&self, id: i64) -> Running<Option<DictValue>> {
        let mut conn = self.pool.get().await?;
        let dict_value = AsyncDsl::first(
            DictValueDsl::dict_value
                .filter(DictValueDsl::id.eq(id))
                .select(DictValue::as_returning()),
            &mut conn,
        )
        .await
        .optional()?;
        Ok(dict_value)
    }

    pub async fn find_value(&self, dict_id: &str, value_id: &str) -> Running<Option<DictValue>> {
        let mut conn = self.pool.get().await?;
        let dict_value = AsyncDsl::first(
            DictValueDsl::dict_value
                .filter(DictValueDsl::dict_id.eq(dict_id))
                .filter(DictValueDsl::value_id.eq(value_id))
                .select(DictValue::as_returning()),
            &mut conn,
        )
        .await
        .optional()?;
        Ok(dict_value)
    }

    pub async fn page_dict_value<'a>(
        &self,
        queryable: &QueryableDictValue<'a>,
        page: u32,
        size: u32,
    ) -> Running<Pagination<DictValue>> {
        let mut conn = self.pool.get().await?;
        let total = AsyncDsl::get_result::<i64>(
            self.apply_value_filters(queryable, DictValueDsl::dict_value.into_boxed())
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
        let values = AsyncDsl::load(
            self.apply_value_filters(queryable, DictValueDsl::dict_value.into_boxed())
                .order_by(DictValueSchema::id.asc())
                .offset(offset as i64)
                .limit(size as i64)
                .select(DictValue::as_returning()),
            &mut conn,
        )
        .await?;
        Ok(Pagination {
            page,
            size,
            pages,
            total: total as u64,
            data: Some(values),
        })
    }

    pub async fn update_value(
        &self,
        id: i64,
        updatable: &UpdatableDictValue,
    ) -> Running<Option<DictValue>> {
        let mut conn = self.pool.get().await?;
        let dict_value = conn
            .transaction::<Option<DictValue>, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let dict_value = AsyncDsl::get_result(
                        diesel::update(DictValueDsl::dict_value)
                            .filter(DictValueSchema::id.eq(id))
                            .set(updatable)
                            .returning(DictValue::as_returning()),
                        &mut tx,
                    )
                    .await
                    .optional()?;
                    Ok(dict_value)
                })
            })
            .await?;
        Ok(dict_value)
    }

    pub async fn save_value<'a>(&self, insertable: &InsertableDictValue<'a>) -> Running<DictValue> {
        let mut conn = self.pool.get().await?;
        let dict_value = conn
            .transaction::<DictValue, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let dict_value = AsyncDsl::get_result(
                        diesel::insert_into(DictValueDsl::dict_value)
                            .values(insertable)
                            .returning(DictValue::as_returning()),
                        &mut tx,
                    )
                    .await?;
                    Ok(dict_value)
                })
            })
            .await?;
        Ok(dict_value)
    }

    pub async fn delete_value(&self, id: i64) -> Running<bool> {
        let mut conn = self.pool.get().await?;
        let result = conn
            .transaction::<bool, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let result = AsyncDsl::get_result(
                        diesel::delete(DictValueDsl::dict_value)
                            .filter(DictValueSchema::id.eq(id))
                            .returning(diesel::dsl::sql::<diesel::sql_types::Bool>("true")),
                        &mut tx,
                    )
                    .await?;
                    Ok(result)
                })
            })
            .await?;
        Ok(result)
    }

    fn apply_filters<'a>(
        &self,
        queryable: &QueryableDict<'a>,
        mut query: DictSchema::BoxedQuery<'a, Pg>,
    ) -> DictSchema::BoxedQuery<'a, Pg> {
        if let Some(param) = queryable.dict_id
            && !param.is_empty()
        {
            query = query.filter(DictDsl::dict_id.eq(param.as_str()))
        }
        if let Some(param) = queryable.dict_name
            && !param.is_empty()
        {
            query = query.filter(DictDsl::dict_name.eq(param.as_str()))
        }
        query
    }

    fn apply_value_filters<'a>(
        &self,
        queryable: &QueryableDictValue<'a>,
        mut query: DictValueSchema::BoxedQuery<'a, Pg>,
    ) -> DictValueSchema::BoxedQuery<'a, Pg> {
        query = query.filter(DictValueDsl::dict_id.eq(queryable.dict_id));
        if let Some(param) = queryable.value_id
            && !param.is_empty()
        {
            query = query.filter(DictValueDsl::value_id.eq(param.as_str()))
        }
        if let Some(param) = queryable.value_name
            && !param.is_empty()
        {
            query = query.filter(DictValueDsl::value_name.eq(param.as_str()))
        }
        query
    }
}
