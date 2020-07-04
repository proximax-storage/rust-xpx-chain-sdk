/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate failure;
extern crate flatbuffers as fb;
#[macro_use]
extern crate serde;
extern crate xpx_chain_crypto as crypto;

pub use self::models::*;

pub mod api;
mod models;
mod utils;
pub mod websocket;

type Result<T> = ::std::result::Result<T, failure::Error>;
