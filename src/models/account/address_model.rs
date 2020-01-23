#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// The address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "networkType")]
    pub network_type: u8,
}

impl Address {
    pub fn new(metadata_id: String, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> AddressMetadataBodyDto {
        Address {
            address,
            network_type,
        }
    }
}

impl Address {
    pub fn from_public_key(public_key: String, network_type: u8) -> Address {
        Address {
            address,
            network_type,
        }
    }
}
