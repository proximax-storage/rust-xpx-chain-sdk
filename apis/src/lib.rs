// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate xpx_chain_sdk as sdk;
extern crate xpx_chain_utils as utils;

use self::error::Error;
pub use self::sirius_client::*;
pub use self::dtos::*;
pub use self::internally::*;


mod dtos;
mod error;
mod internally;
mod request;
mod routes;
mod sirius_client;

type Result<T> = std::result::Result<T, Error>;
