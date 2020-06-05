// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;
extern crate xpx_chain_sdk as sdk;
extern crate xpx_chain_utils as utils;

pub use self::error::Result;
pub use self::sirius_client::*;
pub use self::dtos::*;
pub use self::internally::*;

mod dtos;
pub mod error;
mod internally;
mod request;
mod routes;
mod sirius_client;