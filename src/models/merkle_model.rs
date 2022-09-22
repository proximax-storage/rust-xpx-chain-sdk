// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Debug, Serialize, Deserialize)]
pub struct MerklePathItem {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerkleProofInfo {
    /// The complementary data needed to calculate the merkle root.
    #[serde(rename = "merklePath", skip_serializing_if = "Option::is_none")]
    pub merkle_path: Option<Vec<MerklePathItem>>,
}
