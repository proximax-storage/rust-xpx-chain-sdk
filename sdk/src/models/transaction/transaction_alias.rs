use crate::models::{
    account::{Address, PublicAccount},
    alias::AliasActionType,
    consts::ALIAS_TRANSACTION_HEADER,
    id_model::Id,
    namespace::NamespaceId,
    network::NetworkType,
};

use super::{
    AbstractTransaction, ADDRESS_ALIAS_VERSION, buffer::alias::buffers, Deadline,
    EntityTypeEnum, schema::alias_transaction_schema,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AliasTransaction {
    pub abs_transaction: AbstractTransaction,
    pub action_type: AliasActionType,
    pub namespace_id: NamespaceId,
}

impl AliasTransaction {
    pub fn new(
        deadline: Deadline,
        address: Address,
        namespace_id: NamespaceId,
        action_type: AliasActionType,
        network_type: NetworkType,
    ) -> crate::Result<Self> {
        ensure!(!address.address.is_empty(), "address string is empty.");

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            ADDRESS_ALIAS_VERSION,
            EntityTypeEnum::AddressAlias,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            action_type,
            namespace_id,
        })
    }

    pub(crate) fn to_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.to_aggregate(signer)
    }

    pub(crate) fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }

    pub(crate) fn size(&self) -> usize {
        ALIAS_TRANSACTION_HEADER
    }

    pub(crate) fn embedded_to_bytes(
        &self,
        builder: &mut fb::FlatBufferBuilder,
        alias_vec: fb::WIPOffset<fb::Vector<u8>>,
        alias_size: usize,
    ) -> crate::Result<Vec<u8>> {
        let namespace_vec = builder.create_vector_direct(&self.namespace_id.to_u32_array());

        let abs_vector = self.abs_transaction.build_vector(builder);

        let mut txn_builder = buffers::AliasTransactionBufferBuilder::new(builder);
        txn_builder.add_size_((self.size() + alias_size) as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_action_type(self.action_type.value());
        txn_builder.add_namespace_id(namespace_vec);
        txn_builder.add_alias_id(alias_vec);
        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();

        Ok(alias_transaction_schema().serialize(&mut Vec::from(buf)))
    }
}
