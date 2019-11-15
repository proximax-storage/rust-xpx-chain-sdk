#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CosignatureDtoAllOf {
    /// The public key of the transaction signer.
    #[serde(rename = "signer")]
    pub signer: String,
}

impl CosignatureDtoAllOf {
    pub fn new(signer: String) -> CosignatureDtoAllOf {
        CosignatureDtoAllOf {
            signer,
        }
    }
}


