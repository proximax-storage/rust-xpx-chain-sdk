// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate failure;
extern crate flatbuffers as fb;
#[macro_use]
extern crate serde_derive;
extern crate xpx_chain_crypto as crypto;
extern crate xpx_chain_utils as utils;

pub use self::models::account;
pub use self::models::alias;
pub use self::models::blockchain;
pub use self::models::errors;
pub use self::models::exchange;
pub use self::models::asset_id_model::AssetId;
pub use self::models::message;
pub use self::models::mosaic;
pub use self::models::multisig;
pub use self::models::namespace;
pub use self::models::network;
pub use self::models::node;
pub use self::models::transaction;
pub use self::models::Uint64;

mod models;

pub type Result<T> = ::std::result::Result<T, failure::Error>;
