#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate xpx_chain_sdk as sdk;
extern crate xpx_chain_utils as utils;

use self::error::Error;
pub use self::sirius_client::*;

mod dtos;
mod error;
mod internally;
mod request;
mod routes;
mod sirius_client;

type Result<T> = std::result::Result<T, Error>;
