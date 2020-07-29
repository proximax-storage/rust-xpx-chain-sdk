/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    bytes::Bytes,
    serde_json::Value,
    std::fmt::{Debug, Write},
};

use crate::{
    account::AccountId,
    errors_const,
    mosaic::{MosaicProperties, SUPPLY_MUTABLE, TRANSFERABLE},
    multisig::CosignatoryModification,
    network::NetworkType,
    transaction::{Hash, TransactionType as Entity},
    utils::is_hex,
    Result, Uint64,
};

use super::dtos::{CosignatoryModificationDto, MosaicPropertyDto, TransactionDto};

const TRANSACTION_ORDER_ASC: &str = "id";
const TRANSACTION_ORDER_DESC: &str = "-id";

pub(crate) struct AccountTransactionsOption {
    pub page_size: Option<i32>,
    pub id: Option<String>,
    pub ordering: Option<String>,
}

impl AccountTransactionsOption {
    pub fn new(page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Result<Self> {
        if ordering.is_some() {
            ensure!(
                ordering.unwrap() == TRANSACTION_ORDER_ASC
                    || ordering.unwrap() == TRANSACTION_ORDER_DESC,
                "Invalid value { } ordering",
                ordering.unwrap()
            );
        }

        Ok(Self {
            page_size,
            id: match id {
                Some(i) => Some(i.to_string()),
                _ => None,
            },
            ordering: match ordering {
                Some(i) => Some(i.to_string()),
                _ => None,
            },
        })
    }
}

pub(crate) fn str_to_hash(hash: &str) -> Result<Hash> {
    let raw_hash = hash.trim().to_uppercase();

    ensure!(!raw_hash.is_empty(), errors_const::ERR_INVALID_HASH_HEX);

    ensure!(
        is_hex(&raw_hash),
        "{} {}.",
        errors_const::ERR_INVALID_HASH_HEX,
        raw_hash
    );

    ensure!(
        raw_hash.len() == 64,
        "{} {}.",
        errors_const::ERR_INVALID_HASH_LENGTH,
        raw_hash
    );

    Ok(raw_hash)
}

pub(crate) fn valid_vec_len<T>(vector: &[T], msg: &str) -> Result<()>
where
    T: Debug,
{
    ensure!(!vector.is_empty(), "{}. {:?}", msg, vector);
    Ok(())
}

pub(crate) fn str_to_account_id(id: &str) -> Result<AccountId> {
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

pub(crate) fn valid_vec_hash(vector: &[&str]) -> Result<()> {
    for hash in vector {
        if hash.len() != 24 {
            str_to_hash(hash)?;
        }
    }
    Ok(())
}

pub(crate) fn map_transaction_dto_vec(body: Bytes) -> Result<String> {
    let value_dto_vec: Value = serde_json::from_slice(&body)?;

    let mut value_dto_vec_str: String = "".to_string();
    value_dto_vec_str.write_char('[')?;
    for dto in 0..value_dto_vec.as_array().unwrap().len() {
        let to_array = &value_dto_vec.as_array().unwrap()[dto];

        let to_string = format!("{}", to_array);

        let transaction_dto = map_transaction_dto(Bytes::from(to_string))?;

        value_dto_vec_str.push_str(&serde_json::to_string(&transaction_dto)?);

        if value_dto_vec.as_array().unwrap().len() != dto + 1 {
            value_dto_vec_str.write_char(',')?;
        }
    }

    value_dto_vec_str.write_char(']')?;

    let value_dto_vec_str = value_dto_vec_str
        .replace(&['\\'][..], "")
        .replace(r#"["{"#, r#"[{"#)
        .replace(r#"}","{"#, r#"},{"#)
        .replace(r#"}}}"]"#, r#"}}}]"#);

    Ok(value_dto_vec_str)
}

pub(crate) fn map_transaction_dto(body: Bytes) -> Result<String> {
    let mut value_dto: Value = serde_json::from_slice(&body)?;

    let entity_type = Entity::from(value_dto["transaction"]["type"].as_u64().unwrap() as u16);

    let entity_dto = match entity_type {
        Entity::AccountLink => "AccountLink",
        Entity::AccountRestrictionAddress => "AccountProperties",
        Entity::AccountRestrictionMosaic => "AccountProperties",
        Entity::AccountRestrictionEntity => "AccountProperties",
        Entity::AddressAlias => "AddressAlias",
        Entity::AggregateBonded => "Aggregate",
        Entity::AggregateComplete => "Aggregate",
        Entity::AddExchangeOffer => "AddExchangeOffer",
        Entity::ExchangeOffer => "ExchangeOffer",
        Entity::RemoveExchangeOffer => "RemoveExchangeOffer",
        Entity::Block => "Block",
        Entity::BlockchainUpgrade => "BlockchainUpgrade",
        Entity::Lock => "HashLock",
        Entity::ModifyMultisigAccount => "ModifyMultisigAccount",
        Entity::MosaicAlias => "MosaicAlias",
        Entity::MosaicDefinition => "MosaicDefinition",
        Entity::MosaicSupplyChange => "MosaicSupplyChange",
        Entity::NamespaceRegistration => "RegisterNamespace",
        Entity::NemesisBlock => "NemesisBlock:",
        Entity::NetworkConfigEntityType => "NetworkConfigEntityType",
        Entity::SecretLock => "SecretLock",
        Entity::SecretProof => "SecretProof",
        Entity::Transfer => "Transfer",
        _ => errors_const::ERR_UNKNOWN_BLOCKCHAIN_TYPE,
    };

    if entity_type == Entity::AggregateBonded || entity_type == Entity::AggregateComplete {
        parse_meta_ws(&mut value_dto)?
    }

    if value_dto["meta"].is_null() {
        Ok(parse_meta(value_dto, entity_dto))
    } else {
        Ok(parse_entity_type_dto(value_dto, entity_dto))
    }
}

fn parse_meta_ws(value_dto: &mut Value) -> Result<()> {
    if let Some(v) = value_dto["transaction"]["transactions"].as_array_mut() {
        let mut meta_value: Vec<Value> = vec![];

        for item in v.iter_mut() {
            if item["meta"].is_null() {
                let meta = r#""meta": {}, "transaction":"#;
                let parse_meta_srt = format!("{}", item).replace(r#""transaction":"#, meta);

                let parse_meta: Value = serde_json::from_str(&parse_meta_srt)?;
                meta_value.push(parse_meta)
            } else {
                let parse_meta: Value = serde_json::from_str(&*format!("{}", item))?;
                meta_value.push(parse_meta)
            }
        }
        v.clear();
        v.append(&mut meta_value)
    };
    Ok(())
}

fn parse_meta(value_dto: Value, entity_dto: &str) -> String {
    let meta = r#"{"meta": {}, "transaction":"#;

    let parse_meta = format!("{}", value_dto).replace(r#"{"transaction":"#, meta);

    parse_entity_type_dto(parse_meta.parse().unwrap(), entity_dto)
}

fn parse_entity_type_dto(value_dto: Value, entity_dto: &str) -> String {
    let info_dto = format!("{{\"{}TransactionInfoDto\":{{\"meta\":", entity_dto);

    format!("{}", value_dto)
        .replace(r#"{"meta":"#, &info_dto)
        .replace("}}", r#"}}}"#)
}

pub(crate) fn map_aggregate_transactions_dto(
    transactions: Vec<Value>,
) -> Result<Vec<Box<dyn TransactionDto>>> {
    let mut txs_dto: Vec<Box<dyn TransactionDto>> = vec![];
    for item in transactions.into_iter() {
        let body: Bytes = Bytes::from(item["AggregateTransactionInfoDto"].to_string());
        let map_dto = map_transaction_dto(body)?;
        txs_dto.push(serde_json::from_str(&map_dto)?);
    }

    Ok(txs_dto)
}

pub(crate) fn mosaic_properties(dto: &[MosaicPropertyDto]) -> Result<MosaicProperties> {
    let mut flags: Uint64 = Uint64::default();
    let mut divisibility: u8 = 0;
    let mut duration: Uint64 = Uint64::default();

    for property in dto.iter() {
        match property.id {
            0 => flags = property.value.compact(),
            1 => divisibility = *property.value.compact() as u8,
            2 => duration = property.value.compact(),
            _ => bail!("Unknown Property Id"),
        }
    }

    MosaicProperties::new(
        has_bits(flags, SUPPLY_MUTABLE),
        has_bits(flags, TRANSFERABLE),
        divisibility,
        duration,
    )
}

pub(crate) fn cosignatory_dto_vec_to_struct(
    modifications: Vec<CosignatoryModificationDto>,
    network_type: NetworkType,
) -> Vec<CosignatoryModification> {
    modifications
        .into_iter()
        .map(|item| item.compact(network_type))
        .collect()
}

pub(crate) fn has_bits(number: Uint64, bits: u8) -> bool {
    (*number & bits as u64) == bits as u64
}
