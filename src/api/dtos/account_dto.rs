/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    serde::{Deserialize, Deserializer},
    serde_json::Value,
};

use crate::{
    account::{
        AccountInfo, AccountLinkTypeEnum, AccountName, AccountProperties,
        AccountPropertiesAddressModification, AccountPropertiesEntityTypeModification,
        AccountPropertiesMosaicModification, AccountPropertyType, Address,
    },
    models::{error::Error::Failure, Result},
    mosaic::{Mosaic, MosaicId},
    transaction::{
        AccountPropertiesAddressTransaction, AccountPropertiesEntityTypeTransaction,
        AccountPropertiesMosaicTransaction, EntityTypeEnum, Transaction,
    },
};

use super::{AbstractTransactionDto, MosaicDto, TransactionDto, TransactionMetaDto, Uint64Dto};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountDto {
    address: String,
    address_height: Uint64Dto,
    /// The public key of an account can be used to verify signatures of the account. Only accounts that have already published a transaction have a public key assigned to the account. Otherwise, the field is null.
    public_key: String,
    public_key_height: Uint64Dto,
    /// The list of mosaics the account owns. The amount is represented in absolute amount. Thus a balance of 123456789 for a mosaic with divisibility 6 (absolute) means the account owns 123.456789 instead.
    mosaics: Vec<MosaicDto>,
    account_type: u8,
    /// The public key of a linked account. The linked account can use|provide balance for delegated harvesting.
    linked_account_key: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AccountInfoDto {
    account: AccountDto,
}

impl AccountInfoDto {
    pub(crate) fn compact(&self) -> crate::Result<AccountInfo> {
        let dto = &self.account;
        let add = Address::from_encoded(&dto.clone().address)?;
        let acc_type = AccountLinkTypeEnum::new(dto.clone().account_type);

        let mosaics: Vec<Mosaic> = dto
            .mosaics
            .iter()
            .map(move |mosaic_dto| mosaic_dto.compact())
            .collect();

        Ok(AccountInfo::new(
            add,
            dto.clone().address_height.compact(),
            dto.clone().public_key,
            dto.public_key_height.compact(),
            acc_type,
            mosaics,
        ))
    }
}

/// AccountLinkTransactionDto : Delegates the account importance score to a proxy account.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountLinkTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    /// The public key of the remote account.
    remote_account_key: String,
    action: u8,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountNamesDto {
    /// The address of the account in hexadecimal.
    address: String,
    /// The mosaic linked namespace names.
    names: Vec<String>,
}

impl AccountNamesDto {
    pub fn compact(&self) -> crate::Result<AccountName> {
        let address = Address::from_encoded(&self.address)?;

        Ok(AccountName {
            address,
            names: self.to_owned().names,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountPropertiesTransactionInfoDto {
    meta: TransactionMetaDto,
    transaction: AccountPropertiesTransactionDto,
}

/// AccountPropertiesTransactionDto :
/// Transaction that prevents receiving transactions from undesired
/// addresses, mosaics or sending certain transaction types.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountPropertiesTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    property_type: u8,
    modifications: Vec<AccountPropertiesModificationDto>,
}

#[typetag::serde]
impl TransactionDto for AccountPropertiesTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.compact();

        let abs_transaction = dto.r#abstract.compact(info)?;

        if dto.property_type & AccountPropertyType::AllowAddress.value() != 0 {
            let modifications: Vec<AccountPropertiesAddressModification> = dto
                .modifications
                .iter()
                .map(move |p| AccountPropertiesAddressModification {
                    modification_type: p.r#type,
                    address: Address::from_encoded(p.value.as_str().unwrap()).unwrap_or_default(),
                })
                .collect();

            Ok(Box::new(AccountPropertiesAddressTransaction {
                abs_transaction,
                property_type: AccountPropertyType::from(dto.property_type),
                modifications,
            }))
        } else if dto.property_type & AccountPropertyType::AllowMosaic.value() != 0 {
            let modifications: Vec<AccountPropertiesMosaicModification> = dto
                .modifications
                .iter()
                .map(move |p| AccountPropertiesMosaicModification {
                    modification_type: p.r#type.to_owned(),
                    asset_id: Box::new(MosaicId::from(
                        Uint64Dto::from_value(p.value.to_owned()).compact(),
                    )),
                })
                .collect();

            Ok(Box::new(AccountPropertiesMosaicTransaction {
                abs_transaction,
                property_type: AccountPropertyType::from(dto.property_type),
                modifications,
            }))
        } else if dto.property_type & AccountPropertyType::AllowTransaction.value() != 0 {
            let modifications: Vec<AccountPropertiesEntityTypeModification> = dto
                .modifications
                .iter()
                .map(move |p| AccountPropertiesEntityTypeModification {
                    modification_type: p.r#type.to_owned(),
                    transaction_type: EntityTypeEnum::from(p.value.as_u64().unwrap() as u16),
                })
                .collect();

            Ok(Box::new(AccountPropertiesEntityTypeTransaction {
                abs_transaction,
                property_type: AccountPropertyType::from(dto.property_type),
                modifications,
            }))
        } else {
            Err(Failure(format_err!("invalid AccountPropertyType")))
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct AccountPropertiesModificationDto {
    r#type: u8,
    /// The address, transaction type or mosaic id to filter.
    value: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountPropertiesInfoDto {
    account_properties: AccountPropertiesDto,
}

impl AccountPropertiesInfoDto {
    pub fn compact(&self) -> Result<AccountProperties> {
        let dto = self.account_properties.clone();

        let mut allowed_addresses: Vec<Address> = vec![];
        let mut allowed_mosaic_id: Vec<MosaicId> = vec![];
        let mut allowed_entity_types: Vec<EntityTypeEnum> = vec![];
        let mut blocked_addresses: Vec<Address> = vec![];
        let mut blocked_mosaic_id: Vec<MosaicId> = vec![];
        let mut blocked_entity_types: Vec<EntityTypeEnum> = vec![];

        dto.properties.iter().for_each(|p_dto| {
            if let Some(item) = p_dto.addresses.clone() {
                let property_addresses = item
                    .into_iter()
                    .map(|hex_address| Address::from_encoded(&hex_address).unwrap())
                    .collect();
                if p_dto.property_type == AccountPropertyType::AllowAddress.value() {
                    allowed_addresses = property_addresses;
                } else {
                    blocked_addresses = property_addresses;
                }
            };
        });

        dto.properties.iter().for_each(|p_dto| {
            if let Some(item) = p_dto.mosaic_ids.clone() {
                let property_mosaic_id = item
                    .into_iter()
                    .map(|mosaic_id_dto| MosaicId::from(mosaic_id_dto.compact()))
                    .collect();
                if p_dto.property_type == AccountPropertyType::AllowMosaic.value() {
                    allowed_mosaic_id = property_mosaic_id;
                } else {
                    blocked_mosaic_id = property_mosaic_id;
                }
            };
        });

        dto.properties.iter().for_each(|p_dto| {
            if let Some(item) = p_dto.entity_types.clone() {
                let property_entity_types = item
                    .into_iter()
                    .map(|entity_types_dto| EntityTypeEnum::from(entity_types_dto))
                    .collect();
                if p_dto.property_type == AccountPropertyType::AllowTransaction.value() {
                    allowed_entity_types = property_entity_types;
                } else {
                    blocked_entity_types = property_entity_types;
                }
            };
        });

        Ok(AccountProperties {
            address: Address::from_encoded(&dto.address)?,
            allowed_addresses,
            allowed_mosaic_id,
            allowed_entity_types,
            blocked_addresses,
            blocked_mosaic_id,
            blocked_entity_types,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPropertiesDto {
    address: String,
    properties: Vec<PropertiesDto>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PropertiesDto {
    property_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    mosaic_ids: Option<Vec<Uint64Dto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_types: Option<Vec<u16>>,
}

impl<'de> Deserialize<'de> for PropertiesDto {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct PropertyValue {
            #[serde(rename = "propertyType")]
            r#type: u8,
            values: Value,
        }

        let property = PropertyValue::deserialize(deserializer)?;

        let mut addresses = None;
        let mut mosaic_ids = None;
        let mut entity_types = None;

        if property.r#type & AccountPropertyType::AllowAddress.value() != 0 {
            addresses = Some(serde_json::from_value(property.values).unwrap());
        } else if property.r#type & AccountPropertyType::AllowMosaic.value() != 0 {
            mosaic_ids = Some(serde_json::from_value(property.values).unwrap())
        } else if property.r#type & AccountPropertyType::AllowTransaction.value() != 0 {
            entity_types = Some(serde_json::from_value(property.values).unwrap_or_default());
        };

        Ok(PropertiesDto {
            property_type: property.r#type,
            mosaic_ids,
            addresses,
            entity_types,
        })
    }
}
