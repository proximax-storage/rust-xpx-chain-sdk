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

pub(super) fn map_transaction_dto(mut body: Bytes) -> Result<String> {
    let v: Value = serde_json::from_slice(&mut body)?;

    let entity_type = Entity::from(
        v["transaction"]["type"].as_u64().unwrap()
    );

    let transaction_dto = match entity_type {
        Entity::AccountLink => r#"{"AccountLinkTransactionInfoDto":{"meta":"#,
        Entity::AccountRestrictionAddress => r#"{"AccountRestrictionAddressTransactionInfoDto":{"meta":"#,
        Entity::AccountRestrictionMosaic => r#"{"AccountRestrictionMosaicTransactionInfoDto":{"meta":"#,
        Entity::AccountRestrictionOperation => r#"{"AccountRestrictionOperationTransactionInfoDto":{"meta":"#,
        Entity::AddressAlias => r#"{"AddressAliasTransactionInfoDto":{"meta":"#,
        Entity::AggregateBonded => r#"{"AggregateBondedTransactionInfoDto":{"meta":"#,
        Entity::AggregateComplete => r#"{"AggregateCompleteTransactionInfoDto":{"meta":"#,
        Entity::Block => r#"{"BlockTransactionInfoDto":{"meta":"#,
        Entity::BlockchainUpgrade => r#"{"BlockchainUpgradeTransactionInfoDto":{"meta":"#,
        Entity::Lock => r#"{"LockTransactionInfoDto":{"meta":"#,
        Entity::ModifyMultisigAccount => r#"{"ModifyMultisigAccountTransactionInfoDto":{"meta":"#,
        Entity::MosaicAlias => r#"{"ModifyMultisigAccountTransactionInfoDto":{"meta":"#,
        Entity::MosaicDefinition => r#"{"MosaicDefinitionTransactionInfoDto":{"meta":"#,
        Entity::MosaicSupplyChange => r#"{"MosaicSupplyChangeTransactionInfoDto":{"meta":"#,
        Entity::NamespaceRegistration => r#"{"ModifyMultisigAccountTransactionInfoDto":{"meta":"#,
        Entity::NemesisBlock => r#"{"NemesisBlockTransactionInfoDto":{"meta":"#,
        Entity::NetworkConfigEntityType => r#"{"NetworkConfigEntityTypeTransactionInfoDto":{"meta":"#,
        Entity::SecretLock => r#"{"SecretLockTransactionInfoDto":{"meta":"#,
        Entity::SecretProof => r#"{"SecretProofTransactionInfoDto":{"meta":"#,
        Entity::Transfer => r#"{"TransferTransactionInfoDto":{"meta":"#,
        _ => "NotDto"
    };

    Ok(format!("{:?}", body)
        .trim().replace(&['\\'][..], "")
        .replace(r#"b"{"meta":"#, transaction_dto)
        .replace("}}\"", r#"}}}"#))
}
