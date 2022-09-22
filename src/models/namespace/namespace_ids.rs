/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::NamespaceId;

#[derive(Serialize, Default, Deserialize)]
pub struct NamespaceIds {
    /// The array of namespace identifiers.
    #[serde(rename = "namespaceIds")]
    pub namespace_ids: Vec<String>,
}

impl From<Vec<NamespaceId>> for NamespaceIds {
    fn from(e: Vec<NamespaceId>) -> Self {
        let mut ids = NamespaceIds::default();
        for m in e {
            ids.namespace_ids.push(m.to_string())
        }
        ids
    }
}
