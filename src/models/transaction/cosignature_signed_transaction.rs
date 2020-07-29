/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {serde::Serialize, std::fmt};

use super::{HashValue, Signature, Signer};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosignatureSignedTransaction {
    pub parent_hash: HashValue,
    pub signature: Signature,
    pub signer: Signer,
}

impl CosignatureSignedTransaction {
    pub fn new(parent_hash: HashValue, signature: Signature, signer: Signer) -> Self {
        Self {
            parent_hash,
            signature,
            signer,
        }
    }
}

impl fmt::Display for CosignatureSignedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
