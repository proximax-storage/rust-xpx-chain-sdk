#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedMosaicSupplyChangeTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network. 
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "direction")]
    pub direction: crate::models::MosaicDirectionEnum,
    #[serde(rename = "delta")]
    pub delta: Vec<i32>,
}

impl EmbeddedMosaicSupplyChangeTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, mosaic_id: Vec<i32>, direction: crate::models::MosaicDirectionEnum, delta: Vec<i32>) -> EmbeddedMosaicSupplyChangeTransactionDto {
        EmbeddedMosaicSupplyChangeTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            mosaic_id,
            direction,
            delta,
        }
    }
}


