use crate::IpConfig;

pub fn setup(config: &IpConfig) -> (ip2region::Searcher, ip2region::Searcher){
    let ipv4_searcher = ip2region::Searcher::new(config.ip_v4.as_str()).expect("Failed to load ip2region v4 db");
    let ipv6_searcher = ip2region::Searcher::new(config.ip_v6.as_str()).expect("Failed to load ip2region v6 db");
    (ipv4_searcher, ipv6_searcher)
}