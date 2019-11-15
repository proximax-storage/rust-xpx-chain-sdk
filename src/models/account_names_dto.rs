#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountNamesDto {
    /// The address of the account in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    /// The mosaic linked namespace names.
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

impl AccountNamesDto {
    pub fn new(address: String, names: Vec<String>) -> AccountNamesDto {
        AccountNamesDto {
            address,
            names,
        }
    }
}


