use crate::model::dict::{
    Dict, DictCreateReq, DictDeleteReq, DictQueryReq, DictUpdateReq, DictValue, DictValueCreateReq,
    DictValueDeleteReq, DictValueQueryReq, DictValueUpdateReq,
};
use crate::repository::dict::DictRepository;
use laurel_common::types::Pagination;
use std::sync::Arc;
use laurel_actix::types::service;

pub struct DictService {
    dict_repository: Arc<DictRepository>,
}

impl DictService {
    pub fn new(dict_repository: Arc<DictRepository>) -> Self {
        DictService { dict_repository }
    }

    pub async fn page_dict(
        &self,
        req: &DictQueryReq,
        page: u32,
        size: u32,
    ) -> service::Result<Pagination<Dict>> {
        let queryable = req.into();
        self.dict_repository.page_dict(&queryable, page, size).await
    }

    pub async fn update_dict(&self, req: &DictUpdateReq) -> service::Result<Option<Dict>> {
        let dict = self.dict_repository.find_dict(req.id).await?;
        match dict {
            Some(d) => {
                if d.dict_type == "default" {
                    return Err(anyhow::Error::msg(format!(
                        "字典[{:?}]为内建类型, 不允许更新",
                        req.id
                    )));
                }
                if let Some(id) = &req.dict_id {
                    if id == &d.dict_id && d.id != req.id {
                        return Err(anyhow::Error::msg(format!(
                            "字典id[{:?}]已存在, 不允许更新修改",
                            id
                        )));
                    }
                }
            }
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "字典[{:?}]不存在, 不允许更新",
                    req.id
                )));
            }
        }
        let updatable = req.into();
        let update = self.dict_repository.update(req.id, &updatable).await?;
        Ok(update)
    }

    pub async fn create_dict(&self, req: &DictCreateReq) -> service::Result<Dict> {
        if self
            .dict_repository
            .count_dict(req.dict_id.as_str())
            .await?
            > 0
        {
            return Err(anyhow::Error::msg(format!(
                "字典 [{}] 已存在, 无法创建",
                req.dict_id
            )));
        }
        let insertable = req.into();
        let dict = self.dict_repository.save(&insertable).await?;
        Ok(dict)
    }

    pub async fn delete_dict(&self, req: &DictDeleteReq) -> service::Result<Option<Dict>> {
        let dict = match self.dict_repository.find_dict(req.id).await? {
            Some(d) => d,
            None => {
                return Err(anyhow::Error::msg(format!(
                    "字典 [{:?}] 不存在, 无法删除",
                    req.dict_id
                )));
            }
        };
        if dict.dict_type == "default" {
            return Err(anyhow::Error::msg(format!(
                "字典 [{:?}] 为内建类型, 不允许删除",
                req.dict_id
            )));
        }
        let deleted = self.dict_repository.delete(req.id).await?;
        if !deleted {
            return Err(anyhow::Error::msg(format!(
                "字典 [{:?}] 删除失败",
                req.dict_id
            )));
        }
        Ok(Some(dict))
    }

    pub async fn page_values(
        &self,
        req: &DictValueQueryReq,
        page: u32,
        size: u32,
    ) -> service::Result<Pagination<DictValue>> {
        let queryable = req.into();
        self.dict_repository
            .page_dict_value(&queryable, page, size)
            .await
    }

    pub async fn create_value(&self, req: &DictValueCreateReq) -> service::Result<DictValue> {
        if self
            .dict_repository
            .count_value(req.dict_id.as_str(), req.value_id.as_str())
            .await?
            > 0
        {
            return Err(anyhow::Error::msg(format!(
                "字典值 [{}-{}] 已存在, 无法创建",
                req.dict_id, req.value_id
            )));
        }
        let insertable = req.into();
        let value = self.dict_repository.save_value(&insertable).await?;
        Ok(value)
    }

    pub async fn update_value(&self, req: &DictValueUpdateReq) -> service::Result<Option<DictValue>> {
        match self.dict_repository.find_value_by_id(req.id).await? {
            Some(value) => {
                if value.dict_type == "default" {
                    return Err(anyhow::Error::msg(format!(
                        "字典值 [{:?}] 为内建类型, 不允许更新",
                        value.value_id
                    )));
                }
                match &req.value_id {
                    Some(value_id) => {
                        match self
                            .dict_repository
                            .find_value(&value.dict_id, value_id.as_str())
                            .await?
                        {
                            Some(v) => {
                                if v.id != req.id {
                                    return Err(anyhow::Error::msg(format!(
                                        "字典值 [{}-{}] 已存在, 无法更新",
                                        v.dict_id, v.value_id
                                    )));
                                }
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            None => {
                return Err(anyhow::Error::msg(format!(
                    "字典值 [{:?}] 不存在, 无法更新",
                    req.id
                )));
            }
        };
        let updatable = req.into();
        let value = self
            .dict_repository
            .update_value(req.id, &updatable)
            .await?;
        Ok(value)
    }

    pub async fn delete_value(&self, req: &DictValueDeleteReq) -> service::Result<Option<DictValue>> {
        let value = match self.dict_repository.find_value_by_id(req.id).await? {
            Some(value) => {
                if value.dict_type == "default" {
                    return Err(anyhow::Error::msg(format!(
                        "字典值 [{:?}] 为内建类型, 不允许删除",
                        value.value_id
                    )));
                }
                value
            }
            None => {
                return Err(anyhow::Error::msg(format!(
                    "字典值 [{:?}] 不存在, 无法删除",
                    req.value_id
                )));
            }
        };
        if !self.dict_repository.delete_value(req.id).await? {
            return Err(anyhow::Error::msg(format!(
                "字典值 [{:?}] 删除失败",
                req.value_id
            )));
        }
        Ok(Some(value))
    }
}
