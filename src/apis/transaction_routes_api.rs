use std::fmt::Debug;
use std::sync::Arc;

use hyper::client::connect::Connect;

use crate::apis::sirius_client::ApiClient;
use crate::models::account::{AccountInfo, AccountInfoDto};
use crate::models::transaction::SignedTransaction;

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
    pub async fn announce_transaction(self, transaction_payload: &SignedTransaction) -> super::Result<AnnounceTransactionInfo> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/transaction".to_string(),
        );
        req = req.with_body_param(transaction_payload);

        req.execute(self.client).await
    }
//    fn announce_cosignature_transaction(&self, cosignature: crate::models::Cosignature) -> super::Result<AccountInfo> {
//        let mut req = __internal_request::Request::new(
//            hyper::Method::PUT,
//            "/transaction/cosignature".to_string()
//        );
//        req = req.with_body_param(cosignature);
//
//        req.execute(self.configuration.borrow())
//    }

//    fn announce_partial_transaction(&self, transaction_payload: crate::models::TransactionPayload) -> Box<dyn Future<Item = crate::models::AnnounceTransactionInfoDto, Error = Error<serde_json::Value>>> {
//        let mut req = __internal_request::Request::new(hyper::Method::Put, "/transaction/partial".to_string())
//        ;
//        req = req.with_body_param(transaction_payload);
//
//        req.execute(self.configuration.borrow())
//    }

//    fn get_transaction(&self, transaction_id: &str) -> Box<dyn Future<Item = crate::models::TransactionInfoDto, Error = Error<serde_json::Value>>> {
//        let mut req = __internal_request::Request::new(hyper::Method::Get, "/transaction/{transactionId}".to_string())
//        ;
//        req = req.with_path_param("transactionId".to_string(), transaction_id.to_string());
//
//        req.execute(self.configuration.borrow())
//    }
//
//    fn get_transaction_status(&self, hash: &str) -> Box<dyn Future<Item = crate::models::TransactionStatusDto, Error = Error<serde_json::Value>>> {
//        let mut req = __internal_request::Request::new(hyper::Method::Get, "/transaction/{hash}/status".to_string())
//        ;
//        req = req.with_path_param("hash".to_string(), hash.to_string());
//
//        req.execute(self.configuration.borrow())
//    }
//
//    fn get_transactions(&self, transaction_ids: crate::models::TransactionIds) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>> {
//        let mut req = __internal_request::Request::new(hyper::Method::Post, "/transaction".to_string())
//        ;
//        req = req.with_body_param(transaction_ids);
//
//        req.execute(self.configuration.borrow())
//    }
//
//    fn get_transactions_statuses(&self, transaction_hashes: crate::models::TransactionHashes) -> Box<dyn Future<Item = Vec<crate::models::TransactionStatusDto>, Error = Error<serde_json::Value>>> {
//        let mut req = __internal_request::Request::new(hyper::Method::Post, "/transaction/statuses".to_string())
//        ;
//        req = req.with_body_param(transaction_hashes);
//
//        req.execute(self.configuration.borrow())
//    }
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
