/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::{any::Any, fmt},
    downcast_rs::Downcast,
    serde_json::Value,
};

use crate::models::{
    account::{Account, PublicAccount},
    consts::{SIGNATURE_SIZE, SIGNER_SIZE},
    uint_64::Uint64,
};

use super::{
    deadline::Deadline, AbstractTransaction, HashValue, SignedTransaction, TransactionType,
};

pub type Amount = Uint64;

pub type Height = Uint64;

pub type Duration = Uint64;

pub type TransactionsStatus = Vec<TransactionStatus>;

pub type Transactions = Vec<Box<dyn Transaction>>;

pub(crate) struct AbsVector<'b> {
    pub signature_vec: fb::WIPOffset<fb::Vector<'b, u8>>,
    pub signer_vec: fb::WIPOffset<fb::Vector<'b, u8>>,
    pub version_vec: fb::UOffsetT,
    pub type_vec: u16,
    pub max_fee_vec: fb::WIPOffset<fb::Vector<'b, u32>>,
    pub deadline_vec: fb::WIPOffset<fb::Vector<'b, u32>>,
}

impl<'b> AbsVector<'b> {
    pub fn build_vector(
        abs: &AbstractTransaction,
        builder: &mut fb::FlatBufferBuilder<'b>,
    ) -> Self {
        let max_fee = match abs.max_fee {
            Some(item) => item,
            _ => Uint64::default(),
        };

        let deadline = match abs.deadline {
            Some(item) => item,
            _ => Deadline::default(),
        };

        let network_type: fb::UOffsetT = *abs.network_type as u32;

        let version_vec = (network_type << 24) + abs.version as fb::UOffsetT;
        let signature_vec = builder.create_vector_direct(&[0u8; SIGNATURE_SIZE]);
        let signer_vec = builder.create_vector_direct(&[0u8; SIGNER_SIZE]);
        let deadline_vec = builder.create_vector_direct(&deadline.to_i32_array());

        let max_fee_vec = builder.create_vector_direct(&max_fee.to_i32_array());

        AbsVector {
            signature_vec,
            signer_vec,
            version_vec,
            type_vec: abs.transaction_type.value(),
            max_fee_vec,
            deadline_vec,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatus {
    pub group: String,
    pub status: String,
    pub hash: HashValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Deadline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Height>,
}

impl TransactionStatus {
    pub fn new(
        group: String,
        status: String,
        hash: HashValue,
        deadline: Option<Deadline>,
        height: Option<Height>,
    ) -> Self {
        TransactionStatus {
            group,
            status,
            hash,
            deadline,
            height,
        }
    }

    pub fn is_success(&self) -> bool {
        self.status == "Success"
    }

    pub fn is_confirmed(&self) -> bool {
        self.is_success() && self.group == "confirmed"
    }

    pub fn is_partial(&self) -> bool {
        self.is_success() && self.group == "partial"
    }
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

pub trait AbsTransaction {
    fn height(&self) -> Height {
        let mut height: Height = Uint64::default();
        if let Some(h) = self.abs_transaction().transaction_info {
            height = h.height
        };
        height
    }

    fn abs_transaction(&self) -> AbstractTransaction;

    fn entity_type(&self) -> TransactionType {
        self.abs_transaction().transaction_type
    }

    fn transaction_hash(&self) -> HashValue {
        self.abs_transaction().get_hash()
    }

    /// Returns `true` if this transaction has missing signatures.
    fn has_missing_signatures(&self) -> bool {
        self.abs_transaction().has_missing_signatures()
    }

    fn is_unconfirmed(&self) -> bool {
        self.abs_transaction().is_unconfirmed()
    }

    fn is_confirmed(&self) -> bool {
        self.abs_transaction().is_confirmed()
    }
}

pub trait Transaction
where
    Self: fmt::Debug + AbsTransaction + Downcast + Sync + erased_serde::Serialize,
{
    fn size(&self) -> usize;

    /// Serialize this transaction object.
    fn as_value(&self) -> Value;

    /// Serialize and sign [Transaction] with the given [Account] and network generationHash and
    /// create a new signed_transaction.
    fn sign_transaction_with(
        self,
        account: Account,
        generation_hash: HashValue,
    ) -> crate::Result<SignedTransaction>;

    /// An abstract method to generate the embedded transaction bytes.
    fn embedded_to_bytes(&self) -> crate::Result<Vec<u8>>;

    fn set_aggregate(&mut self, signer: PublicAccount);

    fn as_any(&self) -> &dyn Any;

    fn box_clone(&self) -> Box<dyn Transaction>;
}

// implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Transaction + 'static> {
    fn clone(&self) -> Box<dyn Transaction + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.embedded_to_bytes().unwrap() == other.embedded_to_bytes().unwrap()
    }
}

impl fmt::Display for dyn Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

impl_downcast!(Transaction);

serialize_trait_object!(Transaction);
