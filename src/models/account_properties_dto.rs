#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesDto {
    /// The address of the account in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::AccountPropertyDto>,
}

impl AccountPropertiesDto {
    pub fn new(address: String, properties: Vec<crate::models::AccountPropertyDto>) -> AccountPropertiesDto {
        AccountPropertiesDto {
            address,
            properties,
        }
    }
}


