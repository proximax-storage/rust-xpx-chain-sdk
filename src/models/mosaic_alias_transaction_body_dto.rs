#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicAliasTransactionBodyDto {
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
}

impl MosaicAliasTransactionBodyDto {
    pub fn new(alias_action: crate::models::AliasActionEnum, namespace_id: Vec<i32>, mosaic_id: Vec<i32>) -> MosaicAliasTransactionBodyDto {
        MosaicAliasTransactionBodyDto {
            alias_action,
            namespace_id,
            mosaic_id,
        }
    }
}


