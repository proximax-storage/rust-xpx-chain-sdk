use failure::_core::fmt::Debug;
use hyper::body::Bytes;
use serde_json::Value;

use crate::models::transaction::EntityTypeEnum as Entity;
use crate::models::utils::is_hex;

use crate::Result;
use std::fmt::Write;

pub(super) fn valid_hash(hash: &str) -> Result<bool> {
    ensure!(
        !hash.is_empty(),
        "transaction_hashes is empty."
    );

    ensure!(
        is_hex(hash),
        "hash {} it's not hex.", hash
    );

    ensure!(hash.len() == 64, "hash {} invalid len.", hash);

    Ok(true)
}

pub(super) fn valid_vec_len<T>(vector: &Vec<T>) -> Result<bool> where T: Debug
{
    ensure!(
        !vector.is_empty(),
        "vector {:?} is empty.", vector
    );
    Ok(true)
}

pub(super) fn valid_vec_hash(vector: &Vec<&str>) -> Result<bool> {
    for hash in vector {
        valid_hash(hash)?;
    }
    Ok(true)
}

pub(super) fn map_transaction_dto_vec(body: Bytes) -> Result<String> {
    let value_dto_vec: Value = serde_json::from_slice(&body)?;

    let mut value_dto_vec_str: String = "".to_string();
    value_dto_vec_str.write_char('[')?;
    for dto in 0..value_dto_vec.as_array().unwrap().len() {
       let to_array= &value_dto_vec.as_array().unwrap()[dto];

        let to_string = format!("{}", to_array);

        let transaction_dto = map_transaction_dto(Bytes::from(to_string)).unwrap();

        value_dto_vec_str.push_str(&serde_json::to_string(&transaction_dto).unwrap());

        if value_dto_vec.as_array().unwrap().len() != dto + 1 {
            value_dto_vec_str.write_char(',')?;
        }
    }

    value_dto_vec_str.write_char(']')?;

    let value_dto_vec_str = value_dto_vec_str.replace(&['\\'][..], "");
    let value_dto_vec_str = value_dto_vec_str.replace(r#"["{"#, r#"[{"#);
    let value_dto_vec_str = value_dto_vec_str.replace(r#"}","{"#, r#"},{"#);
    let value_dto_vec_str = value_dto_vec_str.replace(r#"}}}"]"#, r#"}}}]"#);

    Ok(value_dto_vec_str)
}

pub(super) fn map_transaction_dto(body: Bytes) -> Result<String> {
    let value_dto: Value = serde_json::from_slice(&body)?;

    let entity_type = Entity::from(
        value_dto["transaction"]["type"].as_u64().unwrap()
    );

    let entity_dto = match entity_type {
        Entity::AccountLink => "AccountLink",
        Entity::AccountRestrictionAddress => "AccountRestrictionAddress",
        Entity::AccountRestrictionMosaic => "AccountRestrictionMosaic",
        Entity::AccountRestrictionOperation => "AccountRestrictionOperation",
        Entity::AddressAlias => "AddressAlias",
        Entity::AggregateBonded => "AggregateBonded",
        Entity::AggregateComplete => "AggregateComplete",
        Entity::Block => "Block",
        Entity::BlockchainUpgrade => "BlockchainUpgrade",
        Entity::Lock => "Lock",
        Entity::ModifyMultisigAccount => "ModifyMultisigAccount",
        Entity::MosaicAlias => "ModifyMultisigAccount",
        Entity::MosaicDefinition => "MosaicDefinition",
        Entity::MosaicSupplyChange => "MosaicSupplyChange",
        Entity::NamespaceRegistration => "ModifyMultisigAccount",
        Entity::NemesisBlock => "NemesisBlock:",
        Entity::NetworkConfigEntityType => "NetworkConfigEntityType",
        Entity::SecretLock => "SecretLock",
        Entity::SecretProof => "SecretProof",
        Entity::Transfer => "Transfer",
        _ => "NotDto"
    };

    let info_dto = format!("{{\"{}TransactionInfoDto\":{{\"meta\":", entity_dto);

    Ok(format!("{}", value_dto)
        .replace(r#"{"meta":"#, &info_dto)
        .replace("}}", r#"}}}"#))
}
