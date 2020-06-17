// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use super::{HashAlgorithmEnum, Uint64Dto};

#[derive(Serialize, Deserialize)]
pub(crate) struct SecretProofTransactionBodyDto {
    hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient: Option<String>,
    /// The original random set of bytes.
    proof: String,
}

/// SecretProofTransactionDto : Transaction that revealed a proof.
#[derive(Serialize, Deserialize)]
pub(crate) struct SecretProofTransactionDto {
    signature: String,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient: Option<String>,
    /// The original random set of bytes.
    proof: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbeddedSecretProofTransactionDto {
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    hash_algorithm: HashAlgorithmEnum,
    secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient: Option<String>,
    /// The original random set of bytes.
    proof: String,
}
