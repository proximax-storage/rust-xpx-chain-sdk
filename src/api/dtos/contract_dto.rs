// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Serialize, Deserialize)]
pub struct ContractDto {
    multisig: String,
    multisig_address: String,
    start: Vec<i32>,
    duration: Vec<i32>,
    hash: String,
    customers: Vec<String>,
    executors: Vec<String>,
    verifiers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ContractInfoDto {
    #[serde(rename = "contract")]
    contract: ContractDto,
}