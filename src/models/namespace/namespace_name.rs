/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use crate::namespace::NamespaceId;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceName {
    #[serde(rename = "namespace_id")]
    pub namespace_id: NamespaceId,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    pub name: String,
}

impl fmt::Display for NamespaceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
