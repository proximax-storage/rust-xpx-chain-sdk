#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesModificationDto {
    #[serde(rename = "type")]
    pub _type: crate::models::AccountPropertiesModificationTypeEnum,
    /// The address, transaction type or mosaic id to filter.
    #[serde(rename = "values")]
    pub values: Vec<crate::models::OneOfstringarrayinteger>,
}

impl AccountPropertiesModificationDto {
    pub fn new(_type: crate::models::AccountPropertiesModificationTypeEnum, values: Vec<crate::models::OneOfstringarrayinteger>) -> AccountPropertiesModificationDto {
        AccountPropertiesModificationDto {
            _type,
            values,
        }
    }
}


