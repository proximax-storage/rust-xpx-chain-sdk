use ::std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::{fb, models::{
    account::{Account, PublicAccount}, consts::MOSAIC_SUPPLY_CHANGE_TRANSACTION_SIZE,
    Id,
    mosaic::MosaicSupplyType,
    network::NetworkType,
    Uint64
}};

use super::{
    AbstractTransaction,
    buffer::mosaic_supply_change::buffers,
    deadline::Deadline,
    EntityTypeEnum,
    internal::sign_transaction,
    MOSAIC_SUPPLY_CHANGE_VERSION,
    schema::mosaic_supply_change_transaction_schema,
    SignedTransaction,
    Transaction
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicSupplyChangeTransaction {
    pub abs_transaction: AbstractTransaction,
    pub supply_type: MosaicSupplyType,
    pub asset_id: Box<dyn Id>,
    pub delta: Uint64
}

impl MosaicSupplyChangeTransaction {
    pub fn new(
        deadline: Deadline,
        supply_type: MosaicSupplyType,
        asset_id: impl Id + 'static,
        delta: Uint64,
        network_type: NetworkType,
    ) -> crate::Result<MosaicSupplyChangeTransaction> {
        let abs_tx = AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: "".to_string(),
            signer: Default::default(),
            version: MOSAIC_SUPPLY_CHANGE_VERSION,
            transaction_type: EntityTypeEnum::MosaicSupplyChange,
            max_fee: Default::default(),
            deadline,
        };

        let id = Box::new(asset_id);
        Ok(MosaicSupplyChangeTransaction {
            abs_transaction: abs_tx,
            supply_type,
            asset_id: id,
            delta
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

impl Transaction for MosaicSupplyChangeTransaction {
    fn transaction_hash(&self) -> String {
        let mut hash = "".to_owned();

        if let Some(h) = self.abs_transaction().transaction_info {
            hash.push_str(&h.transaction_hash);
        }
        hash
    }

    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }

    fn size(&self) -> usize {
        MOSAIC_SUPPLY_CHANGE_TRANSACTION_SIZE
    }

    fn generate_bytes<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();
        let mosaic_vec = builder.create_vector(&self.asset_id.to_int_array());
        let delta_vec = builder.create_vector(&self.delta.to_int_array());

        let abs_vector = &self.abs_transaction.generate_vector(&mut builder);

        let mut txn_builder =
            buffers::MosaicSupplyChangeTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.get_value());
        txn_builder.add_max_fee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));
        txn_builder.add_mosaic_id(mosaic_vec);
        txn_builder.add_direction(self.supply_type.clone() as u8);
        txn_builder.add_delta(delta_vec);
        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        mosaic_supply_change_transaction_schema().serialize(&mut Vec::from(buf))
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

impl fmt::Display for MosaicSupplyChangeTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
