// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Serialize, Deserialize)]
pub struct ConfigDto {
    height: Vec<i32>,
    network_config: String,
    supported_entity_versions: String,
}


