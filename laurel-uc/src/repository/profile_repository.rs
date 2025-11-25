use crate::model::profile_model::{InsertAbleProfile, Profile};
use crate::schema::schema::profile::dsl as ProfileDsl;
use chrono::Local;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use laurel_actix::types::Running;
use laurel_common::local_now;
use laurel_pg::DbPool;

#[derive(Clone, Debug)]
pub struct ProfileRepository {
    pool: DbPool,
}

impl ProfileRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self, account_id: &str) -> Running<Vec<Profile>> {
        let mut conn = self.pool.get().await?;
        let profiles = ProfileDsl::profile
            .filter(ProfileDsl::account_id.eq(account_id))
            .order_by(ProfileDsl::id.desc())
            .select(Profile::as_select())
            .load(&mut conn)
            .await?;
        Ok(profiles)
    }

    pub async fn list_with_keys(
        &self,
        account_id: &str,
        keys: &Vec<String>,
    ) -> Running<Vec<Profile>> {
        let mut conn = self.pool.get().await?;
        let profiles = ProfileDsl::profile
            .filter(ProfileDsl::account_id.eq(account_id))
            .filter(ProfileDsl::profile_key.eq_any(keys))
            .order_by(ProfileDsl::id.desc())
            .select(Profile::as_select())
            .load(&mut conn)
            .await?;
        Ok(profiles)
    }

    pub async fn find(&self, account_id: &str, key: &str) -> Running<Option<Profile>> {
        let mut conn = self.pool.get().await?;
        let profile = ProfileDsl::profile
            .filter(ProfileDsl::account_id.eq(account_id))
            .filter(ProfileDsl::profile_key.eq(key))
            .select(Profile::as_select())
            .first(&mut conn)
            .await
            .optional()?;
        Ok(profile)
    }

    pub async fn update(&self, account_id: &str, key: &str, value: &str) -> Running<usize> {
        let mut conn = self.pool.get().await?;
        let size = conn
            .transaction::<usize, anyhow::Error, _>(|mut tx_conn| {
                Box::pin(async move {
                    let size = diesel::update(
                        ProfileDsl::profile
                            .filter(ProfileDsl::account_id.eq(account_id))
                            .filter(ProfileDsl::profile_key.eq(key)),
                    )
                    .set(ProfileDsl::profile_value.eq(value))
                    .execute(&mut tx_conn)
                    .await?;
                    Ok(size)
                })
            })
            .await?;
        Ok(size)
    }

    pub async fn save(&self, profiles: Vec<(String, String, Option<String>)>) -> Running<usize> {
        let size = profiles.len();
        if size == 0 {
            return Ok(0);
        }
        let inserts: Vec<InsertAbleProfile> = profiles
            .iter()
            .map(|p| InsertAbleProfile {
                account_id: p.0.clone(),
                profile_key: p.1.clone(),
                profile_value: p.2.clone(),
                cts: local_now!(Local),
                uts: local_now!(Local),
            })
            .collect();
        let mut conn = self.pool.get().await?;
        let size = conn
            .transaction::<usize, anyhow::Error, _>(|mut tx_conn| {
                Box::pin(async move {
                    let size = diesel::insert_into(ProfileDsl::profile)
                        .values(&inserts)
                        .execute(&mut tx_conn)
                        .await?;
                    Ok(size)
                })
            })
            .await?;
        Ok(size)
    }
}
