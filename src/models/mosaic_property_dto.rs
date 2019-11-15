#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicPropertyDto {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::models::MosaicPropertyIdEnum>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<i32>>,
}

impl MosaicPropertyDto {
    pub fn new() -> MosaicPropertyDto {
        MosaicPropertyDto {
            id: None,
            value: None,
        }
    }
}


