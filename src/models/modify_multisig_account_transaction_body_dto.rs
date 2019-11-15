#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyMultisigAccountTransactionBodyDto {
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

impl ModifyMultisigAccountTransactionBodyDto {
    pub fn new(min_removal_delta: i32, min_approval_delta: i32, modifications: Vec<crate::models::CosignatoryModificationDto>) -> ModifyMultisigAccountTransactionBodyDto {
        ModifyMultisigAccountTransactionBodyDto {
            min_removal_delta,
            min_approval_delta,
            modifications,
        }
    }
}


