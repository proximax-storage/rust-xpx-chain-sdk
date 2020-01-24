use futures::Future;
use hyper;
use serde_json;

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use super::{configuration, Error};
use super::request as __internal_request;

pub struct NetworkRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NetworkRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NetworkRoutesApiClient<C> {
        NetworkRoutesApiClient {
            configuration,
        }
    }
}

pub trait NetworkRoutesApi {
    fn get_network_type(&self) -> Box<dyn Future<Item=crate::models::network::NetworkTypeDto, Error=Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> NetworkRoutesApi for NetworkRoutesApiClient<C> {
    fn get_network_type(&self) -> Box<dyn Future<Item=crate::models::network::NetworkTypeDto, Error=Error<serde_json::Value>>> {
        let req = __internal_request::Request::new(hyper::Method::Get, "/network".to_string())
            ;

        req.execute(self.configuration.borrow())
    }
}
