use ::std::fmt::Debug;
use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::models::account::{AccountInfo, AccountInfoDto};

use super::{request as __internal_request, Result, sirius_client::ApiClient};

#[derive(Clone)]
pub struct AccountRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> AccountRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        AccountRoutesApiClient {
            client,
        }
    }
}

impl<C: Connect> AccountRoutesApiClient<C>
    where
        C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_account_info(self, account_id: &str) -> Result<AccountInfo> {
        assert!(
            !account_id.is_empty(),
            "account_id string is empty."
        );

        let mut req = __internal_request::Request::new(
            Method::GET,
            "/account/{accountId}".to_string(),
        );

        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        let dto: Result<AccountInfoDto> = req.execute(self.client).await;

        Ok(dto?.to_struct()?)
    }
}
