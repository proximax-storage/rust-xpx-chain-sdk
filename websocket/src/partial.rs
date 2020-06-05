// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use crate::Handler;
use bytes::Bytes;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsPartialMetaDto {
    pub channel_name: String,
    address: String,
    hash: String,
}

pub struct HandlerPartialAdd {
    pub handler: Box<dyn Fn(sdk::transaction::AggregateTransaction) -> bool + Send>
}

impl Handler for HandlerPartialAdd {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsPartialInfoDto {
    pub meta: WsPartialMetaDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    cosignatures: Option<Vec<api::CosignatureDto>>,
    pub transaction: api::AggregateTransactionDto,
}

impl WsPartialInfoDto {
    pub fn compact(&self) -> super::Result<sdk::transaction::AggregateTransaction> {
        let mut txs_dto: Vec<Box<dyn api::TransactionDto>> = vec![];
        for item in self.transaction.transactions.iter() {

            let body: Bytes = Bytes::from(item.to_string());
            let map_dto = api::map_transaction_dto(body).unwrap();

            txs_dto.push(serde_json::from_str(&map_dto).unwrap());
        };

        let abs_transaction = sdk::transaction::AbstractTransaction{
            transaction_info: None,
            network_type: Default::default(),
            signature: None,
            signer: Default::default(),
            version: 0,
            transaction_type: sdk::transaction::EntityTypeEnum::from(self.transaction._type),
            max_fee: None,
            deadline: None
        };

        let mut cosignatures: Vec<sdk::multisig::Cosignature> = vec![];
        if let Some(c) = self.cosignatures.clone(){
            cosignatures = c
                .into_iter()
                .map(|item| item.compact(abs_transaction.network_type))
                .collect()
        }

        let mut inner_transactions: Vec<Box<dyn sdk::transaction::Transaction>> = Vec::with_capacity(txs_dto.len());
        for transaction_info_dto in txs_dto {
            inner_transactions.push(transaction_info_dto.compact()?);
        };

        Ok(sdk::transaction::AggregateTransaction {
            abs_transaction,
            cosignatures,
            inner_transactions,
        })
    }
}