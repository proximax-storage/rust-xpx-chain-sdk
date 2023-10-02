/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::ops::Deref;

use crate::transaction::Height;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HeightInfo {
    #[serde(rename = "height")]
    pub height: Height,
}

impl Deref for HeightInfo {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.height
    }
}

impl<'a> core::fmt::Display for HeightInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.height).unwrap_or_default())
    }
}

/// Get the current score of the chain.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainScore {
    #[serde(rename = "scoreHigh")]
    pub score_high: u64,
    #[serde(rename = "scoreLow")]
    pub score_low: u64,
}

impl<'a> core::fmt::Display for BlockchainScore {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

/// Diagnostic information about the node storage.
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInfo {
    /// The number of blocks stored.
    #[serde(rename = "numBlocks")]
    pub num_blocks: u64,
    /// The number of transactions stored.
    #[serde(rename = "numTransactions")]
    pub num_transactions: u64,
    /// The number of accounts created.
    #[serde(rename = "numAccounts")]
    pub num_accounts: u64,
}

impl<'a> core::fmt::Display for StorageInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}
