/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{AbstractTransactionDto, HashAlgorithm};

/// SecretProofTransactionDto : Transaction that revealed a proof.
#[derive(Serialize, Deserialize)]
pub(crate) struct SecretProofTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    hash_algorithm: HashAlgorithm,
    /// The proof hashed.
    #[serde(rename = "secret")]
    secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient: Option<String>,
    /// The original random set of bytes.
    proof: String,
}
