pub mod types;


use std::time::Duration;
use bb8::Pool;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use diesel_async::{AsyncPgConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use crate::types::DbConfig;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn setup(db_config: &DbConfig) -> anyhow::Result<DbPool>{
    let password = utf8_percent_encode(db_config.password.as_str(), NON_ALPHANUMERIC);
    let url = match &db_config.options {
        Some(o) => if !o.is_empty(){
            format!(
                "postgres://{user}:{password}@{host}:{port}/{database}?{options}",
                user=db_config.user,
                password=password,
                host=db_config.host,
                port=db_config.port,
                database=db_config.database,
                options=o,
            )
        }else{
            format!(
                "postgres://{user}:{password}@{host}:{port}/{database}",
                user=db_config.user,
                password=password,
                host=db_config.host,
                port=db_config.port,
                database=db_config.database,
            )
        },
        _ => format!(
            "postgres://{user}:{password}@{host}:{port}/{database}",
            user=db_config.user,
            password=password,
            host=db_config.host,
            port=db_config.port,
            database=db_config.database,
        )
    };

    let pool = Pool::builder()
        // 连接池配置
        .max_size(db_config.max_connections)  // 最大连接数（根据数据库性能调整）
        .min_idle(Some(db_config.min_connections))  // 最小闲置连接数（避免频繁创建连接）
        .max_lifetime(Some(Duration::from_secs(300)))  // 连接最大存活时间（5分钟）
        .idle_timeout(Some(Duration::from_secs(60)))  //
        // 连接闲置超时（1分钟）
        // 连接创建函数：通过 AsyncPgConnection 异步建立连接
        //.build_fn(move || AsyncPgConnection::establish(&url))
        .build(AsyncDieselConnectionManager::<AsyncPgConnection>::new(url))
        .await
        .expect("Failed to create database pool");
    Ok(pool)
}