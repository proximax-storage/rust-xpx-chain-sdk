use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod account_routes_api;
pub use self::account_routes_api::{ AccountRoutesApi, AccountRoutesApiClient };
mod block_routes_api;
pub use self::block_routes_api::{ BlockRoutesApi, BlockRoutesApiClient };
mod chain_routes_api;
pub use self::chain_routes_api::{ ChainRoutesApi, ChainRoutesApiClient };
mod config_routes_api;
pub use self::config_routes_api::{ ConfigRoutesApi, ConfigRoutesApiClient };
mod contract_routes_api;
pub use self::contract_routes_api::{ ContractRoutesApi, ContractRoutesApiClient };
mod diagnostic_routes_api;
pub use self::diagnostic_routes_api::{ DiagnosticRoutesApi, DiagnosticRoutesApiClient };
mod metadata_routes_api;
pub use self::metadata_routes_api::{ MetadataRoutesApi, MetadataRoutesApiClient };
mod mosaic_routes_api;
pub use self::mosaic_routes_api::{ MosaicRoutesApi, MosaicRoutesApiClient };
mod namespace_routes_api;
pub use self::namespace_routes_api::{ NamespaceRoutesApi, NamespaceRoutesApiClient };
mod network_routes_api;
pub use self::network_routes_api::{ NetworkRoutesApi, NetworkRoutesApiClient };
mod node_routes_api;
pub use self::node_routes_api::{ NodeRoutesApi, NodeRoutesApiClient };
mod transaction_routes_api;
pub use self::transaction_routes_api::{ TransactionRoutesApi, TransactionRoutesApiClient };
mod upgrade_routes_api;
pub use self::upgrade_routes_api::{ UpgradeRoutesApi, UpgradeRoutesApiClient };

pub mod configuration;
pub mod client;
