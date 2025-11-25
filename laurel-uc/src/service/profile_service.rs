use std::sync::Arc;
use laurel_actix::types::Running;
use crate::model::profile_model::Profile;
use crate::repository::profile_repository::ProfileRepository;

#[derive(Debug)]
pub struct ProfileService{
    profile_repository: Arc<ProfileRepository>,
}

impl ProfileService {
    pub fn new(profile_repository: Arc<ProfileRepository>) -> Self {
        ProfileService {
            profile_repository
        }
    }

    pub async fn list(&self, account_id: &str, keys:&Option<Vec<String>>) -> Running<Vec<Profile>>{
        if let Some(ks) = keys{
            return if ks.is_empty() {
                Ok(vec![])
            } else {
                Ok(self.profile_repository.list_with_keys(account_id, ks).await?)
            }

        }
        Ok(self.profile_repository.list(account_id).await?)
    }
}

