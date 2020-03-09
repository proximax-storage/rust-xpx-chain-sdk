use failure::_core::any::Any;
use serde_json::Value;
use hyper::body::Bytes;

use crate::models::multisig::CosignatureDto;
use crate::models::transaction::{AbstractTransactionDto, AggregateTransaction, Transaction, TransactionDto, TransactionMetaDto};
use crate::models::uint_64::Uint64Dto;
use crate::apis::internally::map_aggregate_transactions_dto;

/// AggregateTransactionDto : Transaction that combines multiple transactions together.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AggregateTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    /// An array of transaction cosignatures.
    pub cosignatures: Vec<CosignatureDto>,
    /// The array of transactions initiated by different accounts.
    pub transactions: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AggregateTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: AggregateTransactionDto,
}

#[typetag::serde]
impl TransactionDto for AggregateTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {

        let dto = self.transaction.clone();

        let info = self.meta.to_struct();

        let mut txs_dto = map_aggregate_transactions_dto(dto.transactions)?;

        let abs_transaction = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct(info)?;

        let cosignatures = dto.cosignatures.into_iter().map(|item|
            {
                item.to_struct(abs_transaction.network_type)
            }
        ).collect();

        let mut inner_transactions: Vec<Box<dyn Transaction>> = Vec::with_capacity(txs_dto.len());
        for transaction_info_dto in txs_dto {
            inner_transactions.push(transaction_info_dto.to_struct()?);
        }

        Ok(Box::new(AggregateTransaction {
            abs_transaction,
            cosignatures,
            inner_transactions
        }))
    }
}
