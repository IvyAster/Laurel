use std::net::IpAddr;

pub fn ip(req: &actix_web::HttpRequest) -> String{
    // 1. 优先读取 X-Forwarded-For（多层代理时，第一个 IP 是真实客户端）
    if let Some(xff) = req.headers().get("X-Forwarded-For") {
        if let Ok(xff_str) = xff.to_str() {
            // X-Forwarded-For 格式：客户端IP, 代理1IP, 代理2IP
            let real_ip = xff_str.split(',').next().unwrap_or("").trim();
            if !real_ip.is_empty() && is_valid_ip(real_ip) {
                return real_ip.to_string();
            }
        }
    }
    // 2. 其次读取 X-Real-IP（Nginx 常用）
    if let Some(xri) = req.headers().get("X-Real-IP") {
        if let Ok(xri_str) = xri.to_str() {
            let ip = xri_str.trim();
            if !ip.is_empty() && is_valid_ip(ip) {
                return ip.to_string();
            }
        }
    }
    // 3. 兜底：无代理时的 remote_addr（剥离端口）
    //let remote_addr = req.connection_info().peer_addr().unwrap_or("unknown").clone();
    let conn_info = req.connection_info();
    parse_ip_from_addr(conn_info.peer_addr().unwrap_or("127.0.0.1"))
}

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

// 辅助：剥离端口（复用基础场景的函数）
fn parse_ip_from_addr(addr: &str) -> String {
    if addr.starts_with('[') {
        if let Some(end) = addr.find(']') {
            return addr[1..end].to_string();
        }
    }
    if let Some(colon) = addr.find(':') {
        return addr[0..colon].to_string();
    }
    addr.to_string()
}