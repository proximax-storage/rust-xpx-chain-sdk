//use futures::Future;
//use hyper;
//use serde_json;
//use std::rc::Rc;
//
//
//use super::{configuration, Error};
//use super::request as __internal_request;

//pub struct ChainRoutesApiClient<C: hyper::client::connect::Connect> {
//    configuration: Rc<configuration::Configuration<C>>,
//}
//
//impl<C: hyper::client::connect::Connect> ChainRoutesApiClient<C> {
//    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ChainRoutesApiClient<C> {
//        ChainRoutesApiClient {
//            configuration,
//        }
//    }
//}
//
//pub trait ChainRoutesApi {
//    fn get_blockchain_height(&self) -> Box<dyn Future<Item=crate::models::blockchain::HeightInfoDto, Error=Error<serde_json::Value>>>;
//    fn get_blockchain_score(&self) -> Box<dyn Future<Item=crate::models::blockchain::BlockchainScoreDto, Error=Error<serde_json::Value>>>;
//}
//
//impl<C: hyper::client::connect::Connect> ChainRoutesApi for ChainRoutesApiClient<C> {
//    fn get_blockchain_height(&self) -> Box<dyn Future<Item=crate::models::blockchain::HeightInfoDto, Error=Error<serde_json::Value>>> {
//        let req = __internal_request::Request::new(hyper::Method::Get, "/chain/height".to_string())
//            ;
//
//        req.execute(&self.configuration)
//    }
//
//    fn get_blockchain_score(&self) -> Box<dyn Future<Item=crate::models::blockchain::BlockchainScoreDto, Error=Error<serde_json::Value>>> {
//        let req = __internal_request::Request::new(hyper::Method::GET, "/chain/score".to_string())
//            ;
//
//        req.execute(self.configuration)
//    }
//}
