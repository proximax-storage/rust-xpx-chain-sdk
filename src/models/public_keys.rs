#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeys {
    #[serde(rename = "publicKeys", skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
}

impl PublicKeys {
    pub fn new() -> PublicKeys {
        PublicKeys {
            public_keys: None,
        }
    }
}


