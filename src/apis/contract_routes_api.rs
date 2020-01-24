use futures::Future;
use hyper;
use serde_json;

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use super::{configuration, Error};
use super::request as __internal_request;

pub struct ContractRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ContractRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ContractRoutesApiClient<C> {
        ContractRoutesApiClient {
            configuration,
        }
    }
}

pub trait ContractRoutesApi {
    fn get_account_contract(&self, public_key: &str) -> Box<dyn Future<Item=Vec<crate::models::contract::ContractInfoDto>, Error=Error<serde_json::Value>>>;
    //    fn get_account_contracts(&self, public_keys: Option<crate::models::account::PublicKeys>) -> Box<dyn Future<Item=Vec<crate::models::contract::ContractInfoDto>, Error=Error<serde_json::Value>>>;
    fn get_contract(&self, contract_id: &str) -> Box<dyn Future<Item=crate::models::contract::ContractInfoDto, Error=Error<serde_json::Value>>>;
    fn get_contracts(&self, account_ids: crate::models::account::AccountIds) -> Box<dyn Future<Item=Vec<crate::models::contract::ContractInfoDto>, Error=Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> ContractRoutesApi for ContractRoutesApiClient<C> {
    fn get_account_contract(&self, public_key: &str) -> Box<dyn Future<Item=Vec<crate::models::contract::ContractInfoDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{publicKey}/contracts".to_string())
            ;
        req = req.with_path_param("publicKey".to_string(), public_key.to_string());

        req.execute(self.configuration.borrow())
    }

//    fn get_account_contracts(&self, public_keys: Option<crate::models::account::PublicKeys>) -> Box<dyn Future<Item=Vec<crate::models::contract::ContractInfoDto>, Error=Error<serde_json::Value>>> {
//        let mut req = __internal_request::Request::new(hyper::Method::Post, "/account/contracts".to_string())
//            ;
//        req = req.with_body_param(public_keys);
//
//        req.execute(self.configuration.borrow())
//    }

    fn get_contract(&self, contract_id: &str) -> Box<dyn Future<Item=crate::models::contract::ContractInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/contract/{contractId}".to_string())
            ;
        req = req.with_path_param("contractId".to_string(), contract_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_contracts(&self, account_ids: crate::models::account::AccountIds) -> Box<dyn Future<Item=Vec<crate::models::contract::ContractInfoDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/contract".to_string())
            ;
        req = req.with_body_param(account_ids);

        req.execute(self.configuration.borrow())
    }
}
