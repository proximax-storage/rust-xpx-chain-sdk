use ::serde_json::Value;

use self::error::Error;

mod error;
mod request;
mod account_routes_api;
mod block_routes_api;
mod chain_routes_api;
mod node_routes_api;
mod mosaic_routes_api;
mod transaction_routes_api;

pub mod sirius_client;

type Result<T> = std::result::Result<T, Error<Value>>;
