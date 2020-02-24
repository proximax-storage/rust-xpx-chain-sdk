use ::std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::{fb, models::{
    account::{Account, Address, PublicAccount},
    consts::{AMOUNT_SIZE, MOSAIC_ID_SIZE, TRANSFER_HEADER_SIZE},
    message::Message,
    mosaic::Mosaic,
    network::NetworkType,
}};

use super::{
    AbstractTransaction,
    buffer::sisrius::buffers,
    deadline::Deadline,
    EntityTypeEnum,
    internal::sign_transaction,
    schema::transfer_transaction_schema,
    SignedTransaction,
    Transaction,
    TRANSFER_VERSION,
    TransferTransactionDto,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransaction {
    pub abs_transaction: AbstractTransaction,
    /// If the bit 0 of byte 0 is not set (like in 0x90), then it is a regular address.
    /// Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
    pub recipient: Address,
    /// The array of mosaics sent to the recipient.
    /// If the most significant bit of byte 0 is set, a
    /// namespaceId (alias) is used instead of a instead of a mosaic_id corresponds to a mosaic_id.
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
    ) -> crate::Result<TransferTransaction> {
        ensure!(
            !recipient.is_empty(),
            "address string is empty."
         );

        ensure!(
            mosaics.len() > 0,
            "mosaics must not be empty."
         );


        let abs_tx = AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: "".to_string(),
            signer: Default::default(),
            version: TRANSFER_VERSION,
            transaction_type: EntityTypeEnum::Transfer,
            max_fee: Default::default(),
            deadline,
        };

        Ok(TransferTransaction {
            abs_transaction: abs_tx,
            recipient,
            mosaics,
            message: Box::new(message),
        })
    }

    pub fn message_size(&self) -> usize {
        self.message.payload_to_bytes().len() + 1
    }

    pub fn to_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.to_aggregate(signer)
    }

    pub fn is_unconfirmed(&self) -> bool {
        self.abs_transaction.is_unconfirmed()
    }

    pub fn is_confirmed(&self) -> bool {
        self.abs_transaction.is_confirmed()
    }

    pub fn has_missing_signatures(&self) -> bool {
        self.abs_transaction.has_missing_signatures()
    }
}

impl Transaction for TransferTransaction {
    fn get_abs_transaction(self) -> AbstractTransaction {
        self.abs_transaction
    }

    fn size(&self) -> usize {
        TRANSFER_HEADER_SIZE + ((MOSAIC_ID_SIZE + AMOUNT_SIZE) * self.mosaics.len()) + self.message_size()
    }

    fn generate_bytes<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        // Create mosaics
        let ml = self.mosaics.len();

        let mut mosaics_buffer: Vec<fb::WIPOffset<buffers::MosaicBuffer<'a>>> = Vec::with_capacity(ml);

        for mosaic in &self.mosaics {
            let mosaic_id = _builder.create_vector(&mosaic.id.to_int_array());
            let mosaic_amount = _builder.create_vector(&mosaic.amount.to_int_array());

            let mut mosaic_buffer = buffers::MosaicBufferBuilder::new(&mut _builder);
            mosaic_buffer.add_id(mosaic_id);
            mosaic_buffer.add_amount(mosaic_amount);

            mosaics_buffer.push(mosaic_buffer.finish());
        };

        // Create message;
        let payload_vec = _builder.create_vector_direct(self.message.payload_to_bytes());

        let mut message_buffer = buffers::MessageBufferBuilder::new(&mut _builder);
        message_buffer.add_type_(self.message.message_type().get_value());
        message_buffer.add_payload(payload_vec);
        let message_vec = message_buffer.finish();

        let recipient = self.recipient.to_decode();

        let recipient_vec = _builder.create_vector_direct(&recipient);

        let mosaic_vec = _builder.create_vector(&mosaics_buffer);

        let abs_vector = &self.abs_transaction.generate_vector(&mut _builder);

        let mut txn_builder =
            buffers::TransferTransactionBufferBuilder::new(&mut _builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.get_value());
        txn_builder.add_max_fee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));
        txn_builder.add_recipient(recipient_vec);
        txn_builder.add_num_mosaics(ml as u8);
        txn_builder.add_message_size(self.message_size() as u16);
        txn_builder.add_message(message_vec);
        txn_builder.add_mosaics(fb::WIPOffset::new(*mosaic_vec));
        let t = txn_builder.finish();
        _builder.finish(t, None);

        let buf = _builder.finished_data();

        transfer_transaction_schema().serialize(&mut Vec::from(buf))
    }

    fn generate_embedded_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn has_missing_signatures(&self) -> bool {
        unimplemented!()
    }

    fn sign_transaction_with(&self, account: Account, generation_hash: String)
                             -> crate::Result<SignedTransaction> {
        sign_transaction(self as &dyn Transaction, account, generation_hash)
    }

    fn entity_type(&self) -> EntityTypeEnum {
        self.abs_transaction.transaction_type.to_owned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Into<(String)> for TransferTransaction {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for TransferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
//
//impl From<Value> for TransferTransaction {
//    fn from(e: Value) -> Self {
//        let algo: TransferTransaction = serde_json::from_value(e).unwrap();
//        return algo
//    }
//}
