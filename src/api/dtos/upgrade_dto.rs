// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Serialize, Deserialize)]
pub(crate) struct UpgradeDto {
    #[serde(rename = "height")]
    height: Vec<i32>,
    #[serde(rename = "blockChainVersion")]
    block_chain_version: Vec<i32>,
}
