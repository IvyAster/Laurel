use std::net::IpAddr;
use std::sync::Arc;
use anyhow::anyhow;
use laurel_actix::types::{service};
use laurel_redis::Redis;
use crate::model::ip::IpLocation;

#[derive(Debug)]
pub struct IpSearchService{
    v4: Arc<ip2region::Searcher>,
    v6: Arc<ip2region::Searcher>,
    redis: Redis
}

impl IpSearchService{
    pub fn new(v4: Arc<ip2region::Searcher>, v6: Arc<ip2region::Searcher>, redis: Redis) -> Self {
        Self { v4, v6, redis }
    }

    pub async fn search(&self, ip: &str)-> service::Result<IpLocation>{
        let addr: IpAddr = ip.parse()?;
        let location = match addr {
            IpAddr::V4(_ip) => {
                self.v4.std_search(ip).map_err(|e| anyhow!(e.to_string()))?
            },
            IpAddr::V6(_ip) => {
                self.v6.std_search(ip).map_err(|e| anyhow!(e.to_string()))?
            }
        };
        let ip_location: IpLocation = location.into();
        self.redis.set::<IpLocation>(ip, ip_location.clone()).await?;
        Ok(ip_location)
    }
}




