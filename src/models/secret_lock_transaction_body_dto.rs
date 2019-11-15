#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretLockTransactionBodyDto {
    #[serde(rename = "duration")]
    pub duration: Vec<i32>,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: crate::models::HashAlgorithmEnum,
    /// The proof hashed.
    #[serde(rename = "secret")]
    pub secret: String,
    /// The address in hexadecimal that will receive the funds once the transaction is unlocked.
    #[serde(rename = "recipient")]
    pub recipient: String,
}

impl SecretLockTransactionBodyDto {
    pub fn new(duration: Vec<i32>, mosaic_id: Vec<i32>, amount: Vec<i32>, hash_algorithm: crate::models::HashAlgorithmEnum, secret: String, recipient: String) -> SecretLockTransactionBodyDto {
        SecretLockTransactionBodyDto {
            duration,
            mosaic_id,
            amount,
            hash_algorithm,
            secret,
            recipient,
        }
    }
}


