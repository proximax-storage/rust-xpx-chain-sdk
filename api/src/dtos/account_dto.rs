// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use {
    sdk::{
        account::{
            AccountInfo, AccountLinkTypeEnum, AccountName, AccountPropertiesAddressModification,
            AccountPropertiesMosaicModification, AccountPropertyType, Address,
        },
        mosaic::{Mosaic, MosaicId},
        transaction::{
            AccountPropertiesAddressTransaction, AccountPropertiesMosaicTransaction, Transaction,
        },
    },
    serde::Serialize,
    serde_json::Value,
};

use crate::error::Error::Failure;

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

#[derive(Serialize, Deserialize)]
struct AccountLinkTransactionBodyDto {
    /// The public key of the remote account.
    #[serde(rename = "remoteAccountKey")]
    remote_account_key: String,
    #[serde(rename = "action")]
    action: u8,
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
pub struct AccountPropertiesTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: AccountPropertiesTransactionDto,
}

/// AccountPropertiesTransactionDto :
/// Transaction that prevents receiving transactions from undesired
/// addresses, mosaics or sending certain transaction types.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPropertiesTransactionDto {
    #[serde(flatten)]
    pub r#abstract: AbstractTransactionDto,
    pub property_type: u8,
    pub modifications: Vec<AccountPropertiesModificationDto>,
}

#[typetag::serde]
impl TransactionDto for AccountPropertiesTransactionInfoDto {
    fn compact(&self) -> crate::Result<Box<dyn Transaction>> {
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
        } else {
            Err(Failure(format_err!("invalid AccountPropertyType")))
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AccountPropertiesModificationDto {
    pub r#type: u8,
    /// The address, transaction type or mosaic id to filter.
    pub value: Value,
}
