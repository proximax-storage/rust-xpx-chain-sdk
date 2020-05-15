#[derive(Serialize, Deserialize)]
pub struct ContractDto {
    multisig: String,
    multisig_address: String,
    start: Vec<i32>,
    duration: Vec<i32>,
    hash: String,
    customers: Vec<String>,
    executors: Vec<String>,
    verifiers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ContractInfoDto {
    #[serde(rename = "contract")]
    contract: ContractDto,
}