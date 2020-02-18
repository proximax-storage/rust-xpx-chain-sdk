use crate::models::uint_64::Uint64Dto;
use crate::models::message::MessageDto;
use crate::models::mosaic::MosaicDto;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedTransferTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    /// If the bit 0 of byte 0 is not set (like in 0x90), then it is a regular address. Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
    #[serde(rename = "recipient")]
    pub recipient: String,
    /// The array of mosaics sent to the recipient. If the most significant bit of byte 0 is set, a namespaceId (alias) is used instead of a instead of a mosaic_id corresponds to a mosaic_id.
    #[serde(rename = "mosaics")]
    pub mosaics: Vec<MosaicDto>,
    #[serde(rename = "message")]
    pub message: MessageDto,
}


