#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::BlockMetaDto,
    #[serde(rename = "block")]
    pub block: crate::models::BlockDto,
}

impl BlockInfoDto {
    pub fn new(meta: crate::models::BlockMetaDto, block: crate::models::BlockDto) -> BlockInfoDto {
        BlockInfoDto {
            meta,
            block,
        }
    }
}


