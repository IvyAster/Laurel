use crate::model::fe_micro_service::{
    FeMicroService, FeMicroServiceCreateReq, FeMicroServiceQuery, InsertAbleFeMicroService,
    MicroServiceUpdateReq, UpdatableFeMicroService,
};
use crate::repository::fe_micro_service::FeMicroServiceRepository;
use anyhow::anyhow;
use bon::Builder;
use chrono::Local;
use laurel_common::types::Pagination;
use laurel_id_api::id::IdApi;
use std::sync::Arc;
use laurel_actix::types::service;

#[derive(Debug, Builder)]
pub struct FeMicroServiceImpl {
    micro_app_repository: Arc<FeMicroServiceRepository>,
    id_api: IdApi,
}

impl FeMicroServiceImpl {
    pub async fn list(&self, app_id: &str) -> service::Result<Vec<FeMicroService>> {
        self.micro_app_repository.list_services(app_id).await
    }
    pub async fn list_with_status(
        &self,
        app_id: &str,
        service_status: &str,
    ) -> service::Result<Vec<FeMicroService>> {
        self.micro_app_repository
            .list_services_with_state(app_id, service_status)
            .await
    }

    pub async fn page(
        &self,
        req: &FeMicroServiceQuery,
        page: u32,
        size: u32,
    ) -> service::Result<Pagination<FeMicroService>> {
        self.micro_app_repository
            .page_micro_apps(req, page, size)
            .await
    }

    pub async fn create(&self, req: &FeMicroServiceCreateReq) -> service::Result<FeMicroService> {
        let id = self.id_api.id().await?;
        let insertable_app = InsertAbleFeMicroService {
            app_id: req.app_id.as_str(),
            service_id: id.as_str(),
            service_name: req.service_name.as_str(),
            service_entry: req.service_entry.as_str(),
            mount_point: req.mount_point.as_str(),
            route_pattern: req.route_pattern.as_str(),
            service_status: if let Some(s) = &req.service_status
                && !s.is_empty()
            {
                s.as_str()
            } else {
                "open"
            },
            cts: Local::now().naive_local(),
            uts: Local::now().naive_local(),
        };
        self.micro_app_repository.save(&insertable_app).await
    }

    pub async fn update(&self, req: &MicroServiceUpdateReq) -> service::Result<FeMicroService> {
        if req.service_id.is_empty() {
            return Err(anyhow!("service_id为空, 无法更新"));
        }
        let updatable = UpdatableFeMicroService {
            service_name: req.service_name.clone(),
            service_entry: req.service_entry.clone(),
            route_pattern: req.route_pattern.clone(),
            service_status: req.service_status.clone(),
            uts: Local::now().naive_local(),
            mount_point: req.mount_point.clone(),
        };
        Ok(self
            .micro_app_repository
            .update_micro_service(req.service_id.as_str(), &updatable)
            .await?
            .expect("当前微服务不存在, 无法更新"))
    }
}
