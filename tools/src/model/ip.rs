use std::str::FromStr;
use anyhow::anyhow;
use fred::bytes_utils::Str;
use fred::error::{Error, ErrorKind};
use fred::prelude::FromValue;
use fred::types::Value;
use ip2region::Location;
use serde::{Deserialize, Serialize};
use laurel_tool_api::ip::IpLocationBo;

#[derive(Deserialize, Serialize, Clone)]
pub struct IpLocation{
    pub contry: Option<String>,
    pub region: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub isp: Option<String>,
}

impl From<Location> for IpLocation {
    fn from(location: Location) -> Self {
        IpLocation {
            contry: location.contry,
            region: location.region,
            province: location.province,
            city: location.city,
            isp: location.isp,
        }
    }
}

impl From<&Location> for IpLocation {
    fn from(location: &Location) -> Self {
        IpLocation {
            contry: location.contry.clone(),
            region: location.region.clone(),
            province: location.province.clone(),
            city: location.city.clone(),
            isp: location.isp.clone(),
        }
    }
}

impl FromValue for IpLocation {
    fn from_value(value: Value) -> Result<Self, Error> {
        let cache = match value {
            Value::String(v) => serde_json::from_slice::<IpLocation>(v.as_bytes())?,
            _ => return Err(Error::new(ErrorKind::Parse, "IpLocation Redis Deserialize Error")),
        };
        Ok(cache)
    }
}

impl TryInto<Value> for IpLocation {
    type Error = fred::error::Error;
    fn try_into(self) -> Result<Value, Self::Error> {
        let cache = serde_json::to_string(&self)?;
        Ok(Value::String(
            Str::from_str(cache.as_str())?
        ))
    }
}

impl Into<IpLocationBo> for IpLocation{
    fn into(self) -> IpLocationBo {
        IpLocationBo {
            contry: self.contry,
            region: self.region,
            province: self.province,
            city: self.city,
            isp: self.isp,
        }
    }
}