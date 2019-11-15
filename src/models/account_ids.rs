#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountIds {
    /// The array of public keys.
    #[serde(rename = "publicKeys", skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    /// The array of addresses.
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
}

impl AccountIds {
    pub fn new() -> AccountIds {
        AccountIds {
            public_keys: None,
            addresses: None,
        }
    }
}


