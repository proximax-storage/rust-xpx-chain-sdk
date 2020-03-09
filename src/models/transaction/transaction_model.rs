use ::std::{any::Any, fmt};

use downcast_rs::Downcast;
use serde_json::Value;

use crate::models::{account::Account, Uint64};

use super::{
    AbstractTransaction,
    deadline::Deadline,
    EntityTypeEnum,
    SignedTransaction,
};

pub type Height = Uint64;

pub type Hash = String;

pub type Transactions = Vec<Box<dyn Transaction>>;

pub trait Transaction: Downcast + Sync + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn transaction_hash(&self) -> &str;

    fn abs_transaction(&self) -> AbstractTransaction;

    fn size(&self) -> usize;

    /// An abstract method to generate the transaction bytes.
    fn generate_bytes(&self) -> Vec<u8>;

    /// An abstract method to generate the embedded transaction bytes.
    fn generate_embedded_bytes(&self) -> Vec<u8>;

    /// Serialize this transaction object.
    fn to_json(&self) -> Value;

    /// Returns `true` if this transaction has missing signatures.
    fn has_missing_signatures(&self) -> bool;

    /// Serialize and sign 'Transaction' with the given 'Account' and network generationHash and
    /// create a new SignedTransaction.
    fn sign_transaction_with(self, account: Account, generation_hash: String) -> crate::Result<SignedTransaction>;

    fn entity_type(&self) -> EntityTypeEnum;

    fn as_any(&self) -> &dyn Any;
}

impl_downcast!(Transaction);

serialize_trait_object!(Transaction);

impl<'a> PartialEq for &'a dyn Transaction {
    fn eq(&self, other: &Self) -> bool {
        &self.generate_bytes() == &other.generate_bytes()
    }
}

impl fmt::Display for dyn Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatus {
    pub group: String,
    pub status: String,
    pub hash: Hash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Deadline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Uint64>,
}

impl TransactionStatus {
    pub fn new(group: String, status: String, hash: String, deadline: Option<Deadline>, height: Option<Uint64>) -> Self {
        TransactionStatus {
            group,
            status,
            hash,
            deadline,
            height,
        }
    }
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
