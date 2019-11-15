#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultisigAccountInfoDto {
    #[serde(rename = "multisig")]
    pub multisig: crate::models::MultisigDto,
}

impl MultisigAccountInfoDto {
    pub fn new(multisig: crate::models::MultisigDto) -> MultisigAccountInfoDto {
        MultisigAccountInfoDto {
            multisig,
        }
    }
}


