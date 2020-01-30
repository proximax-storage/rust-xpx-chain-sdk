use futures::Future;
use hyper;
use serde_json;

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use super::{configuration, Error};
use super::request as __internal_request;

pub struct MetadataRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> MetadataRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> MetadataRoutesApiClient<C> {
        MetadataRoutesApiClient {
            configuration,
        }
    }
}

pub trait MetadataRoutesApi {
    fn get_account_metadata(&self, account_id: &str) -> Box<dyn Future<Item=crate::models::account::AddressMetadataInfoDto, Error=Error<serde_json::Value>>>;
    fn get_metadata(&self, metadata_id: &str) -> Box<dyn Future<Item=crate::models::namespace::NamespaceMetadataInfoDto, Error=Error<serde_json::Value>>>;
    fn get_metadatas(&self, metadata_ids: Option<crate::models::MetadataIds>) -> Box<dyn Future<Item=Vec<crate::models::account::AddressMetadataInfoDto>, Error=Error<serde_json::Value>>>;
    fn get_mosaic_metadata(&self, mosaic_id: &str) -> Box<dyn Future<Item=crate::models::mosaic::MosaicMetadataInfoDto, Error=Error<serde_json::Value>>>;
    fn get_namespace_metadata(&self, namespace_id: &str) -> Box<dyn Future<Item=crate::models::namespace::NamespaceMetadataInfoDto, Error=Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> MetadataRoutesApi for MetadataRoutesApiClient<C> {
    fn get_account_metadata(&self, account_id: &str) -> Box<dyn Future<Item=crate::models::account::AddressMetadataInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{accountId}/metadata".to_string())
            ;
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_metadata(&self, metadata_id: &str) -> Box<dyn Future<Item=crate::models::namespace::NamespaceMetadataInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/metadata/{metadataId}".to_string())
            ;
        req = req.with_path_param("metadataId".to_string(), metadata_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_metadatas(&self, metadata_ids: Option<crate::models::MetadataIds>) -> Box<dyn Future<Item=Vec<crate::models::account::AddressMetadataInfoDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/metadata".to_string())
            ;
        req = req.with_body_param(metadata_ids);

        req.execute(self.configuration.borrow())
    }

    fn get_mosaic_metadata(&self, mosaic_id: &str) -> Box<dyn Future<Item=crate::models::mosaic::MosaicMetadataInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/mosaic/{mosaic_id}/metadata".to_string())
            ;
        req = req.with_path_param("mosaic_id".to_string(), mosaic_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_namespace_metadata(&self, namespace_id: &str) -> Box<dyn Future<Item=crate::models::namespace::NamespaceMetadataInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/namespace/{namespaceId}/metadata".to_string())
            ;
        req = req.with_path_param("namespaceId".to_string(), namespace_id.to_string());

        req.execute(self.configuration.borrow())
    }
}
