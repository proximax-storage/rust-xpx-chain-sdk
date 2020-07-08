/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::fmt, failure::_core::any::Any, serde_json::Value};

use crate::{
    models::{
        account::{Account, Address, PublicAccount},
        consts::{AMOUNT_SIZE, MOSAIC_ID_SIZE, TRANSFER_HEADER_SIZE},
        errors_const,
        message::Message,
        mosaic::Mosaic,
        namespace::{new_address_from_namespace, NamespaceId},
        network::NetworkType,
    },
    AssetId, Result,
};

use super::{
    buffer::transfer::buffers, deadline::Deadline, internal::sign_transaction,
    schema::transfer_transaction_schema, AbsTransaction, AbstractTransaction, EntityTypeEnum,
    SignedTransaction, Transaction, TRANSFER_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransaction {
    pub abs_transaction: AbstractTransaction,
    /// If the bit 0 of byte 0 is not set (like in 0x90), then it is a regular address.
    /// Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
    pub recipient: Address,
    /// The array of mosaics sent to the recipient.
    /// If the most significant bit of byte 0 is set, a
    /// namespace_id (alias) is used instead of a instead of a mosaic_id corresponds to a mosaic_id.
    pub mosaics: Vec<Mosaic>,
    pub message: Box<dyn Message>,
}

impl TransferTransaction {
    pub fn new(
        deadline: Deadline,
        recipient: Address,
        mosaics: Vec<Mosaic>,
        message: impl Message + 'static,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            !recipient.address.is_empty(),
            errors_const::ERR_EMPTY_ADDRESSES
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            TRANSFER_VERSION,
            EntityTypeEnum::Transfer,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            recipient,
            mosaics,
            message: Box::new(message),
        })
    }

    pub fn with_namespace(
        deadline: Deadline,
        recipient: NamespaceId,
        mosaics: Vec<Mosaic>,
        message: impl Message + 'static,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            recipient.to_u64() != 0,
            errors_const::ERR_EMPTY_NAMESPACE_ID
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            TRANSFER_VERSION,
            EntityTypeEnum::Transfer,
            network_type,
        );

        let recipient = new_address_from_namespace(recipient)?;

        Ok(Self {
            abs_transaction: abs_tx,
            recipient,
            mosaics,
            message: Box::new(message),
        })
    }

    pub fn message_size(&self) -> usize {
        self.message.payload_to_bytes().len() + 1
    }
}

impl AbsTransaction for TransferTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for TransferTransaction {
    fn size(&self) -> usize {
        TRANSFER_HEADER_SIZE
            + ((MOSAIC_ID_SIZE + AMOUNT_SIZE) * self.mosaics.len())
            + self.message_size()
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(
        self,
        account: Account,
        generation_hash: String,
    ) -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes<'a>(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        // Create mosaics
        let ml = self.mosaics.len();

        let mut mosaics_buffer: Vec<fb::WIPOffset<buffers::MosaicBuffer<'a>>> =
            Vec::with_capacity(ml);

        for mosaic in self.mosaics.iter() {
            let mosaic_id = _builder.create_vector(&mosaic.asset_id.to_u32_array());
            let mosaic_amount = _builder.create_vector(&mosaic.amount.to_int_array());

            let mut mosaic_buffer = buffers::MosaicBufferBuilder::new(&mut _builder);
            mosaic_buffer.add_id(mosaic_id);
            mosaic_buffer.add_amount(mosaic_amount);

            mosaics_buffer.push(mosaic_buffer.finish());
        }

        // Create message;
        let payload_vec = _builder.create_vector_direct(self.message.payload_to_bytes());

        let mut message_buffer = buffers::MessageBufferBuilder::new(&mut _builder);
        message_buffer.add_type_(self.message.message_type().value());
        message_buffer.add_payload(payload_vec);
        let message_vec = message_buffer.finish();

        let recipient = self.recipient.to_decode();

        let recipient_vec = _builder.create_vector_direct(&recipient);

        let mosaic_vec = _builder.create_vector(&mosaics_buffer);

        let abs_vector = self.abs_transaction.build_vector(&mut _builder);

        let mut txn_builder = buffers::TransferTransactionBufferBuilder::new(&mut _builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_recipient(recipient_vec);
        txn_builder.add_num_mosaics(ml as u8);
        txn_builder.add_message_size(self.message_size() as u16);
        txn_builder.add_message(message_vec);
        txn_builder.add_mosaics(fb::WIPOffset::new(*mosaic_vec));

        let t = txn_builder.finish();

        _builder.finish(t, None);

        let buf = _builder.finished_data();

        Ok(transfer_transaction_schema().serialize(&mut buf.to_vec()))
    }

    fn to_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.to_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn Transaction + 'static> {
        Box::new((*self).clone())
    }
}

impl fmt::Display for TransferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
