/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use crate::helpers::TransactionHash;
use crate::transaction::TransactionGroupType;

use super::{Deadline, Height};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatus {
    pub group: String,
    pub status: String,
    pub hash: TransactionHash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Deadline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Height>,
}

impl TransactionStatus {
    pub fn new(
        group: String,
        status: String,
        hash: TransactionHash,
        deadline: Option<Deadline>,
        height: Option<Height>,
    ) -> Self {
        TransactionStatus { group, status, hash, deadline, height }
    }

    pub fn is_success(&self) -> bool {
        self.status == "Success"
    }

    pub fn is_confirmed(&self) -> bool {
        self.is_success() && self.group == "confirmed"
    }

    pub fn is_partial(&self) -> bool {
        self.is_success() && self.group == "partial"
    }

    pub fn to_group_type(&self) -> TransactionGroupType {
        match self.group.as_str() {
            "confirmed" => TransactionGroupType::Confirmed,
            "partial" => TransactionGroupType::Partial,
            "unconfirmed" => TransactionGroupType::Unconfirmed,
            _ => TransactionGroupType::Failed,
        }
    }
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
