use std::any::Any;

use failure::_core::fmt::Debug;
use hyper::body::Bytes;
use serde_json::Value;

use crate::models::transaction::EntityTypeEnum as Entity;
use crate::models::utils::is_hex;

type Result<T> = ::std::result::Result<T, failure::Error>;

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

pub(super) fn map_transaction_dto_vec(mut body: Bytes) -> Result<String> {
    let value_dto_vec: Value = serde_json::from_slice(&mut body)?;

    let mut value_dto_vec_str = String::new().to_owned();
    for dto in value_dto_vec.as_object() {
//        value_dto_vec_str.push_str( &map_transaction_dto(dto)?)
    }

    Ok(value_dto_vec_str)

}

pub(super) fn map_transaction_dto(mut body: Bytes) -> Result<String> {
    let value_dto: Value = serde_json::from_slice(&mut body)?;

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
