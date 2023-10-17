// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use std::str::FromStr;

use crate::TransactionHash;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionHashes {
    /// The array of transaction hashes.
    #[serde(rename = "hashes")]
    pub hashes: Vec<TransactionHash>,
}

impl TryFrom<Vec<&str>> for TransactionHashes {
    type Error = anyhow::Error;

    fn try_from(value: Vec<&str>) -> anyhow::Result<Self> {
        let mut ids = TransactionHashes::default();
        for id in value {
            ids.hashes.push(TransactionHash::from_str(id.trim())?)
        }
        Ok(ids)
    }
}

impl From<Vec<TransactionHash>> for TransactionHashes {
    fn from(e: Vec<TransactionHash>) -> Self {
        let mut ids = TransactionHashes::default();
        for id in e {
            ids.hashes.push(id)
        }
        ids
    }
}
