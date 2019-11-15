/// MetadataModificationTypeEnum : The type of the metadata modification: * 0 - Add metadata. * 1 - Remove metadata.

/// The type of the metadata modification: * 0 - Add metadata. * 1 - Remove metadata. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MetadataModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}




