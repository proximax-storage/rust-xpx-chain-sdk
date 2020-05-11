use ::std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::models::{
    account::{Account, PublicAccount},
    consts::MOSAIC_SUPPLY_CHANGE_TRANSACTION_SIZE,
    id_model::Id,
    mosaic::MosaicSupplyType,
    network::NetworkType,
    uint_64::Uint64,
};
use crate::Result;

use super::{
    AbstractTransaction,
    AbsTransaction,
    buffer::mosaic_supply_change::buffers,
    deadline::Deadline,
    EntityTypeEnum,
    internal::sign_transaction,
    MOSAIC_SUPPLY_CHANGE_VERSION,
    schema::mosaic_supply_change_transaction_schema,
    SignedTransaction,
    Transaction,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicSupplyChangeTransaction {
    pub abs_transaction: AbstractTransaction,
    pub supply_type: MosaicSupplyType,
    pub asset_id: Box<dyn Id>,
    pub delta: Uint64,
}

impl MosaicSupplyChangeTransaction {
    pub fn new(
        deadline: Deadline,
        supply_type: MosaicSupplyType,
        asset_id: impl Id + 'static,
        delta: Uint64,
        network_type: NetworkType,
    ) -> Result<Self> {
        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            MOSAIC_SUPPLY_CHANGE_VERSION,
            EntityTypeEnum::MosaicSupplyChange,
            network_type,
        );

        let id = Box::new(asset_id);

        Ok(Self { abs_transaction: abs_tx, supply_type, asset_id: id, delta })
    }
}

impl AbsTransaction for MosaicSupplyChangeTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for MosaicSupplyChangeTransaction {
    fn size(&self) -> usize {
        MOSAIC_SUPPLY_CHANGE_TRANSACTION_SIZE
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(self, account: Account, generation_hash: String)
                             -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes<'a>(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();
        let mosaic_vec = builder.create_vector(&self.asset_id.to_u32_array());
        let delta_vec = builder.create_vector(&self.delta.to_int_array());

        let abs_vector = self.abs_transaction.build_vector(&mut builder);

        let mut txn_builder =
            buffers::MosaicSupplyChangeTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_mosaic_id(mosaic_vec);
        txn_builder.add_direction(self.supply_type.clone() as u8);
        txn_builder.add_delta(delta_vec);
        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        Ok(mosaic_supply_change_transaction_schema().serialize(&mut Vec::from(buf)))
    }

    fn to_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.to_aggregate(signer)
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
