#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate std;
extern crate xpx_chain_sdk as sdk;

use self::const_routes::*;
use self::error::Error;
pub use self::sirius_client::*;

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

mod dtos;
mod internally;
mod sirius_client;

type Result<T> = std::result::Result<T, Error>;
