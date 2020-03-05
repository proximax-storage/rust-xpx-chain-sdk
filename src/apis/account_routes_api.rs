use ::std::fmt::Debug;
use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::apis::internally::{valid_vec_hash, valid_vec_len};

use crate::models::{
    account::{AccountInfo, AccountInfoDto},
    errors::ERR_EMPTY_ADDRESSES_IDS,
    utils::is_hex
};

use super::{request as __internal_request, Result, sirius_client::ApiClient};

/// Account ApiClient routes.
///
#[derive(Clone)]
pub struct AccountRoutes<C: Connect> {
    client: Arc<ApiClient<C>>,
}

/// Account related endpoints.
///
impl<C: Connect> AccountRoutes<C>
    where
        C: Clone + Send + Sync + 'static
{
    pub(crate) fn new(client: Arc<ApiClient<C>>) -> Self {
        AccountRoutes {
            client,
        }
    }

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
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
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

    pub async fn get_accounts_info(self, accounts_id: Vec<&str>) -> Result<Vec<AccountInfo>> {
        valid_vec_len(&accounts_id, ERR_EMPTY_ADDRESSES_IDS)?;

        #[derive(Clone, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct AccountsId<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            addresses: Option<Vec<&'a str>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            public_keys: Option<Vec<&'a str>>
        };

        let mut accounts = AccountsId{ addresses: None, public_keys: None };
        let mut public_keys = vec![];
        let mut addresses = vec![];

        for (i, id) in accounts_id.iter().enumerate() {
            if is_hex(*id){
                public_keys.push(*id);
            }
            else {
                addresses.push(*id);
            }

            if i == accounts_id.len() -1 {
                if !public_keys.is_empty() {
                    accounts.public_keys = Some(public_keys.to_owned())
                }else if !addresses.is_empty() {
                    accounts.addresses = Some(addresses.to_owned())
                }
            }
        }

        let mut req = __internal_request::Request::new(
            Method::POST,
            "/account".to_string(),
        );

        req = req.with_body_param(&accounts);

        let dto: Vec<AccountInfoDto> = req.execute(self.client).await?;

        let mut accounts_info: Vec<AccountInfo> = Vec::with_capacity(dto.len());
        for i in dto {
            let account_info = i;
            accounts_info.push(account_info.to_struct()?);
        }

        Ok(accounts_info)
    }
}
