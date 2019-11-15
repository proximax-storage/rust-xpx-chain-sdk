#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertyDto {
    #[serde(rename = "propertyType")]
    pub property_type: crate::models::AccountPropertyTypeEnum,
    /// The address, transaction type or mosaic id to filter.
    #[serde(rename = "values")]
    pub values: Vec<crate::models::OneOfstringarrayinteger>,
}

impl AccountPropertyDto {
    pub fn new(property_type: crate::models::AccountPropertyTypeEnum, values: Vec<crate::models::OneOfstringarrayinteger>) -> AccountPropertyDto {
        AccountPropertyDto {
            property_type,
            values,
        }
    }
}


