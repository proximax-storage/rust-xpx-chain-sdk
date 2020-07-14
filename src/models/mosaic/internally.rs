/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use sha3::{Digest, Sha3_256};

use crate::{
    models::{account::PublicAccount, namespace::NAMESPACE_BIT, Uint64},
    utils::array_u8_to_u64,
};

use super::MosaicNonce;

pub(super) const XPX_DIVISIBILITY: u64 = 1_000_000;

pub(super) const XPX_MAX_VALUE: u64 = XPX_MAX_RELATIVE_VALUE * XPX_DIVISIBILITY;

pub(super) const XPX_MAX_RELATIVE_VALUE: u64 = 9_000_000_000;

pub(super) const XPX_MIN_VALUE: u64 = 1;

pub(super) const PRX_XPX_U64: u64 = 13_833_723_942_089_965_046;

pub(super) fn generate_mosaic_id(nonce: MosaicNonce, owner_public_id: PublicAccount) -> Uint64 {
    let mut hash = Sha3_256::default();

    let nonce_bytes = nonce.to_array();

    hash.input(nonce_bytes);

    let owner_bytes: [u8; 32] = owner_public_id.to_bytes();

    hash.input(owner_bytes);

    let hash_to_array = hash.result();

    Uint64(array_u8_to_u64(&mut hash_to_array.as_slice()) ^ NAMESPACE_BIT)
}
