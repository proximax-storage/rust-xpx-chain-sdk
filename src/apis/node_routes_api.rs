use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct NodeRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NodeRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NodeRoutesApiClient<C> {
        NodeRoutesApiClient {
            configuration,
        }
    }
}

pub trait NodeRoutesApi {
    fn get_node_info(&self, ) -> Box<dyn Future<Item = crate::models::NodeInfoDto, Error = Error<serde_json::Value>>>;
    fn get_node_time(&self, ) -> Box<dyn Future<Item = crate::models::NodeTimeDto, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>NodeRoutesApi for NodeRoutesApiClient<C> {
    fn get_node_info(&self, ) -> Box<dyn Future<Item = crate::models::NodeInfoDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/node/info".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_node_time(&self, ) -> Box<dyn Future<Item = crate::models::NodeTimeDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/node/time".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
