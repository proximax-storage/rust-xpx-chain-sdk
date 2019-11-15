#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractDto {
    #[serde(rename = "multisig")]
    pub multisig: String,
    #[serde(rename = "multisigAddress")]
    pub multisig_address: String,
    #[serde(rename = "start")]
    pub start: Vec<i32>,
    #[serde(rename = "duration")]
    pub duration: Vec<i32>,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "customers")]
    pub customers: Vec<String>,
    #[serde(rename = "executors")]
    pub executors: Vec<String>,
    #[serde(rename = "verifiers")]
    pub verifiers: Vec<String>,
}

impl ContractDto {
    pub fn new(multisig: String, multisig_address: String, start: Vec<i32>, duration: Vec<i32>, hash: String, customers: Vec<String>, executors: Vec<String>, verifiers: Vec<String>) -> ContractDto {
        ContractDto {
            multisig,
            multisig_address,
            start,
            duration,
            hash,
            customers,
            executors,
            verifiers,
        }
    }
}


