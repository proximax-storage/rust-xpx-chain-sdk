// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionHashes {
    /// The array of transaction hashes.
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
}

impl From<Vec<&str>> for TransactionHashes {
    fn from(e: Vec<&str>) -> Self {
        let mut ids = TransactionHashes::default();
        for id in e {
            ids.hashes.push(id.trim().to_uppercase())
        }
        ids
    }
}
