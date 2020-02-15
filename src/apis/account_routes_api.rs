#[allow(unused_imports)]
use core::option::Option;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::result::Result;
use std::sync::Arc;

use ::async_trait::*;
use failure::_core::pin::Pin;

use futures::Future;
use hyper;
use hyper::client::connect::Connect;
use serde_json::Value;

use crate::apis::configuration::ApiClient;
use crate::apis::Error;
use crate::models::account::{AccountInfoDto, AccountInfo};

use super::configuration;
use super::request as __internal_request;

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
    pub async fn get_account_info(self, account_id: &str) -> Result<AccountInfo, Error<Value>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/account/{accountId}".to_string());

        req = req.with_path_param("accountId".to_string(), account_id.to_string());

       req.execute(self.client).await
    }
}
