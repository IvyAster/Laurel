use std::time::Duration;
use tracing::info;

pub use fred::prelude::*;
pub use fred::types::*;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Redis(Pool);

impl Redis {
    pub async fn new(config: &RedisConfig) -> anyhow::Result<Redis> {
        let servers = |c: &RedisConfig| -> Vec<Server> {
            c.hosts.iter().map(|h| {
                Server{
                    host: h.host.clone().into(),
                    port: h.port,
                }
            }).collect()
        };

        let server = |c: &RedisConfig| -> ServerConfig{
            let host = c.hosts.first().unwrap();
            ServerConfig::Centralized{
                server: Server {
                    host: host.host.clone().into(),
                    port: host.port,
                },
            }
        };
        let s = match &config.mode {
            Some(x) => {
                match x.as_str() {
                    "cluster" => {
                        ServerConfig::Clustered {
                            hosts: servers(config),
                            policy: Default::default(),
                        }
                    },
                    "sentinel" => {
                        ServerConfig::Sentinel {
                            hosts: servers(config),
                            service_name: config.server_name.clone().unwrap(),
                        }
                    },
                    _ => server(config),
                }
            },
            _ =>server(config),
        };

        info!("pool init before: {:?}", &s);
        let redis_config = Config{
            fail_fast: true,
            blocking: Default::default(),
            username: config.username.clone(),
            password: config.password.clone(),
            server: s,
            version: RespVersion::RESP3,
            database: config.db,
        };
        let pool = match Builder::from_config(redis_config)
            .build_pool(
                match config.max_pool_size {
                    Some(x) => x as usize,
                    _ => 10,
                }
            ){
          Ok( pool) => pool,
          Err(e) => panic!("Redis Pool build Error: {:?}", e),
        };

        pool.init().await.expect("Redis Pool init Error");
        Ok(
            Redis(pool)
        )
    }

    pub fn pool(&self) -> &Pool {
        &self.0
    }

    pub async fn get_optional<R>(&self, key: &str) -> Result<Option<R>, Error>
    where R: FromValue
    {
        let value =  self.0.get::<Value, &str>( key).await?;
        match value{
            Value::Null => Ok(None),
            _ => Ok(Some(value.convert()?)),
        }
    }

    pub async fn get_del_optional<R>(&self, key: &str) -> Result<Option<R>, Error>
    where R: FromValue
    {
        let value =  self.0.getdel::<Value, &str>( key).await?;
        match value{
            Value::Null => Ok(None),
            _ => {
                Ok(Some(value.convert()?))
            },
        }
    }

    pub async fn get_del<R>(&self, key: &str) -> Result<R, Error>
    where R: FromValue
    {
        Ok(
            self.0.getdel::<R, &str>(key).await?
        )
    }

    pub async fn get<R>(&self, key: &str) -> Result<R, Error>
    where R: FromValue
    {
        Ok(
            self.0.get::<R, &str>( key).await?
        )
    }

    pub async fn set<V>(&self, key: &str, value: V) -> Result<(), Error>
    where V: TryInto<Value> + Send,
          V::Error: Into<Error> + Send,
    {
        let _: () = self.0.set::<(), &str, V>(key, value, None, None, false).await?;
        Ok(())
    }

    pub async fn set_with_expire<V>(&self, key: &str, value: V, duration: Duration) -> Result<(), Error>
    where V: TryInto<Value> + Send,
          V::Error: Into<Error> + Send,
    {
        let expire = Some(
            Expiration::EX( duration.as_secs() as i64 )
        );
        let _: () = self.0.set::<(), &str, V>(key, value, expire, None, false).await?;
        Ok(())
    }

    pub async fn expire<V>(&self, key: &str, duration: Duration) -> Result<(), Error>{
        let _: () = self.0.expire::<(), &str>(key, duration.as_secs() as i64, None).await?;
        Ok(())
    }
}





#[derive(Debug, Deserialize)]
pub struct RedisConfig{

    // single cluster sentinel
    pub mode: Option<String>,

    pub hosts: Vec<RedisHost>,

    pub username: Option<String>,

    pub password: Option<String>,

    pub db: Option<u8>,

    //pub min_pool_size: Option<u32>,

    pub max_pool_size: Option<u32>,

    pub connection_timeout: Option<u32>,

    //pub read_timeout: Option<u32>,

    pub server_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RedisHost{
    pub host: String,
    pub port: u16,
}
