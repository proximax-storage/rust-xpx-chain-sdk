use futures::Future;
use hyper;
use serde_json;

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use super::{configuration, Error};
use super::request as __internal_request;

pub struct MosaicRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> MosaicRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> MosaicRoutesApiClient<C> {
        MosaicRoutesApiClient {
            configuration,
        }
    }
}

pub trait MosaicRoutesApi {
    fn get_mosaic(&self, mosaic_id: &str) -> Box<dyn Future<Item=crate::models::mosaic::MosaicInfoDto, Error=Error<serde_json::Value>>>;
    fn get_mosaics(&self, mosaic_ids: crate::models::mosaic::MosaicIds) -> Box<dyn Future<Item=Vec<crate::models::mosaic::MosaicInfoDto>, Error=Error<serde_json::Value>>>;
    fn get_mosaics_names(&self, mosaic_ids: crate::models::mosaic::MosaicIds) -> Box<dyn Future<Item=Vec<crate::models::mosaic::MosaicNamesDto>, Error=Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> MosaicRoutesApi for MosaicRoutesApiClient<C> {
    fn get_mosaic(&self, mosaic_id: &str) -> Box<dyn Future<Item=crate::models::mosaic::MosaicInfoDto, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/mosaic/{mosaicId}".to_string())
            ;
        req = req.with_path_param("mosaicId".to_string(), mosaic_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_mosaics(&self, mosaic_ids: crate::models::mosaic::MosaicIds) -> Box<dyn Future<Item=Vec<crate::models::mosaic::MosaicInfoDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/mosaic".to_string())
            ;
        req = req.with_body_param(mosaic_ids);

        req.execute(self.configuration.borrow())
    }

    fn get_mosaics_names(&self, mosaic_ids: crate::models::mosaic::MosaicIds) -> Box<dyn Future<Item=Vec<crate::models::mosaic::MosaicNamesDto>, Error=Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/mosaic/names".to_string())
            ;
        req = req.with_body_param(mosaic_ids);

        req.execute(self.configuration.borrow())
    }
}
