use std::fmt;

use crate::models::account::{Address, PublicAccount};
use crate::models::message::Message;
use crate::models::mosaic::Mosaic;
use crate::models::network::NetworkType;
use crate::models::transaction::{AbstractTransaction, Transaction, TransactionType, TRANSFER_VERSION};
use crate::models::transaction::deadline::Deadline;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransaction {
    abs_transaction: AbstractTransaction,
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
        message: Box<dyn Message>,
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
            signature: "".to_string(),
            signer: Default::default(),
            version: TRANSFER_VERSION,
            transaction_type: TransactionType::Transfer,
            max_fee: Default::default(),
            deadline,
        };

        Ok(TransferTransaction {
            abs_transaction: abs_tx,
            recipient,
            mosaics,
            message,
        })
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
        unimplemented!()
    }

    fn generate_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn generate_embedded_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn serialize(&self) -> String {
        unimplemented!()
    }

    fn has_missing_signatures(&self) -> bool {
        unimplemented!()
    }
}

impl fmt::Display for TransferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
