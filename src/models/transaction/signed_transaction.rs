/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::Value;

use crate::helpers::TransactionHash;

use super::TransactionType;

/// Used to transfer the transaction data and the signature to a nem server in order to
/// initiate and broadcast a transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedTransaction {
    /// The transaction type.
    #[serde(rename = "transactionType")]
    pub entity_type: TransactionType,

    /// The serialized transaction data.
    pub payload: String,

    /// The transaction hash.
    pub hash: TransactionHash,
}

impl SignedTransaction {
    pub(crate) fn hash_to_bytes(&self) -> &[u8] {
        self.hash.as_bytes()
    }

    pub fn payload_to_bytes(&self) -> Vec<u8> {
        hex::decode(&self.get_payload()).unwrap()
    }

    pub fn get_payload(&self) -> String {
        self.payload.to_owned()
    }

    pub fn get_type(&self) -> TransactionType {
        self.entity_type
    }

    pub fn get_hash(&self) -> TransactionHash {
        self.hash
    }

    // pub fn type_to_string(&self) -> String {
    //     self.entity_type.to_string()
    // }

    pub fn as_value(&self) -> Value {
        serde_json::from_str(&format!("{}", self)).unwrap()
    }
}

impl core::fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
