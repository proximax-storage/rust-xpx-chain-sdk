use ::std::fmt::Debug;
use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::models::account::{AccountInfo, AccountInfoDto};

use super::{request as __internal_request, Result, sirius_client::ApiClient};

/// Account ApiClient routes.
///
#[derive(Clone)]
pub struct AccountRoutes<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> AccountRoutes<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        AccountRoutes {
            client,
        }
    }
}

/// Account related endpoints.
///
impl<C: Connect> AccountRoutes<C>
    where
        C: Clone + Send + Sync + 'static
{
    /// Get [Account] information
    ///
    /// # Inputs
    ///
    /// * `account_id` =    The public key or address of the account.
    ///
    /// # Example
    ///
    /// ```
    ///use hyper::Client;
    ///use xpx_chain_sdk::apis::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnetswap.xpxsirius.io:3000";
    ///const PUBLIC_KEY: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(node, Client::new());
    ///
    ///    let account_info = client.account.get_account_info( PUBLIC_KEY).await;
    ///
    ///    match account_info {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [AccountInfo] the account information or
    /// whose error value is an `Error<Value>` describing the error that occurred.
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
