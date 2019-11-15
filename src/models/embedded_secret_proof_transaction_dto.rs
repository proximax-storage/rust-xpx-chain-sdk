#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedSecretProofTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network. 
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
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

impl EmbeddedSecretProofTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, hash_algorithm: crate::models::HashAlgorithmEnum, secret: String, proof: String) -> EmbeddedSecretProofTransactionDto {
        EmbeddedSecretProofTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            hash_algorithm,
            secret,
            recipient: None,
            proof,
        }
    }
}


