use crate::models::receipt_dto::ReceiptTypeEnum;
use crate::models::uint_64::Uint64Dto;

/// BalanceChangeReceiptDto : The invisible state change changed an account balance.
#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceChangeReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: u32,
    #[serde(rename = "type")]
    pub _type: ReceiptTypeEnum,
    /// The target account public key.
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    pub amount: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceChangeReceiptDtoAllOf {
    /// The target account public key.
    pub account: String,
    pub mosaic_id: Uint64Dto,
    pub amount: Uint64Dto,
}

/// BalanceTransferReceiptDto : The invisible state change triggered a mosaic transfer.
#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceTransferReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: u32,
    #[serde(rename = "type")]
    pub _type: ReceiptTypeEnum,
    /// The public key of the sender.
    #[serde(rename = "sender")]
    pub sender: String,
    /// The public key of the recipient.
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    pub amount: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BalanceTransferReceiptDtoAllOf {
    /// The public key of the sender.
    #[serde(rename = "sender")]
    pub sender: String,
    /// The public key of the recipient.
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "amount")]
    pub amount: Uint64Dto,
}
