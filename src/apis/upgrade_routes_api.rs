use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct UpgradeRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> UpgradeRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UpgradeRoutesApiClient<C> {
        UpgradeRoutesApiClient {
            configuration,
        }
    }
}

pub trait UpgradeRoutesApi {
    fn get_upgrade(&self, height: i64) -> Box<dyn Future<Item = crate::models::BlockchainUpgradeDto, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>UpgradeRoutesApi for UpgradeRoutesApiClient<C> {
    fn get_upgrade(&self, height: i64) -> Box<dyn Future<Item = crate::models::BlockchainUpgradeDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/upgrade/{height}".to_string())
        ;
        req = req.with_path_param("height".to_string(), height.to_string());

        req.execute(self.configuration.borrow())
    }

}
