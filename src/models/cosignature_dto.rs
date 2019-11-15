#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CosignatureDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node. 
    #[serde(rename = "signature")]
    pub signature: String,
    /// The public key of the transaction signer.
    #[serde(rename = "signer")]
    pub signer: String,
}

impl CosignatureDto {
    pub fn new(signature: String, signer: String) -> CosignatureDto {
        CosignatureDto {
            signature,
            signer,
        }
    }
}


