#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedModifyMultisigAccountTransactionDto {
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
    /// The number of signatures needed to remove a cosignatory. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories. 
    #[serde(rename = "minRemovalDelta")]
    pub min_removal_delta: i32,
    /// The number of signatures needed to approve a transaction. If we are modifying an existing multisig account, this indicates the relative change of the minimum cosignatories. 
    #[serde(rename = "minApprovalDelta")]
    pub min_approval_delta: i32,
    /// The array of cosignatory accounts to add or delete.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::CosignatoryModificationDto>,
}

impl EmbeddedModifyMultisigAccountTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, min_removal_delta: i32, min_approval_delta: i32, modifications: Vec<crate::models::CosignatoryModificationDto>) -> EmbeddedModifyMultisigAccountTransactionDto {
        EmbeddedModifyMultisigAccountTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            min_removal_delta,
            min_approval_delta,
            modifications,
        }
    }
}


