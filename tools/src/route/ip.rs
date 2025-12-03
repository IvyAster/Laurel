pub mod ip_api {
    use crate::service::ip::IpSearchService;
    use actix_web::{post, web};
    use laurel_actix::Data;
    use laurel_actix::types::{Autowired, RequestBody, route};
    use laurel_tool_api::ip::{IpLocationBo, IpReqBo};

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/interface/tools/ip").service(ip_location));
    }

    #[post("/location")]
    pub async fn ip_location(
        service: Autowired<IpSearchService>,
        req: RequestBody<IpReqBo>,
    ) -> route::Result<IpLocationBo> {
        Data!(Into::<IpLocationBo>::into(
            service.search(req.ip.as_str()).await?
        ))
    }
}
