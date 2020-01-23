#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// The address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "networkType")]
    pub network_type: crate::models::MetadataTypeEnum,
}

//impl Address {
//    pub fn from_public_key(public_key: String, network_type: crate::models::MetadataTypeEnum) -> Address {
//        Address {
//            address,
//            network_type,
//        }
//    }
//}
