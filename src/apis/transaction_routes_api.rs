use std::fmt::Debug;
use std::sync::Arc;

use hyper::client::connect::Connect;

use crate::apis::sirius_client::ApiClient;
use crate::models::account::{AccountInfo, AccountInfoDto};
use crate::models::transaction::{SignedTransaction, TransactionStatus, TransactionStatusDto};

use super::request as __internal_request;
use std::fmt;

#[derive(Clone)]
pub struct TransactionRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> TransactionRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub fn new(client: Arc<ApiClient<C>>) -> TransactionRoutesApiClient<C> {
        let clone = client.clone();

        TransactionRoutesApiClient {
            client: clone,
        }
    }
}

impl<C: Connect> TransactionRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{

    pub async fn get_transaction_status(self, hash: &str) -> super::Result<TransactionStatus> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/transaction/{hash}/status".to_string()
        );

        req = req.with_path_param("hash".to_string(), hash.to_string());

        let dto: super::Result<TransactionStatusDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }

    pub async fn announce_transaction(self, transaction_payload: &SignedTransaction) -> super::Result<AnnounceTransactionInfo> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/transaction".to_string(),
        );

        req = req.with_body_param(transaction_payload);

        req.execute(self.client).await
    }
}

#[derive(Debug, Deserialize)]
pub struct AnnounceTransactionInfo {
    #[serde(rename = "message")]
    pub message: String,
}

impl fmt::Display for AnnounceTransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}", &self.message
        )
    }
}
