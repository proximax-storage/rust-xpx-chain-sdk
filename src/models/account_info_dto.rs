#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::AccountMetaDto,
    #[serde(rename = "account")]
    pub account: crate::models::AccountDto,
}

impl AccountInfoDto {
    pub fn new(meta: crate::models::AccountMetaDto, account: crate::models::AccountDto) -> AccountInfoDto {
        AccountInfoDto {
            meta,
            account,
        }
    }
}


