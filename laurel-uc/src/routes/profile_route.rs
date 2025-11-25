use actix_web::{post, web};
use laurel_actix::types::{Autowired, Done, RequestBody, LR};
use laurel_uc_api::profile_api::{ProfileBo, ProfileQuery};
use crate::service::profile_service::ProfileService;

pub fn config(cfg: &mut web::ServiceConfig) {
    //cfg.service(web::scope("/api/uc/account").service(login).service(test));
    cfg.service(web::scope("/interface/uc/profile").service(list_profiles)
        //.service(find_account)
        //.service(test1)
    );
}


#[post("/profiles")]
async fn list_profiles(
    profile_service: Autowired<ProfileService>,
    req: RequestBody<ProfileQuery>
) -> Done<Vec<ProfileBo>>{
    Ok(
        LR::of(
            profile_service.list(req.account_id.as_str(), &req.keys)
                .await?
                .into_iter()
                .map(|e| e.into())
                .collect()
        )
    )
}