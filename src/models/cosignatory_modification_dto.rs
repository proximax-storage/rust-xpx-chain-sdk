#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CosignatoryModificationDto {
    #[serde(rename = "modificationType")]
    pub modification_type: crate::models::MultisigModificationTypeEnum,
    /// The public key of the cosignatory account.
    #[serde(rename = "cosignatoryPublicKey")]
    pub cosignatory_public_key: String,
}

impl CosignatoryModificationDto {
    pub fn new(modification_type: crate::models::MultisigModificationTypeEnum, cosignatory_public_key: String) -> CosignatoryModificationDto {
        CosignatoryModificationDto {
            modification_type,
            cosignatory_public_key,
        }
    }
}


