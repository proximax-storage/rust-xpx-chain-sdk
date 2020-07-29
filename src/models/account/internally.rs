/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::ripemd160::{Digest, Ripemd160},
    ::sha3::Sha3_256,
};

use crate::models::network::NetworkType;

pub(crate) static HASH512_LENGTH: usize = 64;

pub static EMPTY_PUBLIC_KEY: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

pub(crate) fn public_key_to_address(
    public_key: &str,
    version: NetworkType,
) -> crate::Result<[u8; 25]> {
    let pk: Vec<u8> = hex::decode(public_key)?;

    // step 1: sha3 hash of the public key
    let sha3_public_key_hash = Sha3_256::digest(&pk);

    // step 2: Ripemd160 hash of (1)
    let ripemd160step_one_hash = Ripemd160::digest(&sha3_public_key_hash);

    // step 3: add version byte in front of (2)
    let version_prefixed_ripemd160hash = [&[*version], &*ripemd160step_one_hash].concat();

    // step 4: get the checksum of (3)
    let step_three_checksum = generate_checksum(&version_prefixed_ripemd160hash);

    // step 5: concatenate (3) and (4)
    let concat_step_three_and_step_six =
        [&version_prefixed_ripemd160hash, &*step_three_checksum].concat();

    let mut bts: [u8; 25] = [0u8; 25];
    bts.copy_from_slice(&concat_step_three_and_step_six[..25]);

    Ok(bts)
}

fn generate_checksum(input: &[u8]) -> Box<[u8]> {
    // step 1: sha3 hash of (input
    let sha3_hash = Sha3_256::digest(input);

    // step 2: get the first NUM_CHECKSUM_BYTES bytes of (1)
    let p = &sha3_hash[0..4];

    Box::from(p)
}
