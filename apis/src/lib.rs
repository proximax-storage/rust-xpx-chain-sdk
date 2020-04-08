#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate std;
extern crate xpx_chain_sdk as sdk;

use self::error::Error;
use self::routes::*;
pub use self::sirius_client::*;

mod dtos;
mod error;
mod internally;
mod request;
mod routes;
mod sirius_client;

type Result<T> = std::result::Result<T, Error>;
