// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Serialize, Deserialize)]
pub struct FieldDto {
    #[serde(rename = "key")]
    key: String,
    #[serde(rename = "value")]
    value: String,
}