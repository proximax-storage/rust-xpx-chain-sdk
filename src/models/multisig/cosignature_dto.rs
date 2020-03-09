use crate::models::account::PublicAccount;
use crate::models::multisig::{CosignatoryModification, Cosignature, MultisigModificationType};
use crate::models::network::NetworkType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CosignatoryModificationDto {
    modification_type: u8,
    cosignatory_public_key: String,
}

impl CosignatoryModificationDto {
    pub fn to_struct(&self, network_type: NetworkType) -> CosignatoryModification {
        let public_account = PublicAccount::from_public_key(
            &self.cosignatory_public_key, network_type).unwrap();

        CosignatoryModification {
            modification_type: MultisigModificationType::from(self.modification_type),
            public_account
        }
    }
}

//#[derive(Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub(crate) struct CosignatureDto {
//    /// The hash of parent aggregate transaction that has been signed by a cosignatory of the transaction.
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub parent_hash: Option<String>,
//    /// The signatures generated by signing the parent aggregate transaction hash.
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub signature: Option<String>,
//    /// The signer of the transaction.
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub signer: Option<String>,
//}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosignatureDto {
    /// The signature of the entity.
    /// The signature was generated by the signer and can be used to validate tha the entity data
    /// was not modified by a node.
    pub signature: String,
    /// The public account of the cosignatory.
    pub signer: String,
}

impl CosignatureDto {
    pub fn to_struct(&self, network_type: NetworkType) -> Cosignature {
        let signer = PublicAccount::from_public_key(
            &self.signer, network_type).unwrap();

        Cosignature { signature: (&self.signature).parse().unwrap(), signer }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CosignatureDtoAllOf {
    /// The public key of the transaction signer.
    pub signer: String,
}
