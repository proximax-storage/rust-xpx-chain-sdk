#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::MosaicMetaDto,
    #[serde(rename = "mosaic")]
    pub mosaic: crate::models::MosaicDefinitionDto,
}

impl MosaicInfoDto {
    pub fn new(meta: crate::models::MosaicMetaDto, mosaic: crate::models::MosaicDefinitionDto) -> MosaicInfoDto {
        MosaicInfoDto {
            meta,
            mosaic,
        }
    }
}


