// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

/// RolesTypeEnum : The role of the node: * 1 - A peer node. * 2 - An api node.

/// The role of the node: * 1 - A peer node. * 2 - An api node. 
#[derive(Serialize, Deserialize)]
pub enum RolesTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,

}




