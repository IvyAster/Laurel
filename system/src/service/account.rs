use crate::model::account::{AccountEntity, AccountLoginVo};
use crate::repository::account::AccountRepository;
use crate::repository::passport::PassportRepository;
use crate::utils::{passport_utils};
use anyhow::{Error};
use laurel_actix::types::{service};
use laurel_redis::Redis;
use std::sync::Arc;
use chrono::{Local, NaiveDateTime};
use tracing::error;
use laurel_common::date_time::DTF;
use laurel_common::types::api;
use laurel_id_api::id::IdApi;
use laurel_logs_api::logs::{LogApi, LoginLogCreateReqBo};
use laurel_tool_api::ua::{UaApi};
use crate::model::ticket::{InsertableTicket, Ticket};
use crate::repository;

#[derive(Debug)]
pub struct AccountService {
    account_repository: Arc<AccountRepository>,
    passport_repository: Arc<PassportRepository>,
    redis: Redis,
    log_api: Arc<LogApi>,
    ticket_repository: Arc<repository::ticket::Repository>,
    token_service: Arc<crate::service::token::TokenService>,
    id_api: IdApi,
    ip_api: laurel_tool_api::ip::IpApi,
    ua_api: UaApi,
}

impl AccountService {
    pub fn new(
        account_repository: Arc<AccountRepository>,
        passport_repository: Arc<PassportRepository>,
        redis: Redis,
        log_api: Arc<LogApi>,
        ticket_repository: Arc<repository::ticket::Repository>,
        token_service: Arc<crate::service::token::TokenService>,
        id_api: IdApi,
        ip_api: laurel_tool_api::ip::IpApi,
        ua_api: UaApi,
    ) -> Self {
        Self {
            account_repository,
            passport_repository,
            redis,
            log_api,
            ticket_repository,
            token_service,
            id_api,
            ip_api,
            ua_api
        }
    }

    async fn do_login(&self, req: &AccountLoginVo) -> service::Result<(AccountEntity, Ticket)>{
        let account = self
            .account_repository
            .find_by_name(req.account.as_str(), "name")
            .await?
            .expect("account not found");
        let passport = self
            .passport_repository
            .find(account.account_id.as_str())
            .await?
            .expect("account passport not init");
        let p = passport_utils::password(
            account.account_id.as_str(),
            req.password.as_str(),
            passport.salt.as_str(),
        )?;
        if p != passport.password{
            return Err(Error::msg("account passport error"));
        }
        let ticket_id = self.id_api.id().await?;
        let token = self.token_service.make(ticket_id.as_str())?;
        let insertable = InsertableTicket{
            ticket_id: ticket_id.as_str(),
            token: token.as_str(),
            account_id: account.account_id.as_str(),
            login_type: "name",
            ticket_state: "normal",
            cts: Local::now().naive_local(),
            uts: Local::now().naive_local(),
            ets: Local::now().naive_local(),
        };
        let ticket = self.ticket_repository.save(&insertable).await?;
        Ok((account, ticket))
    }

    async fn process_ua(&self, req: &mut LoginLogCreateReqBo, ua: &str) {
        if let Some((browser, os, device)) = self.ua_api.parse(ua)
            .await
            .unwrap_or_else(api::ApiResult::from)
            .data
            .map(|data| data.show()){
            req.browser = browser;
            req.os = os;
            req.device = device;
        }
    }

    async fn process_ip(&self, req: &mut LoginLogCreateReqBo, ip: &str){
        req.ip = Some(String::from(ip));
        let location = self.ip_api.location(ip).await.unwrap_or_else(api::ApiResult::from).data.map(|data| data.show()).unwrap_or("未知".to_string());
        req.location = Some(location);
    }

    async fn after_login(&self, req: &AccountLoginVo, ua: Option<&str>, ip: String, result: &service::Result<(AccountEntity, Ticket)>){
        let mut log_req = LoginLogCreateReqBo::default();
        log_req.account = req.account.clone();
        log_req.login_type = "name".to_string();
        log_req.ip = Some(ip.clone());
        if let Some(ua) = ua{
            self.process_ua(& mut log_req, ua).await;
        }
        self.process_ip(& mut log_req, ip.as_str()).await;
        match &result{
            Ok((_account, token)) => {
                log_req.login_state = "normal".to_string();
                log_req.login_result = Some("登录成功".to_string());
                log_req.login_cts = token.cts.format(DTF).to_string();
                log_req.ticket_id = token.ticket_id.clone();
            },
            Err(err) => {
                log_req.login_state = "error".to_string();
                log_req.login_result = Some(format!("登录失败: {}", err.to_string()));
                log_req.login_cts = NaiveDateTime::default().format(DTF).to_string();
                if let Ok(id) = self.id_api.id().await{
                    log_req.ticket_id = id;
                }
            }
        };
        let api = Arc::clone(&self.log_api);
        tokio::task::spawn(async move {
            let result: api::Result<i64> = api.save_login_log(&log_req).await;
            if !result.is_ok(){
                error!("save login log [{:?}] error: {:?}", &log_req, result)
            }
        });
    }

    pub async fn login(&self, req: &AccountLoginVo, ua: Option<&str>, ip: String) -> service::Result<(AccountEntity, Ticket)> {
        let result = self.do_login(req).await;
        self.after_login(req, ua, ip, &result).await;
        result
    }

    pub async fn find_account_by_id(&self, account_id: &str) -> service::Result<Option<AccountEntity>> {
        let account = self
            .account_repository
            .find_by_account_id(account_id)
            .await?;
        Ok(account)
    }

    pub async fn find_account_by_account(
        &self,
        account_name: &str,
        account_type: &str,
    ) -> service::Result<Option<AccountEntity>> {
        let account = self
            .account_repository
            .find_by_name(account_name, account_type)
            .await?;
        Ok(account)
    }
}
