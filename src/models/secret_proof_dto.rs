use crate::models::hash_lock_dto::HashAlgorithmEnum;
use crate::models::uint_64::Uint64Dto;

#[derive(Serialize, Deserialize)]
pub(crate) struct SecretProofTransactionBodyDto {
    pub hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    pub secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The original random set of bytes.
    pub proof: String,
}

/// SecretProofTransactionDto : Transaction that revealed a proof.
#[derive(Serialize, Deserialize)]
pub(crate) struct SecretProofTransactionDto {
    pub signature: String,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
    pub hash_algorithm: HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    pub secret: String,
    /// The address in hexadecimal that received the funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The original random set of bytes.
    pub proof: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbeddedSecretProofTransactionDto {
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
    pub hash_algorithm: HashAlgorithmEnum,
    pub secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The original random set of bytes.
    pub proof: String,
}
