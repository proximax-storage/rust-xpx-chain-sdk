#[allow(unused_imports)]
use core::option::Option;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::sync::Arc;

use ::async_trait::*;
use failure::_core::pin::Pin;
use futures::Future;
use hyper;
use hyper::client::connect::Connect;
use serde::Deserialize;
use serde_json::Value;

use crate::apis::configuration::ApiClient;
use crate::apis::Error;
use crate::models::account::{AccountInfo, AccountInfoDto};

use super::configuration;
use super::request as __internal_request;

type Result<T> = std::result::Result<T, Error<Value>>;

#[derive(Debug, Clone)]
pub struct AccountRoutesApiClient<C: hyper::client::connect::Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: hyper::client::connect::Connect> AccountRoutesApiClient<C> where C: Clone {
    pub fn new(client: Arc<ApiClient<C>>) -> AccountRoutesApiClient<C> {
        let clone = client.clone();

        AccountRoutesApiClient {
            client: clone,
        }
    }
}

impl<C: hyper::client::connect::Connect> AccountRoutesApiClient<C>
    where
        C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_account_info(self, account_id: &str) -> Result<AccountInfo> {
        assert!(
            !account_id.is_empty(),
            "account_id string is empty."
        );

        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/account/{accountId}".to_string(),
        );

        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        let dto: Result<AccountInfoDto> = req.execute(self.client).await;

        Ok(dto.unwrap().to_struct()?)
    }
}
