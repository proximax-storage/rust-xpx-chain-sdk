// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[macro_use]
extern crate serde_derive;
extern crate xpx_chain_sdk as sdk;

use std::error::Error;

pub use self::block::*;
pub use self::client::*;

pub mod client;
pub mod block;
mod model;

type Result<T> = std::result::Result<T, tokio_tungstenite::tungstenite::Error>;