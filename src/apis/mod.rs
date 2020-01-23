use hyper;
use serde;
use serde_json;

pub use self::account_routes_api::{AccountRoutesApi, AccountRoutesApiClient};
pub use self::block_routes_api::{BlockRoutesApi, BlockRoutesApiClient};
pub use self::chain_routes_api::{ChainRoutesApi, ChainRoutesApiClient};
pub use self::config_routes_api::{ConfigRoutesApi, ConfigRoutesApiClient};
pub use self::contract_routes_api::{ContractRoutesApi, ContractRoutesApiClient};
pub use self::diagnostic_routes_api::{DiagnosticRoutesApi, DiagnosticRoutesApiClient};
pub use self::metadata_routes_api::{MetadataRoutesApi, MetadataRoutesApiClient};
pub use self::mosaic_routes_api::{MosaicRoutesApi, MosaicRoutesApiClient};
pub use self::namespace_routes_api::{NamespaceRoutesApi, NamespaceRoutesApiClient};
pub use self::network_routes_api::{NetworkRoutesApi, NetworkRoutesApiClient};
pub use self::node_routes_api::{NodeRoutesApi, NodeRoutesApiClient};
pub use self::transaction_routes_api::{TransactionRoutesApi, TransactionRoutesApiClient};
pub use self::upgrade_routes_api::{UpgradeRoutesApi, UpgradeRoutesApiClient};

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
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
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
        return Error::Hyper(e);
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

mod request;

mod account_routes_api;

mod block_routes_api;

mod chain_routes_api;

mod config_routes_api;

mod contract_routes_api;

mod diagnostic_routes_api;

mod metadata_routes_api;

mod mosaic_routes_api;

mod namespace_routes_api;

mod network_routes_api;

mod node_routes_api;

mod transaction_routes_api;

mod upgrade_routes_api;

pub mod configuration;
pub mod client;
