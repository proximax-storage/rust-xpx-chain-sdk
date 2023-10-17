/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use crate::account::PublicAccount;

use super::{NamespaceAlias, NamespaceId, NamespaceType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceInfo {
    pub namespace_id: NamespaceId,
    pub active: bool,
    pub type_space: NamespaceType,
    pub depth: u8,
    pub levels: Vec<NamespaceId>,
    pub alias: NamespaceAlias,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<NamespaceInfo>>,
    pub owner: PublicAccount,
    pub start_height: u64,
    pub end_height: u64,
}

impl NamespaceInfo {
    pub fn get_parent(&self) -> Option<Box<NamespaceInfo>> {
        self.parent.to_owned()
    }
}

impl fmt::Display for NamespaceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
