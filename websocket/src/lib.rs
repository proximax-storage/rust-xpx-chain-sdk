// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate serde_derive;
extern crate xpx_chain_apis as api;
extern crate xpx_chain_sdk as sdk;

use tokio_tungstenite::tungstenite::Error;

pub use self::block::*;
pub use self::client::*;
pub use self::status::*;
pub use self::transaction_confirmed::*;
pub use self::transaction_unconfirmed::*;

pub mod client;
pub mod block;
pub mod status;
pub mod transaction_confirmed;
pub mod transaction_unconfirmed;


mod model;

type Result<T> = std::result::Result<T, Error>;