#[derive(Serialize, Deserialize)]
pub struct ContractDto {
    pub multisig: String,
    pub multisig_address: String,
    pub start: Vec<i32>,
    pub duration: Vec<i32>,
    pub hash: String,
    pub customers: Vec<String>,
    pub executors: Vec<String>,
    pub verifiers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ContractInfoDto {
    #[serde(rename = "contract")]
    pub contract: ContractDto,
}