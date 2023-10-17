/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
	account::Address,
	alias::AliasActionType,
	mosaic::MosaicId,
	namespace::NamespaceAlias,
	namespace::NamespaceId,
	transaction::Transaction,
	transaction::{AddressAliasTransaction, AliasTransaction, MosaicAliasTransaction},
};

use super::{AbstractTransactionDto, TransactionDto, TransactionMetaDto, Uint64Dto};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AliasDto {
	#[serde(rename = "type")]
	_type: u16,
	#[serde(skip_serializing_if = "Option::is_none")]
	mosaic_id: Option<Uint64Dto>,
	/// The aliased address in hexadecimal.
	#[serde(skip_serializing_if = "Option::is_none")]
	address: Option<String>,
}

impl AliasDto {
	pub fn compact(&self) -> anyhow::Result<NamespaceAlias> {
		let mut alias = NamespaceAlias::default();
		alias.type_ = self._type as u8;

		if let Some(a) = &self.address {
			let address = Address::from_encoded(a)?;
			alias.address = Some(address)
		};

		if let Some(m) = &self.mosaic_id {
			let mosaic_id = MosaicId::from(m.compact());
			alias.mosaic_id = Some(mosaic_id)
		}

		Ok(alias)
	}
}

/// AddressAliasTransactionDto :
/// Transaction that attaches a namespace to an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AliasTransactionDto {
	#[serde(flatten)]
	r#abstract: AbstractTransactionDto,
	alias_action: u8,
	namespace_id: Uint64Dto,
	#[serde(skip_serializing_if = "Option::is_none")]
	address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	mosaic_id: Option<Uint64Dto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddressAliasTransactionInfoDto {
	meta: TransactionMetaDto,
	transaction: AliasTransactionDto,
}

#[typetag::serde]
impl TransactionDto for AddressAliasTransactionInfoDto {
	fn compact(&self) -> anyhow::Result<Box<dyn Transaction>> {
		let dto = self.transaction.clone();

		let info = self.meta.compact()?;

		let address_encoded = dto.address.unwrap();

		let address = Address::from_encoded(&address_encoded)?;

		let common = dto.r#abstract.compact(info)?;

		Ok(Box::new(AddressAliasTransaction {
			alias_transaction: AliasTransaction {
				common,
				action_type: AliasActionType::from(dto.alias_action),
				namespace_id: NamespaceId::from(dto.namespace_id.compact()),
			},
			address,
		}))
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicAliasTransactionInfoDto {
	meta: TransactionMetaDto,
	transaction: AliasTransactionDto,
}

#[typetag::serde]
impl TransactionDto for MosaicAliasTransactionInfoDto {
	fn compact(&self) -> anyhow::Result<Box<dyn Transaction>> {
		let dto = self.transaction.clone();

		let info = self.meta.compact()?;

		let mosaic_id_dto = dto.mosaic_id.unwrap();

		let mosaic_id = MosaicId::from(mosaic_id_dto.compact());

		let common = dto.r#abstract.compact(info)?;

		Ok(Box::new(MosaicAliasTransaction {
			alias_transaction: AliasTransaction {
				common,
				action_type: AliasActionType::from(dto.alias_action),
				namespace_id: NamespaceId::from(dto.namespace_id.compact()),
			},
			mosaic_id,
		}))
	}
}
