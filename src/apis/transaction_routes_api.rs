use futures::Future;
use hyper;
use serde_json;

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use super::{configuration, Error};
use super::request as __internal_request;

pub struct TransactionRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TransactionRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TransactionRoutesApiClient<C> {
        TransactionRoutesApiClient {
            configuration,
        }
    }
}

pub trait TransactionRoutesApi {
    fn announce_cosignature_transaction(&self, cosignature: crate::models::multisig::Cosignature) -> Box<dyn Future<Item=crate::models::AnnounceTransactionInfoDto, Error=Error<serde_json::Value>>>;
    fn announce_partial_transaction(&self, transaction_payload: crate::models::transaction::TransactionPayload) -> Box<dyn Future<Item=crate::models::AnnounceTransactionInfoDto, Error=Error<serde_json::Value>>>;
    fn announce_transaction(&self, transaction_payload: crate::models::transaction::TransactionPayload) -> Box<dyn Future<Item=crate::models::AnnounceTransactionInfoDto, Error=Error<serde_json::Value>>>;
    fn get_transaction(&self, transaction_id: &str) -> Box<dyn Future<Item=crate::models::transaction::TransactionInfoDto, Error=Error<serde_json::Value>>>;
    fn get_transaction_status(&self, hash: &str) -> Box<dyn Future<Item=crate::models::transaction::TransactionStatusDto, Error=Error<serde_json::Value>>>;
    fn get_transactions(&self, transaction_ids: crate::models::transaction::TransactionIds) -> Box<dyn Future<Item=Vec<crate::models::transaction::TransactionInfoDto>, Error=Error<serde_json::Value>>>;
    fn get_transactions_statuses(&self, transaction_hashes: crate::models::transaction::TransactionHashes) -> Box<dyn Future<Item=Vec<crate::models::transaction::TransactionStatusDto>, Error=Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> TransactionRoutesApi for TransactionRoutesApiClient<C> {
    fn announce_cosignature_transaction(&self, cosignature: crate::models::multisig::Cosignature) -> Box<dyn Future<Item=crate::models::AnnounceTransactionInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/transaction/cosignature".to_string())
            ;
        req = req.with_body_param(cosignature);

        req.execute(self.configuration.borrow())
    }

    fn announce_partial_transaction(&self, transaction_payload: crate::models::transaction::TransactionPayload) -> Box<dyn Future<Item=crate::models::AnnounceTransactionInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/transaction/partial".to_string())
            ;
        req = req.with_body_param(transaction_payload);

        req.execute(self.configuration.borrow())
    }

    fn announce_transaction(&self, transaction_payload: crate::models::transaction::TransactionPayload) -> Box<dyn Future<Item=crate::models::AnnounceTransactionInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/transaction".to_string())
            ;
        req = req.with_body_param(transaction_payload);

        req.execute(self.configuration.borrow())
    }

    fn get_transaction(&self, transaction_id: &str) -> Box<dyn Future<Item=crate::models::transaction::TransactionInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/transaction/{transactionId}".to_string())
            ;
        req = req.with_path_param("transactionId".to_string(), transaction_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_transaction_status(&self, hash: &str) -> Box<dyn Future<Item=crate::models::transaction::TransactionStatusDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/transaction/{hash}/status".to_string())
            ;
        req = req.with_path_param("hash".to_string(), hash.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_transactions(&self, transaction_ids: crate::models::transaction::TransactionIds) -> Box<dyn Future<Item=Vec<crate::models::transaction::TransactionInfoDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/transaction".to_string())
            ;
        req = req.with_body_param(transaction_ids);

        req.execute(self.configuration.borrow())
    }

    fn get_transactions_statuses(&self, transaction_hashes: crate::models::transaction::TransactionHashes) -> Box<dyn Future<Item=Vec<crate::models::transaction::TransactionStatusDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/transaction/statuses".to_string())
            ;
        req = req.with_body_param(transaction_hashes);

        req.execute(self.configuration.borrow())
    }
}
