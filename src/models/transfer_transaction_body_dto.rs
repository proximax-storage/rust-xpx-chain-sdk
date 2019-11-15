#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferTransactionBodyDto {
    /// If the bit 0 of byte 0 is not set (like in 0x90), then it is a regular address. Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
    #[serde(rename = "recipient")]
    pub recipient: String,
    /// The array of mosaics sent to the recipient. If the most significant bit of byte 0 is set, a namespaceId (alias) is used instead of a instead of a mosaicId corresponds to a mosaicId.
    #[serde(rename = "mosaics")]
    pub mosaics: Vec<crate::models::MosaicDto>,
    #[serde(rename = "message")]
    pub message: crate::models::MessageDto,
}

impl TransferTransactionBodyDto {
    pub fn new(recipient: String, mosaics: Vec<crate::models::MosaicDto>, message: crate::models::MessageDto) -> TransferTransactionBodyDto {
        TransferTransactionBodyDto {
            recipient,
            mosaics,
            message,
        }
    }
}


