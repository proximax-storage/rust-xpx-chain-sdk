// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    // #[serde(rename = "payload")]
    // pub payload: MerkleProofInfo,
    #[serde(rename = "type")]
    _type: String,
}
