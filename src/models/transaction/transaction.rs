/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use sha3::{Digest, Sha3_256};

use ::std::fmt;
use std::any::Any;
use serde_json::Value;
use crate::account::{Account, PublicAccount};

use crate::anyhow::Result;
use crate::{AsUint64, GenerationHash};
use crate::helpers::{hex_decode, hex_encode, TransactionHash};
use crate::models::consts::{HALF_OF_SIGNATURE, SIGNATURE_SIZE, SIGNER_SIZE, SIZE_SIZE};
use crate::transaction::{SignedTransaction, TransactionType};

use super::{
    CommonTransaction, deadline::Deadline, TransactionStatus,
};

pub type Amount = u64;

pub type Height = u64;

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
        common: &CommonTransaction,
        builder: &mut fb::FlatBufferBuilder<'b>,
    ) -> Self {
        let max_fee = match common.max_fee {
            Some(item) => item,
            _ => u64::default(),
        };

        let deadline = match common.deadline {
            Some(item) => item,
            _ => Deadline::default(),
        };

        let network_type: fb::UOffsetT = common.network_type.value() as u32;

        let version_vec = (network_type << 24) + *common.version as fb::UOffsetT;
        let signature_vec = builder.create_vector(&[0u8; SIGNATURE_SIZE]);
        let signer_vec = builder.create_vector(&[0u8; SIGNER_SIZE]);
        let deadline_vec = builder.create_vector(&deadline.to_dto());

        let max_fee_vec = builder.create_vector(&max_fee.to_dto());

        AbsVector {
            signature_vec,
            signer_vec,
            version_vec,
            type_vec: common.transaction_type.value(),
            max_fee_vec,
            deadline_vec,
        }
    }
}

/// An abstract transaction trait that serves as the base of all transaction types.
///
#[typetag::serde]
pub trait Transaction
    where
        Self: fmt::Debug + 'static + Sync + Send,
{
    fn size(&self) -> usize;

    /// Serialize this transaction object.
    fn as_value(&self) -> Value;

    /// Get the `CommonTransaction`.
    fn get_common_transaction(&self) -> CommonTransaction;

    /// Get the `TransactionType`.
    fn get_transaction_type(&self) -> TransactionType {
        self.get_common_transaction().transaction_type
    }

    /// Get the `TransactionHash`.
    fn get_transaction_hash(&self) -> TransactionHash {
        self.get_common_transaction().get_hash()
    }

    /// Generate signing bytes.
    ///
    /// # Inputs
    ///
    /// * `payload_bytes`: Payload buffer.
    /// * `generation_hash_bytes`: GenerationHash buffer.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>`.
    fn get_signing_bytes(&self, payload_bytes: &[u8], generation_hash_bytes: &[u8]) -> Vec<u8> {
        let byte_buffer_without_header = &payload_bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..];
        if self.get_transaction_type() == TransactionType::AggregateBonded
            || self.get_transaction_type() == TransactionType::AggregateComplete
        {
            [generation_hash_bytes[..32].as_ref(), byte_buffer_without_header].concat()
        } else {
            [generation_hash_bytes, byte_buffer_without_header].concat()
        }
    }

    /// Serialize and sign transaction creating a SignedTransaction.
    ///
    /// # Inputs
    ///
    /// * `account`: The account to sign the transaction.
    /// * `generation_hash`: Network generation hash hex.
    ///
    /// # Returns
    ///
    /// A Symbol `SignedTransaction`.
    fn sign_with(
        &self,
        account: Account,
        generation_hash: GenerationHash,
    ) -> Result<SignedTransaction> {
        let generation_hash_bytes = generation_hash.to_fixed_bytes();
        let transaction_buffer = self.to_serializer();

        let signing_bytes =
            self.get_signing_bytes(transaction_buffer.as_ref(), generation_hash_bytes.as_ref());

        let signature = account.key_pair.sign(signing_bytes.as_ref());

        let mut signed_transaction_buffer = Vec::with_capacity(transaction_buffer.len());
        signed_transaction_buffer.extend_from_slice(transaction_buffer[..4].as_ref());
        signed_transaction_buffer.extend_from_slice(&signature.to_bytes());
        signed_transaction_buffer.extend_from_slice(account.public_account.public_key.as_ref());
        signed_transaction_buffer.extend_from_slice(
            transaction_buffer[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..transaction_buffer.len()]
                .as_ref(),
        );
        let payload = hex_encode(signed_transaction_buffer.as_ref());
        let transaction_hash =
            <dyn Transaction>::create_transaction_hash(&payload, &generation_hash_bytes);

        Ok(SignedTransaction {
            payload: payload.to_uppercase(),
            hash: transaction_hash,
            entity_type: self.get_transaction_type(),
        })
    }

    /// An abstract method to generate the embedded transaction bytes.
    fn to_serializer(&self) -> Vec<u8>;

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.get_common_transaction().set_aggregate(signer)
    }

    /// Transaction pending to be included in a block.
    fn is_unconfirmed(&self) -> bool {
        self.get_common_transaction().is_unconfirmed()
    }

    /// Transaction included in a block.
    fn is_confirmed(&self) -> bool {
        self.get_common_transaction().is_confirmed()
    }

    /// if a transaction has missing signatures.
    fn has_missing_signatures(&self) -> bool {
        self.get_common_transaction().has_missing_signatures()
    }

    /// Transaction is not known by the network
    fn is_unannounced(&self) -> bool {
        self.get_common_transaction().is_unannounced()
    }

    fn as_any(&self) -> &dyn Any;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    fn box_clone(&self) -> Box<dyn Transaction>;
}

impl dyn Transaction {
    /// Transaction header size
    ///
    /// Included fields are `size`, `signer_public_key` and `signature`.
    ///
    const HEADER_SIZE: usize = 4 + 32 + 64;

    /// Index of the transaction *type*
    ///
    /// Included fields are the transaction header, `version` and `network`
    const TYPE_INDEX: usize = Self::HEADER_SIZE + 2;

    /// Index of the transaction *body*
    ///
    /// Included fields are the transaction header, `version`, `type`, `maxFee` and `deadline`
    const BODY_INDEX: usize = Self::HEADER_SIZE + 4 + 2 + 8 + 8;

    /// Generate transaction hash hex.
    ///
    /// # Inputs
    ///
    /// * `transaction_payload`: HexString Payload.
    /// * `generation_hash`: Network generation hash byte.
    ///
    /// # Returns
    ///
    /// A Transaction Payload hash.
    fn create_transaction_hash(
        transaction_payload: &str,
        generation_hash: &[u8],
    ) -> TransactionHash {
        let mut transaction_hash = TransactionHash::zero();

        let transaction_bytes = hex_decode(transaction_payload);

        // read transaction type
        static TYPE_IDX: usize = <dyn Transaction>::TYPE_INDEX;

        let mut sb = vec![];

        sb.extend_from_slice(&transaction_bytes[SIZE_SIZE..SIZE_SIZE + HALF_OF_SIGNATURE]);

        sb.extend_from_slice(
            &transaction_bytes
                [SIGNATURE_SIZE + SIZE_SIZE..SIZE_SIZE + SIGNATURE_SIZE + SIGNER_SIZE],
        );

        sb.extend_from_slice(generation_hash);

        sb.extend_from_slice(&transaction_bytes[100..]);

        let sha3_hash = Sha3_256::digest(sb.as_slice());

        transaction_hash.assign_from_slice(&sha3_hash.as_slice()[0..32]);
        transaction_hash
    }

    /// Downcast a reference to this generic [`Transaction`] to a specific transaction type.
    ///
    /// # Panics
    ///
    /// Panics if the transaction type is not `T`. In normal usage, you should know the
    /// specific Transaction type. In other cases, use `try_downcast_ref`.
    ///
    pub fn downcast_ref<T: Transaction>(&self) -> &T {
        self.try_downcast_ref::<T>().unwrap_or_else(|| {
            panic!(
                "downcast to wrong Transaction type; original transaction type: {}",
                self.get_transaction_type()
            )
        })
    }

    /// Downcast a reference to this generic [`Transaction`] to a specific transaction type.
    #[inline]
    pub fn try_downcast_ref<T: Transaction>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /// Downcast this generic [`Transaction`] to a specific transaction type.
    ///
    /// # Panics
    ///
    /// Panics if the transaction type is not `T`. In normal usage, you should know the
    /// specific transaction type. In other cases, use `try_downcast`.
    ///
    pub fn downcast<T: Transaction>(self: Box<Self>) -> Box<T> {
        self.try_downcast().unwrap_or_else(|err| panic!("{}", err))
    }

    /// Downcast this generic [`Transaction`] to a specific transaction type.
    #[inline]
    pub fn try_downcast<T: Transaction>(self: Box<Self>) -> Result<Box<T>> {
        if self.as_ref().as_any().is::<T>() {
            Ok(self.into_any().downcast().unwrap())
        } else {
            Err(anyhow!(
				"downcast to wrong Transaction type; original transaction type: {}",
				self.get_transaction_type()
			))
        }
    }
}

// implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Transaction + 'static> {
    fn clone(&self) -> Box<dyn Transaction + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.to_serializer() == other.to_serializer()
    }
}

impl fmt::Display for dyn Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
