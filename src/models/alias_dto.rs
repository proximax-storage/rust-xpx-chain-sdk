#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasDto {
    #[serde(rename = "type")]
    pub _type: crate::models::AliasTypeEnum,
    #[serde(rename = "mosaicId", skip_serializing_if = "Option::is_none")]
    pub mosaic_id: Option<Vec<i32>>,
    /// The aliased address in hexadecimal.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl AliasDto {
    pub fn new(_type: crate::models::AliasTypeEnum) -> AliasDto {
        AliasDto {
            _type,
            mosaic_id: None,
            address: None,
        }
    }
}


