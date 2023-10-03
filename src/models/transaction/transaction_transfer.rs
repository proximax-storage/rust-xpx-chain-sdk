/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;

use ::std::fmt;
use serde_json::Value;

use crate::{
	account::UnresolvedAddress,
	AsUint64,
	message::Message,
	mosaic::Mosaic,
	network::NetworkType,
};
use crate::account::PublicAccount;
use crate::models::consts::{AMOUNT_SIZE, MOSAIC_ID_SIZE, TRANSFER_HEADER_SIZE};
use crate::transaction::buffers;
use crate::transaction::schema::transfer_transaction_schema;

use super::{
	CommonTransaction, deadline::Deadline,
	Transaction, TransactionType, TransactionVersion,
};

/// It defines a structure for Sirius transfer transactions with recipient information, mosaic data, and message content.
#[derive(Clone, Debug, Serialize, Builder, Deserialize)]
#[builder(create_empty = "empty", build_fn(error = "crate::api::error::Error"))]
#[serde(rename_all = "camelCase")]
pub struct TransferTransaction {
    /// Represents common transaction information..
    #[builder(private, pattern = "mutable")]
    pub common: CommonTransaction,
    /// If the bit 0 of byte 0 is not set (like in 0x90), then it is a regular address.
    /// Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
    #[builder(setter(custom))]
    pub recipient: Box<dyn UnresolvedAddress>,
    /// The array of mosaics sent to the recipient.
    /// If the most significant bit of byte 0 is set, a
    /// namespace_id (alias) is used instead of a instead of a mosaic_id corresponds to a mosaic_id.
    #[builder(default)]
    pub mosaics: Vec<Mosaic>,
    /// Represents the message included in the transaction.
    #[builder(setter(custom), default = "Box::new(<dyn Message>::empty_message())")]
    pub message: Box<dyn Message>,
}

impl TransferTransactionBuilder {
    /// The recipient method sets the recipient field.
    pub fn recipient<R: 'static + UnresolvedAddress>(
        &mut self,
        value: R,
    ) -> &mut TransferTransactionBuilder {
        self.recipient = Some(Box::new(value));
        self
    }

    /// The message method sets the message field.
    pub fn message<M: 'static + Message>(&mut self, value: M) -> &mut TransferTransactionBuilder {
        self.message = Some(Box::new(value));
        self
    }

    /// The deadline method sets the deadline field.
    pub fn deadline(&mut self, value: Deadline) -> &mut TransferTransactionBuilder {
        self.common.as_mut().map(|item| item.deadline = Some(value));
        self
    }

    /// The max_fee method sets the max_fee field.
    pub fn max_fee(&mut self, value: u64) -> &mut TransferTransactionBuilder {
        self.common.as_mut().map(|item| item.max_fee = Some(value));
        self
    }
}

impl TransferTransaction {
    /// Build a transfer transaction object.
    pub fn builder(network_type: NetworkType) -> TransferTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::Transfer,
            network_type,
            TransactionVersion::TRANSFER,
            Some(Default::default()),
            None,
        );
        TransferTransactionBuilder { common: Some(common), ..Default::default() }
    }

    /// The String notation for the set recipient.
    pub fn recipient_to_string(&self) -> String {
        self.recipient.to_string()
    }

    /// Sorted mosaic vec.
    pub fn sort_mosaics(&self) -> Vec<Mosaic> {
        let mut mosaics_clone = self.mosaics.clone();
        mosaics_clone.sort_by(|a, b| {
            let long_a = a.asset_id.to_u64();
            let long_b = b.asset_id.to_u64();
            long_a.cmp(&long_b)
        });
        mosaics_clone
    }

    pub fn message_size(&self) -> usize {
        self.message.clone().payload_to_vec().len() + 1
    }
}

impl TryFrom<TransferTransactionBuilder> for TransferTransaction {
    type Error = crate::api::error::Error;

    fn try_from(builder: TransferTransactionBuilder) -> Result<Self, Self::Error> {
        builder.build()
    }
}

#[typetag::serde]
impl Transaction for TransferTransaction {
    fn size(&self) -> usize {
        TRANSFER_HEADER_SIZE
            + ((MOSAIC_ID_SIZE + AMOUNT_SIZE) * self.mosaics.len())
            + self.message_size()
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn get_common_transaction(&self) -> CommonTransaction {
        self.common.to_owned()
    }

    fn to_serializer<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        // Create mosaics
        let ml = self.mosaics.len();

        let mut mosaics_buffer: Vec<fb::WIPOffset<buffers::MosaicBuffer<'a>>> =
            Vec::with_capacity(ml);

        for mosaic in self.mosaics.iter() {
            let mosaic_id = _builder.create_vector(&mosaic.asset_id.to_dto());
            let mosaic_amount = _builder.create_vector(&mosaic.amount.to_dto());

            let mut mosaic_buffer = buffers::MosaicBufferBuilder::new(&mut _builder);
            mosaic_buffer.add_id(mosaic_id);
            mosaic_buffer.add_amount(mosaic_amount);

            mosaics_buffer.push(mosaic_buffer.finish());
        }

        // Create message;
        let payload_vec = _builder.create_vector(&self.message.payload_to_vec());

        let mut message_buffer = buffers::MessageBufferBuilder::new(&mut _builder);
        message_buffer.add_type_(self.message.message_type().value());
        message_buffer.add_payload(payload_vec);
        let message_vec = message_buffer.finish();

        let recipient = self.recipient.unresolved_address_to_bytes(self.common.network_type);

        let recipient_vec = _builder.create_vector(&recipient);

        let mosaic_vec = _builder.create_vector(&mosaics_buffer);

        let abs_vector = self.common.build_vector(&mut _builder);

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

        transfer_transaction_schema().serialize(&mut buf.to_vec())
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.common.set_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn box_clone(&self) -> Box<dyn Transaction + 'static> {
        Box::new((*self).clone())
    }
}

impl fmt::Display for TransferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

#[cfg(test)]
pub mod tests {
    pub mod tests_size {
		use std::str::FromStr;

		use lazy_static::lazy_static;

		use crate::account::{Address, UnresolvedAddress};
		use crate::account::tests::TESTING_ACCOUNT;
		use crate::helpers::GenerationHash;
		use crate::message::{Message, PlainMessage};
		use crate::mosaic::Mosaic;
		use crate::namespace::NamespaceId;
		use crate::network::NetworkType;
		use crate::transaction::{Transaction, TransferTransaction};

		const NETWORK_TYPE: NetworkType = NetworkType::PublicTest;

        lazy_static! {
			static ref GENERATION_HASH: GenerationHash = GenerationHash::from_str(
				"56D112C98F7A7E34D1AEDC4BD01BC06CA2276DD546A93E36690B785E82439CA9"
			)
			.unwrap();
			static ref NAMESPACE_ID: NamespaceId = NamespaceId::from_name("cat.currency").unwrap();
			static ref CAT_CURRENCY: Mosaic =
				Mosaic::create_relative((*NAMESPACE_ID).clone(), 100, 6).unwrap();
		}

        #[test]
        fn should_default_max_fee_field_be_set_to_none() {
            let transfer_transaction = TransferTransaction::builder(NETWORK_TYPE)
                .recipient(Address::from_raw("VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU").unwrap())
                .message(PlainMessage::create("test-message"))
                .build()
                .unwrap();

            assert_eq!(transfer_transaction.common.max_fee, None);
        }

        #[test]
        fn should_filled_max_fee_override_transaction_max_fee() {
            let transfer_transaction = TransferTransaction::builder(NETWORK_TYPE)
                .recipient(Address::from_raw("VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU").unwrap())
                .message(PlainMessage::create("test-message"))
                .max_fee(1)
                .build()
                .unwrap();

            assert_eq!(transfer_transaction.common.max_fee, Some(1));
        }

        #[test]
        fn should_create_complete_an_transfer_transaction_and_sign_it_without_mosaics() {
            let transfer_transaction = TransferTransaction::builder(NETWORK_TYPE)
                .recipient(
                    Address::from_raw("VDI5IS-5YXT5G-LRT5RS-S3EZIB-4QOAI2-6GLWR2-TDK7").unwrap(),
                )
                .message(PlainMessage::create("test-message"))
                .build()
                .unwrap();

            assert_eq!(transfer_transaction.message.as_ref().payload(), "test-message");
            assert_eq!(transfer_transaction.mosaics.len(), 0);
            assert!(transfer_transaction.recipient.try_downcast_ref::<Address>().is_some());
            assert_eq!(
                transfer_transaction.recipient.downcast_ref::<Address>().recipient_to_string(),
                "VDI5IS5YXT5GLRT5RSS3EZIB4QOAI26GLWR2TDK7"
            );

            let signed_transaction = transfer_transaction
                .sign_with(TESTING_ACCOUNT.clone(), *GENERATION_HASH)
                .unwrap();

            assert_eq!(&signed_transaction.payload[236..signed_transaction.payload.len()],
                       "37000000A8D1D44BB8BCFA65C67D8CA5B26501E41C046BC65DA3A98D5F0D000000746573742D6D657373616765");
        }

        #[test]
        fn should_create_complete_an_transfer_transaction_with_empty_message() {
            let transfer_transaction = TransferTransaction::builder(NETWORK_TYPE)
                .recipient(
                    Address::from_raw("VDI5IS-5YXT5G-LRT5RS-S3EZIB-4QOAI2-6GLWR2-TDK7").unwrap(),
                )
                .message(<dyn Message>::empty_message())
                .build()
                .unwrap();

            assert_eq!(transfer_transaction.message.as_ref().payload(), "");
            assert_eq!(transfer_transaction.mosaics.len(), 0);
            assert!(transfer_transaction.recipient.try_downcast_ref::<Address>().is_some());
            assert_eq!(
                transfer_transaction.recipient.downcast_ref::<Address>().recipient_to_string(),
                "VDI5IS5YXT5GLRT5RSS3EZIB4QOAI26GLWR2TDK7"
            );

            let signed_transaction = transfer_transaction
                .sign_with(TESTING_ACCOUNT.clone(), *GENERATION_HASH)
                .unwrap();

            assert_eq!(
                &signed_transaction.payload[236..signed_transaction.payload.len()],
                "37000000A8D1D44BB8BCFA65C67D8CA5B26501E41C046BC65DA3A98D5F01000000"
            );
        }

        #[test]
        fn should_return_154_for_transfer_transaction_with_1_mosaic_and_message_xpx() {
            let transfer_transaction = TransferTransaction::builder(NETWORK_TYPE)
                .recipient(Address::from_raw("VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU").unwrap())
                .message(PlainMessage::create("XPX"))
                .build()
                .unwrap();

            assert_eq!(transfer_transaction.to_serializer().len(), transfer_transaction.size());
            assert_eq!(transfer_transaction.size(), 154);
        }
    }
}
