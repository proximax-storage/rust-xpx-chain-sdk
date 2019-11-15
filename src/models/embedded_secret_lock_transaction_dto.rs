#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedSecretLockTransactionDto {
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

impl EmbeddedSecretLockTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, duration: Vec<i32>, mosaic_id: Vec<i32>, amount: Vec<i32>, hash_algorithm: crate::models::HashAlgorithmEnum, secret: String, recipient: String) -> EmbeddedSecretLockTransactionDto {
        EmbeddedSecretLockTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            duration,
            mosaic_id,
            amount,
            hash_algorithm,
            secret,
            recipient,
        }
    }
}


