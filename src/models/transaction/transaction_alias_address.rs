use std::any::Any;
use std::fmt;

use serde_json::Value;

use crate::models::account::{Account, Address, PublicAccount};
use crate::models::alias::AliasActionType;
use crate::models::consts::ADDRESS_SIZE;
use crate::models::namespace::NamespaceId;
use crate::models::network::NetworkType;
use crate::Result;

use super::{AbstractTransaction, AbsTransaction, ADDRESS_ALIAS_VERSION, AliasTransaction,
            Deadline, EntityTypeEnum, sign_transaction, SignedTransaction, Transaction
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressAliasTransaction {
    pub alias_transaction: AliasTransaction,
    pub address: Address
}

impl AddressAliasTransaction {
    pub fn new(deadline: Deadline, address: Address, namespace_id: NamespaceId,
               action_type: AliasActionType, network_type: NetworkType) -> Result<Self>
    {
        ensure!(
            !address.address.is_empty(),
            "address string is empty."
         );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            ADDRESS_ALIAS_VERSION,
            EntityTypeEnum::AddressAlias,
            network_type
        );

        Ok(Self {
            alias_transaction: AliasTransaction {
                abs_transaction: abs_tx,
                action_type,
                namespace_id
            },
            address
        })
    }
}

impl AbsTransaction for AddressAliasTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.alias_transaction.abs_transaction()
    }
}

impl Transaction for AddressAliasTransaction {
    fn size(&self) -> usize {
        self.alias_transaction.size() + ADDRESS_SIZE
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(self, account: Account, generation_hash: String)
                             -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let address_bytes = self.address.to_decode();

        let address_vector = builder.create_vector_direct(&address_bytes);

        self.alias_transaction.embedded_to_bytes(&mut builder, address_vector, ADDRESS_SIZE)
    }

    fn to_aggregate(&mut self, signer: PublicAccount) {
        self.alias_transaction.to_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for AddressAliasTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}