/// MetadataTypeEnum : The type of the metadata: * 1 - Address metadata. * 2 - Mosaic metadata. * 3 - Namespace metadata.

/// The type of the metadata: * 1 - Address metadata. * 2 - Mosaic metadata. * 3 - Namespace metadata. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MetadataTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,

}




