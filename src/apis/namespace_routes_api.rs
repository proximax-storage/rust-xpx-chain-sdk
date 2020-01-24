use futures::Future;
use hyper;
use serde_json;

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use super::{configuration, Error};
use super::request as __internal_request;

pub struct NamespaceRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NamespaceRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NamespaceRoutesApiClient<C> {
        NamespaceRoutesApiClient {
            configuration,
        }
    }
}

pub trait NamespaceRoutesApi {
    fn get_namespace(&self, namespace_id: &str) -> Box<dyn Future<Item=crate::models::namespace::NamespaceInfoDto, Error=Error<serde_json::Value>>>;
    fn get_namespaces_from_account(&self, account_id: &str, page_size: Option<i32>, id: Option<&str>) -> Box<dyn Future<Item=Vec<crate::models::namespace::NamespaceInfoDto>, Error=Error<serde_json::Value>>>;
    fn get_namespaces_from_accounts(&self, account_ids: crate::models::account::AccountIds, page_size: Option<i32>, id: Option<&str>) -> Box<dyn Future<Item=Vec<crate::models::namespace::NamespaceInfoDto>, Error=Error<serde_json::Value>>>;
    fn get_namespaces_names(&self, namespace_ids: crate::models::namespace::NamespaceIds) -> Box<dyn Future<Item=Vec<crate::models::namespace::NamespaceNameDto>, Error=Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> NamespaceRoutesApi for NamespaceRoutesApiClient<C> {
    fn get_namespace(&self, namespace_id: &str) -> Box<dyn Future<Item=crate::models::namespace::NamespaceInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/namespace/{namespaceId}".to_string())
            ;
        req = req.with_path_param("namespaceId".to_string(), namespace_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_namespaces_from_account(&self, account_id: &str, page_size: Option<i32>, id: Option<&str>) -> Box<dyn Future<Item=Vec<crate::models::namespace::NamespaceInfoDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{accountId}/namespaces".to_string())
            ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_namespaces_from_accounts(&self, account_ids: crate::models::account::AccountIds, page_size: Option<i32>, id: Option<&str>) -> Box<dyn Future<Item=Vec<crate::models::namespace::NamespaceInfoDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/account/namespaces".to_string())
            ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        req = req.with_body_param(account_ids);

        req.execute(self.configuration.borrow())
    }

    fn get_namespaces_names(&self, namespace_ids: crate::models::namespace::NamespaceIds) -> Box<dyn Future<Item=Vec<crate::models::namespace::NamespaceNameDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/namespace/names".to_string())
            ;
        req = req.with_body_param(namespace_ids);

        req.execute(self.configuration.borrow())
    }
}
