/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    regex::Regex,
    sha3::{Digest, Sha3_256},
};

use crate::{
    account::Address,
    models::{asset_id_model::AssetId, errors_const},
    network::ALIAS_ADDRESS,
    utils::{array_u8_to_u64, u64_to_array_u8, vec_u8_to_hex},
};

use super::NamespaceId;

pub(crate) const NAMESPACE_BIT: u64 = 1 << 63;

fn is_valid_namespace_name(name: &str) -> bool {
    let reg_valid_namespace: Regex = Regex::new(r"^[a-z0-9][a-z0-9-_]*$").unwrap();
    reg_valid_namespace.is_match(name)
}

/// Generates a `NamespaceId` from a namespaceFullName.
pub(crate) fn generate_namespace_path(name: &str) -> crate::Result<Vec<NamespaceId>> {
    let parts: Vec<&str> = name.split(".").collect();

    ensure!(parts.len() != 0, errors_const::ERR_INVALID_NAMESPACE_NAME);

    ensure!(parts.len() <= 3, errors_const::ERR_NAMESPACE_TOO_MANY_PART);

    let mut namespace_id = NamespaceId::default();

    let mut path: Vec<NamespaceId> = vec![];

    for part in parts {
        ensure!(
            is_valid_namespace_name(part),
            errors_const::ERR_INVALID_NAMESPACE_NAME
        );

        namespace_id = generate_namespace_id(part, namespace_id)?;

        path.push(namespace_id)
    }

    Ok(path)
}

pub(crate) fn generate_namespace_id(
    name: &str,
    parent_id: NamespaceId,
) -> crate::Result<NamespaceId> {
    let mut result = Sha3_256::default();

    let id_to_bytes = parent_id.to_bytes();

    result.input(id_to_bytes);

    result.input(name);

    let t_result = result.result();

    Ok(NamespaceId::new(array_u8_to_u64(&t_result) | NAMESPACE_BIT))
}

// returns new Address from namespace identifier
pub(crate) fn new_address_from_namespace(namespace_id: NamespaceId) -> crate::Result<Address> {
    // 0x91 | namespaceId on 8 bytes | 16 bytes 0-pad = 25 bytes
    let mut address_raw = ALIAS_ADDRESS.to_hex();

    let buf = u64_to_array_u8(*namespace_id);

    address_raw += &vec_u8_to_hex(buf.to_vec());

    address_raw += "00000000000000000000000000000000";

    Address::from_encoded(&address_raw)
}
