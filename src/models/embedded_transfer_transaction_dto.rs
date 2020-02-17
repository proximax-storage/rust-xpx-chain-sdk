use crate::models::entity_dto::EntityType;
use crate::models::uint_64::Uint64Dto;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedTransferTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: EntityType,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    /// If the bit 0 of byte 0 is not set (like in 0x90), then it is a regular address. Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
    #[serde(rename = "recipient")]
    pub recipient: String,
    /// The array of mosaics sent to the recipient. If the most significant bit of byte 0 is set, a namespaceId (alias) is used instead of a instead of a mosaic_id corresponds to a mosaic_id.
    #[serde(rename = "mosaics")]
    pub mosaics: Vec<crate::models::mosaic::MosaicDto>,
    #[serde(rename = "message")]
    pub message: crate::models::message::MessageDto,
}

impl EmbeddedTransferTransactionDto {
    pub fn new(signer: String, version: i32, _type: EntityType, max_fee: Uint64Dto, deadline: Uint64Dto, recipient: String, mosaics: Vec<crate::models::mosaic::MosaicDto>, message: crate::models::message::MessageDto) -> EmbeddedTransferTransactionDto {
        EmbeddedTransferTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            recipient,
            mosaics,
            message,
        }
    }
}


