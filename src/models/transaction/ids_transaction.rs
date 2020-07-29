// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionIds {
    /// The array of transaction ids.
    #[serde(rename = "transactionIds")]
    pub transaction_ids: Vec<String>,
}

impl From<Vec<&str>> for TransactionIds {
    fn from(e: Vec<&str>) -> Self {
        let mut ids = TransactionIds::default();
        for id in e {
            ids.transaction_ids.push(id.trim().to_uppercase())
        }
        ids
    }
}
