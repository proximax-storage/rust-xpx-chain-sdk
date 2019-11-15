#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetaDto {
    #[serde(rename = "id")]
    pub id: String,
}

impl MosaicMetaDto {
    pub fn new(id: String) -> MosaicMetaDto {
        MosaicMetaDto {
            id,
        }
    }
}


