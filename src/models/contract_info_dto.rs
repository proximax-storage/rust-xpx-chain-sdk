#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractInfoDto {
    #[serde(rename = "contract")]
    pub contract: crate::models::ContractDto,
}

impl ContractInfoDto {
    pub fn new(contract: crate::models::ContractDto) -> ContractInfoDto {
        ContractInfoDto {
            contract,
        }
    }
}


