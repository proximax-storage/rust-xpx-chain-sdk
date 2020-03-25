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
use crate::models::account::PublicAccount;

pub type Amount = Uint64;

pub type Height = Uint64;

pub type Duration = Uint64;

pub type Hash = String;

pub type Transactions = Vec<Box<dyn Transaction>>;

pub type TransactionsStatus = Vec<TransactionStatus>;

pub trait AbsTransaction {
    fn abs_transaction(&self) -> AbstractTransaction;

    fn entity_type(&self) -> EntityTypeEnum { self.abs_transaction().transaction_type }

    fn transaction_hash(&self) -> Hash { self.abs_transaction().get_hash() }

    /// Returns `true` if this transaction has missing signatures.
    fn has_missing_signatures(&self) -> bool {
        self.abs_transaction().has_missing_signatures()
    }

    fn is_unconfirmed(&self) -> bool { self.abs_transaction().is_unconfirmed() }

    fn is_confirmed(&self) -> bool {
        self.abs_transaction().is_confirmed()
    }
}

pub trait Transaction
    where
        Self: fmt::Debug + AbsTransaction + Downcast + Sync + erased_serde::Serialize
{
    fn size(&self) -> usize;

    /// Serialize this transaction object.
    fn to_json(&self) -> Value;

    /// Serialize and sign [Transaction] with the given [Account] and network generationHash and
    /// create a new signed_transaction.
    fn sign_transaction_with(self, account: Account, generation_hash: String) -> crate::Result<SignedTransaction>;

    /// An abstract method to generate the embedded transaction bytes.
    fn embedded_to_bytes(&self) -> crate::Result<Vec<u8>>;

    fn to_aggregate(&mut self, signer: PublicAccount);

    fn as_any(&self) -> &dyn Any;
}

impl_downcast!(Transaction);

serialize_trait_object!(Transaction);

impl<'a> PartialEq for &'a dyn Transaction {
    fn eq(&self, other: &Self) -> bool {
        &self.embedded_to_bytes().unwrap() == &other.embedded_to_bytes().unwrap()
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

    pub fn is_success(&self) -> bool { self.status == "Success" }

    pub fn is_confirmed(&self) -> bool { self.is_success() && self.group == "confirmed" }

    pub fn is_partial(&self) -> bool { self.is_success() && self.group == "partial" }
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
