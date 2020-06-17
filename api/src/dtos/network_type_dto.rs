// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct NetworkTypeDto {
    /// The name of the network.
    #[serde(rename = "name")]
    name: String,
    /// A short text describing the network.
    #[serde(rename = "description")]
    description: String,
}