#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network. 
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
}

impl EntityDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum) -> EntityDto {
        EntityDto {
            signer,
            version,
            _type,
        }
    }
}


