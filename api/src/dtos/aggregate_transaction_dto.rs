// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use serde_json::Value;

use sdk::transaction::{AggregateTransaction, Transaction};

use sdk::multisig::Cosignature;

use crate::internally::map_aggregate_transactions_dto;

use super::{
    AbstractTransactionDto, CosignatureDto, TransactionDto, TransactionMetaDto, Uint64Dto,
};

/// AggregateTransactionDto : Transaction that combines multiple transactions together.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<Uint64Dto>,
    /// An array of transaction cosignatures.
    #[serde(skip_serializing_if = "Option::is_none")]
    cosignatures: Option<Vec<CosignatureDto>>,
    /// The array of transactions initiated by different accounts.
    pub transactions: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: AggregateTransactionDto,
}

#[typetag::serde]
impl TransactionDto for AggregateTransactionInfoDto {
    fn compact(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.compact();

        let txs_dto = map_aggregate_transactions_dto(dto.transactions)?;

        let abs_transaction = AbstractTransactionDto::new(
            dto.signature,
            dto.signer,
            dto.version,
            dto._type,
            dto.max_fee,
            dto.deadline,
        ).compact(info)?;

        let mut cosignatures: Vec<Cosignature> = vec![];
        if let Some(c) = dto.cosignatures{
            cosignatures = c
                .into_iter()
                .map(|item| item.compact(abs_transaction.network_type))
                .collect()
        }

        let mut inner_transactions: Vec<Box<dyn Transaction>> = Vec::with_capacity(txs_dto.len());
        for transaction_info_dto in txs_dto {
            inner_transactions.push(transaction_info_dto.compact()?);
        }

        Ok(Box::new(AggregateTransaction {
            abs_transaction,
            cosignatures,
            inner_transactions,
        }))
    }
}
