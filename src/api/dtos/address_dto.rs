/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
	account::Address,
	transaction::{MetadataAddressTransaction, Transaction},
};

use super::TransactionDto;
use super::{ModifyMetadataTransactionDto, TransactionMetaDto};

/// AddressMetadataTransactionDto :
///Transaction that addes metadata to account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddressMetadataTransactionDto {
	#[serde(flatten)]
	metadata_transaction: ModifyMetadataTransactionDto,
	#[serde(rename = "metadataId")]
	address: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddressMetadataTransactionInfoDto {
	meta: TransactionMetaDto,
	transaction: AddressMetadataTransactionDto,
}

#[typetag::serde]
impl TransactionDto for AddressMetadataTransactionInfoDto {
	fn compact(&self) -> anyhow::Result<Box<dyn Transaction>> {
		let dto = self.transaction.clone();

		let info = self.meta.compact()?;

		let metadata_transaction = dto.metadata_transaction.compact(info)?;

		let address = Address::from_encoded(&dto.address)?;

		Ok(Box::new(MetadataAddressTransaction { metadata_transaction, address }))
	}
}
