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

pub type Amount = Uint64;

pub type Height = Uint64;

pub type Hash = String;

pub type Transactions = Vec<Box<dyn Transaction>>;

pub trait AbsTransaction {
    fn transaction_hash(&self) -> &str;

    /// Returns `true` if this transaction has missing signatures.
    fn has_missing_signatures(&self) -> bool;

    fn is_unconfirmed(&self) -> bool;

    fn is_confirmed(&self) -> bool;

    fn abs_transaction(&self) -> AbstractTransaction;
}

pub trait Transaction: AbsTransaction + Downcast + Sync + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn size(&self) -> usize;

    /// Serialize this transaction object.
    fn to_json(&self) -> Value;

    /// Serialize and sign [Transaction] with the given [Account] and network generationHash and
    /// create a new signed_transaction.
    fn sign_transaction_with(self, account: Account, generation_hash: String) -> crate::Result<SignedTransaction>;

    /// An abstract method to generate the embedded transaction bytes.
    fn embedded_to_bytes(&self) -> Vec<u8>;

    fn entity_type(&self) -> EntityTypeEnum;

    fn as_any(&self) -> &dyn Any;
}

impl_downcast!(Transaction);

serialize_trait_object!(Transaction);

impl<'a> PartialEq for &'a dyn Transaction {
    fn eq(&self, other: &Self) -> bool {
        &self.embedded_to_bytes() == &other.embedded_to_bytes()
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
    pub height: Option<Height>,
}

impl TransactionStatus {
    pub fn new(group: String, status: String, hash: String, deadline: Option<Deadline>, height: Option<Height>) -> Self {
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
