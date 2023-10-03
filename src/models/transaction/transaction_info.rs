/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::helpers::TransactionHash;

use super::Height;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub height: Height,
    pub index: u32,
    pub id: String,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<TransactionHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_component_hash: Option<TransactionHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_hash: Option<TransactionHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_aggregate_hash: Option<TransactionHash>,
}

impl TransactionInfo {
    pub fn transaction_hash(&self) -> TransactionHash {
        match self.hash.to_owned() {
            Some(h) => h,
            None => TransactionHash::zero(),
        }
    }
}

impl core::fmt::Display for TransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
