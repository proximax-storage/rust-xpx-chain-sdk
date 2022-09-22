/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{ReceiptType, Uint64Dto};

/// BalanceChangeReceiptDto : The invisible state change changed an account balance.
#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceChangeReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    version: i32,
    #[serde(rename = "type")]
    _type: ReceiptType,
    /// The target account public key.
    #[serde(rename = "account")]
    account: String,
    #[serde(rename = "mosaic_id")]
    mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceChangeReceiptDtoAllOf {
    /// The target account public key.
    account: String,
    mosaic_id: Uint64Dto,
    amount: Uint64Dto,
}

/// BalanceTransferReceiptDto : The invisible state change triggered a mosaic transfer.
#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceTransferReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    version: i32,
    #[serde(rename = "type")]
    _type: ReceiptType,
    /// The public key of the sender.
    #[serde(rename = "sender")]
    sender: String,
    /// The public key of the recipient.
    #[serde(rename = "recipient")]
    recipient: String,
    #[serde(rename = "mosaic_id")]
    mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceTransferReceiptDtoAllOf {
    /// The public key of the sender.
    #[serde(rename = "sender")]
    sender: String,
    /// The public key of the recipient.
    #[serde(rename = "recipient")]
    recipient: String,
    #[serde(rename = "mosaic_id")]
    mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
}
