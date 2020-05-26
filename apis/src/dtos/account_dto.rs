// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use sdk::{
    account::{AccountInfo, AccountLinkTypeEnum, AccountName, Address},
    mosaic::Mosaic,
};

use super::{MosaicDto, Uint64Dto};

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
    pub(crate) fn to_struct(&self) -> crate::Result<AccountInfo> {
        let dto = &self.account;
        let add = Address::from_encoded(&dto.clone().address)?;
        let acc_type = AccountLinkTypeEnum::new(dto.clone().account_type);

        let mosaics: Vec<Mosaic> = dto
            .mosaics
            .iter()
            .map(move |mosaic_dto| mosaic_dto.to_struct())
            .collect();

        Ok(AccountInfo::new(
            add,
            dto.clone().address_height.to_struct(),
            dto.clone().public_key,
            dto.public_key_height.to_struct(),
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
    /// The signature of the entity.
    /// The signature was generated by the signer and can be used to validate
    /// tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0x68 (MAIN_NET) - PUBLIC main network.
    /// * 0x98 (TEST_NET) - PUBLIC test network.
    /// * 0x60 (MIJIN) - PRIVATE network.
    /// * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    /// The public key of the remote account.
    remote_account_key: String,
    action: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct AccountNamesDto {
    /// The address of the account in hexadecimal.
    #[serde(rename = "address")]
    address: String,
    /// The mosaic linked namespace names.
    #[serde(rename = "names")]
    names: Vec<String>,
}

impl AccountNamesDto {
    pub fn to_struct(&self) -> crate::Result<AccountName> {
        let address = Address::from_encoded(&self.address)?;

        Ok(AccountName {
            address,
            names: self.to_owned().names,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct AccountPropertiesDto {
    /// The address of the account in hexadecimal.
    #[serde(rename = "address")]
    address: String,
    #[serde(rename = "properties")]
    properties: Vec<AccountPropertyDto>,
}

#[derive(Serialize, Deserialize)]
struct AccountPropertiesInfoDto {
    #[serde(rename = "accountProperties")]
    account_properties: AccountPropertiesDto,
}

#[derive(Serialize, Deserialize)]
struct AccountPropertiesModificationDto {
    #[serde(rename = "type")]
    _type: u16,
    /// The address, transaction type or mosaic id to filter.
    #[serde(rename = "values")]
    values: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
struct AccountPropertiesTransactionBodyDto {
    #[serde(rename = "propertyType")]
    property_type: u32,
    #[serde(rename = "modifications")]
    modifications: Vec<AccountPropertiesModificationDto>,
}

/// AccountPropertiesTransactionDto : Transaction that prevents receiving transactions from undesired addresses, mosaics or sending certain transaction types.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountPropertiesTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and
    /// can be used to validate tha the entity data was not modified by a node.
    signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0x68 (MAIN_NET) - PUBLIC main network.
    /// * 0x98 (TEST_NET) - PUBLIC test network.
    /// * 0x60 (MIJIN) - PRIVATE network.
    /// * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    property_type: u32,
    modifications: Vec<AccountPropertiesModificationDto>,
}

#[derive(Serialize, Deserialize)]
struct AccountPropertyDto {
    #[serde(rename = "propertyType")]
    property_type: u32,
    /// The address, transaction type or mosaic id to filter.
    #[serde(rename = "values")]
    values: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmbeddedAccountLinkTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0x68 (MAIN_NET) - PUBLIC main network.
    /// * 0x98 (TEST_NET) - PUBLIC test network.
    /// * 0x60 (MIJIN) - PRIVATE network.
    /// * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    /// The public key of the remote account.
    remote_account_key: String,
    action: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmbeddedAccountPropertiesTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    signer: String,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0x68 (MAIN_NET) - PUBLIC main network.
    /// * 0x98 (TEST_NET) - PUBLIC test network.
    /// * 0x60 (MIJIN) - PRIVATE network.
    /// * 0x90 (MIJIN_TEST) - PRIVATE test network.
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    property_type: u8,
    modifications: Vec<AccountPropertiesModificationDto>,
}
