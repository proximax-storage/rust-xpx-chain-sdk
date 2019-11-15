#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesTransactionBodyDto {
    #[serde(rename = "propertyType")]
    pub property_type: crate::models::AccountPropertyTypeEnum,
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::AccountPropertiesModificationDto>,
}

impl AccountPropertiesTransactionBodyDto {
    pub fn new(property_type: crate::models::AccountPropertyTypeEnum, modifications: Vec<crate::models::AccountPropertiesModificationDto>) -> AccountPropertiesTransactionBodyDto {
        AccountPropertiesTransactionBodyDto {
            property_type,
            modifications,
        }
    }
}


