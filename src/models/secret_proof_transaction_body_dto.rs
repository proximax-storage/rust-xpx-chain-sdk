#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretProofTransactionBodyDto {
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: crate::models::HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    pub secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The original random set of bytes.
    #[serde(rename = "proof")]
    pub proof: String,
}

impl SecretProofTransactionBodyDto {
    pub fn new(hash_algorithm: crate::models::HashAlgorithmEnum, secret: String, proof: String) -> SecretProofTransactionBodyDto {
        SecretProofTransactionBodyDto {
            hash_algorithm,
            secret,
            recipient: None,
            proof,
        }
    }
}


