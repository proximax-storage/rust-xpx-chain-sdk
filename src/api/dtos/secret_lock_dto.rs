/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{AbstractTransactionDto, HashAlgorithmEnum, Uint64Dto};

/// SecretLockTransactionDto : Transaction that sends mosaics to a recipient if the proof used is revealed. If the duration is reached, the locked funds go back to the sender of the transaction.
#[derive(Serialize, Deserialize)]
pub(crate) struct SecretLockTransactionDto {
    #[serde(flatten)]
    pub r#abstract: AbstractTransactionDto,
    #[serde(rename = "duration")]
    duration: Uint64Dto,
    #[serde(rename = "mosaic_id")]
    mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
    #[serde(rename = "hashAlgorithm")]
    hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    secret: String,
    /// The address in hexadecimal that will receive the funds once the transaction is unlocked.
    #[serde(rename = "recipient")]
    recipient: String,
}
