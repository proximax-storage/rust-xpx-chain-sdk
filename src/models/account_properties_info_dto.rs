#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesInfoDto {
    #[serde(rename = "accountProperties")]
    pub account_properties: crate::models::AccountPropertiesDto,
}

impl AccountPropertiesInfoDto {
    pub fn new(account_properties: crate::models::AccountPropertiesDto) -> AccountPropertiesInfoDto {
        AccountPropertiesInfoDto {
            account_properties,
        }
    }
}


