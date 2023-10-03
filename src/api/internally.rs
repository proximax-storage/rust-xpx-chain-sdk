/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::Result;
use hyper::Method;
use serde::de::DeserializeOwned;

use {::std::fmt::Debug, bytes::Bytes, serde_json::Value};

use crate::{
    api, errors_const, helpers::is_hex, metadata::MetadataModification, mosaic::MosaicProperties,
    multisig::CosignatoryModification, network::NetworkType,
    transaction::TransactionType as Entity,
};
use crate::api::{
    AccountPropertiesTransactionInfoDto, AddExchangeOfferTransactionInfoDto,
    AddressAliasTransactionInfoDto, AggregateTransactionInfoDto,
    BlockchainUpgradeTransactionInfoDto, ExchangeOfferTransactionInfoDto,
    HashLockTransactionInfoDto, MetaDataEntryTransactionInfoDto, ModifyMultisigAccountTransactionInfoDto,
    MosaicAliasTransactionInfoDto, MosaicDefinitionTransactionInfoDto,
    MosaicSupplyChangeTransactionInfoDto, NetworkConfigTransactionInfoDto,
    Pagination, RegisterNamespaceTransactionInfoDto, RemoveExchangeOfferTransactionInfoDto,
    request as __internal_request, TransactionQueryParams, TransferTransactionInfoDto,
};
use crate::api::transport::service::Connection;
use crate::transaction::{Transaction, TransactionGroupType, Transactions, TransactionSearch};

use super::{
    CosignatoryModificationDto, MetadataModificationDto, MosaicPropertyDto, TransactionDto,
};

pub(crate) fn valid_vec_len<T>(vector: &[T], msg: &str) -> Result<()>
    where
        T: Debug,
{
    ensure!(!vector.is_empty(), "{}. {:?}", msg, vector);
    Ok(())
}

pub(crate) fn str_to_account_id(id: &str) -> Result<String> {
    match id.trim().len() {
        64 => {
            if !is_hex(id) {
                bail!(errors_const::ERR_INVALID_ACCOUNT_ID)
            }
            Ok(id.to_uppercase())
        }
        40 | 46 => Ok(id.to_uppercase().replace("-", "")),
        _ => bail!(errors_const::ERR_INVALID_ACCOUNT_ID),
    }
}

pub(crate) fn value_to_transaction_search(
    body_value: Value,
) -> api::error::Result<TransactionSearch> {
    let mut transactions: Transactions = vec![];
    let (transactions_dto, pagination) = map_transaction_dto_with_data(body_value)?;

    for transaction_dto in transactions_dto.into_iter() {
        transactions.push(transaction_dto.compact()?)
    }

    Ok(TransactionSearch { transactions, pagination })
}

pub(crate) fn value_to_transaction(body_value: Value) -> api::error::Result<Transactions> {
    let mut transactions: Transactions = vec![];
    let transactions_dto = map_transaction_vec_dto(body_value)?;

    for transaction_dto in transactions_dto.into_iter() {
        transactions.push(transaction_dto.compact()?);
    }

    Ok(transactions)
}

pub(crate) fn map_transaction_dto_with_data(
    body_value: Value,
) -> Result<(Vec<Box<dyn TransactionDto>>, Pagination)> {
    let value_data_dto_vec = body_value["data"].clone();
    let entity_dto_vec = map_transaction_vec_dto(value_data_dto_vec)?;

    let pagination: Pagination = serde_json::from_value(body_value["pagination"].clone())?;

    Ok((entity_dto_vec, pagination))
}

pub(crate) fn map_transaction_vec_dto(body_value: Value) -> Result<Vec<Box<dyn TransactionDto>>> {
    let mut entity_dto_vec = vec![];

    for value_raw in body_value.as_array().unwrap().clone() {
        let transaction_dto = map_transaction_dto_new(Bytes::from(format!("{}", value_raw)))?;
        entity_dto_vec.push(transaction_dto)
    }

    Ok(entity_dto_vec)
}

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn ser_callback<T: DeserializeOwned + TransactionDto + 'static>(
    value_dto: Value,
) -> Box<dyn TransactionDto> {
    Box::new(serde_json::from_value::<T>(value_dto).unwrap())
}

pub(crate) fn map_transaction_dto_new(body: Bytes) -> Result<Box<dyn TransactionDto>> {
    let value_dto: Value = serde_json::from_slice(&body)?;

    let entity_type: Entity = (value_dto["transaction"]["type"].as_u64().unwrap() as u16).into();

    let entity_dto = match entity_type {
        Entity::Transfer => ser_callback::<TransferTransactionInfoDto>(value_dto),
        Entity::AggregateBonded | Entity::AggregateComplete => {
            ser_callback::<AggregateTransactionInfoDto>(value_dto)
        }
        Entity::MosaicAlias => ser_callback::<MosaicAliasTransactionInfoDto>(value_dto),
        Entity::AddressAlias => ser_callback::<AddressAliasTransactionInfoDto>(value_dto),
        Entity::MosaicDefinition => ser_callback::<MosaicDefinitionTransactionInfoDto>(value_dto),
        Entity::MosaicSupplyChange => {
            ser_callback::<MosaicSupplyChangeTransactionInfoDto>(value_dto)
        }
        Entity::RegisterNamespace => ser_callback::<RegisterNamespaceTransactionInfoDto>(value_dto),
        Entity::AddExchangeOffer => ser_callback::<AddExchangeOfferTransactionInfoDto>(value_dto),
        Entity::ExchangeOffer => ser_callback::<ExchangeOfferTransactionInfoDto>(value_dto),
        Entity::RemoveExchangeOffer => {
            ser_callback::<RemoveExchangeOfferTransactionInfoDto>(value_dto)
        }
        Entity::MultisigAccountModify => {
            ser_callback::<ModifyMultisigAccountTransactionInfoDto>(value_dto)
        }

        Entity::AccountRestrictionAddress
        | Entity::AccountRestrictionMosaic
        | Entity::AccountRestrictionOperation => {
            ser_callback::<AccountPropertiesTransactionInfoDto>(value_dto)
        }
        Entity::HashLock => ser_callback::<HashLockTransactionInfoDto>(value_dto),

        // Entity::AccountLink => "AccountLink",
        // Entity::Block => "Block",
        Entity::AccountMetadataV2 | Entity::MosaicMetadataV2 | Entity::NamespaceMetadataV2 => {
            ser_callback::<MetaDataEntryTransactionInfoDto>(value_dto)
        }
        Entity::BlockchainUpgrade => ser_callback::<BlockchainUpgradeTransactionInfoDto>(value_dto),
        // Entity::NemesisBlock => "NemesisBlock:",
        Entity::NetworkConfigEntityType => {
            ser_callback::<NetworkConfigTransactionInfoDto>(value_dto)
        }
        // }
        // Entity::SecretLock => "SecretLock",
        // Entity::SecretProof => "SecretProof",
        _ => panic!("{}", errors_const::ERR_UNKNOWN_BLOCKCHAIN_TYPE),
    };

    Ok(entity_dto)
}

pub(crate) fn map_aggregate_transactions_dto(
    transactions: Vec<Value>,
) -> Result<Vec<Box<dyn TransactionDto>>> {
    let mut txs_dto: Vec<Box<dyn TransactionDto>> = vec![];
    for item in transactions.into_iter() {
        let body: Bytes = Bytes::from(item.to_string());
        let map_dto = map_transaction_dto_new(body)?;
        txs_dto.push(map_dto);
    }

    Ok(txs_dto)
}

pub(crate) fn mosaic_properties(dto: &[MosaicPropertyDto]) -> Result<MosaicProperties> {
    let mut flags: u64 = u64::default();
    let mut divisibility: u8 = 0;
    let mut duration: u64 = u64::default();

    for property in dto.iter() {
        match property.id {
            0 => flags = property.value.compact(),
            1 => divisibility = property.value.compact() as u8,
            2 => duration = property.value.compact(),
            _ => bail!("Unknown Property Id"),
        }
    }

    Ok(MosaicProperties::create(
        has_bits(flags, MosaicProperties::FLAG_SUPPLY_MUTABLE),
        has_bits(flags, MosaicProperties::FLAG_TRANSFERABLE),
        divisibility,
        Some(duration),
    ))
}

pub(crate) fn cosignatory_dto_vec_to_struct(
    modifications: Vec<CosignatoryModificationDto>,
    network_type: NetworkType,
) -> Vec<CosignatoryModification> {
    modifications.into_iter().map(|item| item.compact(network_type)).collect()
}

pub(crate) fn has_bits(number: u64, bits: u8) -> bool {
    (number & bits as u64) == bits as u64
}

pub(crate) fn metadata_dto_vec_to_struct(
    modifications: Vec<MetadataModificationDto>,
) -> Vec<MetadataModification> {
    modifications.iter().map(|modification| modification.compact()).collect()
}

pub(crate) async fn __internal_get_transactions(
    connection: Connection,
    route: &str,
    query_params: TransactionQueryParams,
) -> api::error::Result<TransactionSearch> {
    let mut req = __internal_request::ApiRequest::new(Method::GET, route.to_string());

    req = req.with_query_param(query_params.to_query_string());

    let body_value = req.execute(connection).await?;
    value_to_transaction_search(body_value)
}

pub(crate) async fn __internal_get_transactions_by_group_with_pagination(
    connection: Connection,
    route: &str,
    group_type: TransactionGroupType,
    query_params: Option<TransactionQueryParams>,
) -> api::error::Result<TransactionSearch> {
    let mut req = __internal_request::ApiRequest::new(Method::GET, route.to_string());

    req = req.with_path_param("group".to_string(), group_type.to_string());

    if let Some(query) = query_params {
        req = req.with_query_param(query.to_query_string());
    }

    let body_value = req.execute(connection).await?;
    value_to_transaction_search(body_value)
}

pub(crate) async fn __internal_get_transaction(
    connection: Connection,
    req: __internal_request::ApiRequest,
) -> api::error::Result<Box<dyn Transaction>> {
    let body_value: Value = req.execute(connection).await?;

    let transaction_dto = map_transaction_dto_new(Bytes::from(format!("{}", body_value)))?;

    Ok(transaction_dto.compact()?)
}
