/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::Value;

use crate::{
    multisig::Cosignature,
    transaction::{AggregateTransaction, Transaction}
};
use crate::api::map_aggregate_transactions_dto;

use super::{AbstractTransactionDto, CosignatureDto, TransactionDto, TransactionMetaDto};

/// AggregateTransactionDto : Transaction that combines multiple transactions together.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AggregateTransactionDto {
	#[serde(flatten)]
	pub r#abstract: AbstractTransactionDto,
	cosignatures: Option<Vec<CosignatureDto>>,
	/// The array of transactions initiated by different accounts.
	pub transactions: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AggregateTransactionInfoDto {
	pub meta: TransactionMetaDto,
	pub transaction: AggregateTransactionDto,
}

#[typetag::serde]
impl TransactionDto for AggregateTransactionInfoDto {
	fn compact(&self) -> anyhow::Result<Box<dyn Transaction>> {
		let dto = self.transaction.clone();

		let info = self.meta.compact()?;

		let txs_dto = map_aggregate_transactions_dto(dto.transactions)?;

		let common = dto.r#abstract.compact(info)?;

		let mut cosignatures: Vec<Cosignature> = vec![];
		if let Some(c) = dto.cosignatures {
			cosignatures = c.into_iter().map(|item| item.compact(common.network_type)).collect()
		}

		let mut inner_transactions: Vec<Box<dyn Transaction>> = Vec::with_capacity(txs_dto.len());
		for transaction_info_dto in txs_dto {
			inner_transactions.push(transaction_info_dto.compact()?);
		}

		Ok(Box::new(AggregateTransaction { common, cosignatures, inner_transactions }))
	}
}
