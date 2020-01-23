use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct DiagnosticRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DiagnosticRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DiagnosticRoutesApiClient<C> {
        DiagnosticRoutesApiClient {
            configuration,
        }
    }
}

pub trait DiagnosticRoutesApi {
    fn get_diagnostic_storage(&self, ) -> Box<dyn Future<Item = crate::models::StorageInfoDto, Error = Error<serde_json::Value>>>;
    fn get_server_info(&self, ) -> Box<dyn Future<Item = crate::models::ServerDto, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>DiagnosticRoutesApi for DiagnosticRoutesApiClient<C> {
    fn get_diagnostic_storage(&self, ) -> Box<dyn Future<Item = crate::models::StorageInfoDto, Error = Error<serde_json::Value>>> {
        let req = __internal_request::Request::new(hyper::Method::Get, "/diagnostic/storage".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_server_info(&self, ) -> Box<dyn Future<Item = crate::models::ServerDto, Error = Error<serde_json::Value>>> {
        let req = __internal_request::Request::new(hyper::Method::Get, "/diagnostic/server".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
