/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::Result;

use {
    regex::Regex,
    sha3::{Digest, Sha3_256},
};

use crate::{errors_const, helpers::array_u8_to_u64, mosaic::UnresolvedMosaicId};

use super::NamespaceId;

pub(crate) const NAMESPACE_BIT: u64 = 1 << 63;

fn is_valid_namespace_name(name: &str) -> bool {
    let reg_valid_namespace: Regex = Regex::new(r"^[a-z0-9][a-z0-9-_]*$").unwrap();
    reg_valid_namespace.is_match(name)
}

/// Generates a `NamespaceId` from a namespaceFullName.
pub(crate) fn generate_namespace_path(name: &str) -> Result<Vec<NamespaceId>> {
    let parts: Vec<&str> = name.split('.').collect();

    ensure!(!parts.is_empty(), errors_const::ERR_INVALID_NAMESPACE_NAME);

    ensure!(parts.len() <= 3, errors_const::ERR_NAMESPACE_TOO_MANY_PART);

    let mut namespace_id = NamespaceId::default();

    let mut path: Vec<NamespaceId> = vec![];

    for part in parts {
        ensure!(is_valid_namespace_name(part), errors_const::ERR_INVALID_NAMESPACE_NAME);

        namespace_id = generate_namespace_id(part, namespace_id)?;

        path.push(namespace_id)
    }

    Ok(path)
}

pub(crate) fn generate_namespace_id(name: &str, parent_id: NamespaceId) -> Result<NamespaceId> {
    let mut result = Sha3_256::default();

    let id_to_bytes = parent_id.to_builder();

    result.update(id_to_bytes);

    result.update(name);

    let t_result = result.finalize();

    Ok(NamespaceId::create(array_u8_to_u64(&t_result) | NAMESPACE_BIT))
}
