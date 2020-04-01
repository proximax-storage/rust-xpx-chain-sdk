use self::const_routes::*;
use self::error::Error;

mod account_routes_api;
mod block_routes_api;
mod chain_routes_api;
mod const_routes;
mod error;
mod exchange_routes_api;
mod mosaic_routes_api;
mod namespace_routes_api;
mod node_routes_api;
mod request;
mod transaction_routes_api;

pub mod internally;
pub mod sirius_client;

type Result<T> = std::result::Result<T, Error>;
