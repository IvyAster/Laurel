use crate::service::profile::ProfileService;
use actix_web::{post, web};
use laurel_actix::Data;
use laurel_actix::types::{Autowired, RequestBody, route};
use laurel_uc_api::profile::{ProfileBo, ProfileQuery};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/interface/system/uc/profile").service(list_profiles),
    );
}

#[post("/profiles")]
async fn list_profiles(
    profile_service: Autowired<ProfileService>,
    req: RequestBody<ProfileQuery>,
) -> route::Result<Vec<ProfileBo>> {
    let profiles: Vec<ProfileBo>  = profile_service
        .list(req.account_id.as_str(), &req.keys)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();
    Data!(
        profiles
    )
}
