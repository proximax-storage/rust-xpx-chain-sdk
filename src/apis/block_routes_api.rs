use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct BlockRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> BlockRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> BlockRoutesApiClient<C> {
        BlockRoutesApiClient {
            configuration,
        }
    }
}

pub trait BlockRoutesApi {
    fn get_block_by_height(&self, height: i64) -> Box<dyn Future<Item = crate::models::BlockInfoDto, Error = Error<serde_json::Value>>>;
    fn get_block_receipts(&self, height: i64) -> Box<dyn Future<Item = crate::models::StatementsDto, Error = Error<serde_json::Value>>>;
    fn get_block_transactions(&self, height: i64, page_size: Option<i32>, id: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>>;
    fn get_blocks_by_height_with_limit(&self, height: i64, limit: i32) -> Box<dyn Future<Item = Vec<crate::models::BlockInfoDto>, Error = Error<serde_json::Value>>>;
    fn get_merkle_receipts(&self, height: i64, hash: &str) -> Box<dyn Future<Item = crate::models::MerkleProofInfoDto, Error = Error<serde_json::Value>>>;
    fn get_merkle_transaction(&self, height: i64, hash: &str) -> Box<dyn Future<Item = crate::models::MerkleProofInfoDto, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>BlockRoutesApi for BlockRoutesApiClient<C> {
    fn get_block_by_height(&self, height: i64) -> Box<dyn Future<Item = crate::models::BlockInfoDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/block/{height}".to_string())
        ;
        let altura :i64 = 1;
        req = req.with_path_param("height".to_string(), altura.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_block_receipts(&self, height: i64) -> Box<dyn Future<Item = crate::models::StatementsDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/block/{height}/receipts".to_string())
        ;
        req = req.with_path_param("height".to_string(), height.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_block_transactions(&self, height: i64, page_size: Option<i32>, id: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/block/{height}/transactions".to_string())
        ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        req = req.with_path_param("height".to_string(), height.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_blocks_by_height_with_limit(&self, height: i64, limit: i32) -> Box<dyn Future<Item = Vec<crate::models::BlockInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/blocks/{height}/limit/{limit}".to_string())
        ;
        req = req.with_path_param("height".to_string(), height.to_string());
        req = req.with_path_param("limit".to_string(), limit.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_merkle_receipts(&self, height: i64, hash: &str) -> Box<dyn Future<Item = crate::models::MerkleProofInfoDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/block/{height}/receipt/{hash}/merkle".to_string())
        ;
        req = req.with_path_param("height".to_string(), height.to_string());
        req = req.with_path_param("hash".to_string(), hash.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_merkle_transaction(&self, height: i64, hash: &str) -> Box<dyn Future<Item = crate::models::MerkleProofInfoDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/block/{height}/transaction/{hash}/merkle".to_string())
        ;
        req = req.with_path_param("height".to_string(), height.to_string());
        req = req.with_path_param("hash".to_string(), hash.to_string());

        req.execute(self.configuration.borrow())
    }

}
