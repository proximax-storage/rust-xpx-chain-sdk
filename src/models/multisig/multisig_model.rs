use crate::models::errors::ERR_UNKNOWN_TYPE;
use crate::models::account::PublicAccount;

/// MultisigModificationTypeEnum :
/// The type of the modification:
/// * 0 - Add cosignatory.
/// * 1 - Remove cosignatory.
#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum MultisigModificationType {
    Add,
    Remove
}

impl MultisigModificationType {
    pub fn value(&self) -> u8 {
        match self {
            MultisigModificationType::Add => 0,
            MultisigModificationType::Remove => 1,
        }
    }
}

impl From<u8> for MultisigModificationType {
    fn from(t: u8) -> Self {
        assert!(t <= 1, ERR_UNKNOWN_TYPE);
        match t {
            0 => MultisigModificationType::Add,
            _ => MultisigModificationType::Remove
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CosignatoryModification {
    #[serde(rename = "modificationType")]
    pub modification_type: MultisigModificationType,
    /// The public key of the cosignatory account.
    #[serde(rename = "cosignatory_public_key")]
    pub public_account: PublicAccount,
}

impl CosignatoryModification {
    pub fn new(
        modification_type: MultisigModificationType,
        public_account: PublicAccount
    ) -> Self {
        CosignatoryModification { modification_type, public_account }
    }
}