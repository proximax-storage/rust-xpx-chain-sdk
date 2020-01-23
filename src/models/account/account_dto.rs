/// AccountLinkTypeEnum : The account link types: * 0 -  Unlinked. Account is not linked to another account. * 1 -  Main. Account is a balance-holding account that is linked to a remote harvester account. * 2 -  Remote. Account is a remote harvester account that is linked to a balance-holding account. * 3 -  Remote_Unlinked. Account is a remote harvester eligible account that is unlinked.
/// The account link types: * 0 -  Unlinked. Account is not linked to another account. * 1 -  Main. Account is a balance-holding account that is linked to a remote harvester account. * 2 -  Remote. Account is a remote harvester account that is linked to a balance-holding account. * 3 -  Remote_Unlinked. Account is a remote harvester eligible account that is unlinked.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountLinkTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,

}

/// AccountPropertiesModificationTypeEnum : The account properties modification type: * 0 - Add property. * 1 - Remove property.
/// The account properties modification type: * 0 - Add property. * 1 - Remove property.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountPropertiesModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}

/// AccountPropertyTypeEnum : The account properties type: * 0x01 (1 decimal) - The property type only allows receiving transactions from an address. * 0x02 (2 decimal) - The property type only allows receiving transactions containing a mosaic id. * 0x04 (4 decimal) - The property type only allows sending transactions with a given transaction type. * 0x05 (5 decimal) - Property type sentinel. * 0x81 (129 decimal) - The property type blocks receiving transactions from an address. * 0x82 (130 decimal) - The property type blocks receiving transactions containing a mosaic id. * 0x84 (132 decimal) -  The property type blocks sending transactions with a given transaction type.
/// The account properties type: * 0x01 (1 decimal) - The property type only allows receiving transactions from an address. * 0x02 (2 decimal) - The property type only allows receiving transactions containing a mosaic id. * 0x04 (4 decimal) - The property type only allows sending transactions with a given transaction type. * 0x05 (5 decimal) - Property type sentinel. * 0x81 (129 decimal) - The property type blocks receiving transactions from an address. * 0x82 (130 decimal) - The property type blocks receiving transactions containing a mosaic id. * 0x84 (132 decimal) -  The property type blocks sending transactions with a given transaction type.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountPropertyTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "4")]
    _4,
    #[serde(rename = "5")]
    _5,
    #[serde(rename = "129")]
    _129,
    #[serde(rename = "130")]
    _130,
    #[serde(rename = "132")]
    _132,

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountMetaDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "merkleComponentHash")]
    pub merkle_component_hash: String,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "id")]
    pub id: String,
}

impl AccountMetaDto {
    pub fn new(height: Vec<i32>, hash: String, merkle_component_hash: String, index: i32, id: String) -> AccountMetaDto {
        AccountMetaDto {
            height,
            hash,
            merkle_component_hash,
            index,
            id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDto {
    /// The account unique address in hexadecimal. 
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "addressHeight")]
    pub address_height: Vec<i32>,
    /// The public key of an account can be used to verify signatures of the account. Only accounts that have already published a transaction have a public key assigned to the account. Otherwise, the field is null. 
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "publicKeyHeight")]
    pub public_key_height: Vec<i32>,
    /// The list of mosaics the account owns. The amount is represented in absolute amount. Thus a balance of 123456789 for a mosaic with divisibility 6 (absolute) means the account owns 123.456789 instead. 
    #[serde(rename = "mosaics")]
    pub mosaics: Vec<crate::models::MosaicDto>,
    #[serde(rename = "accountType")]
    pub account_type: crate::models::AccountLinkTypeEnum,
    /// The public key of a linked account. The linked account can use|provide balance for delegated harvesting. 
    #[serde(rename = "linkedAccountKey")]
    pub linked_account_key: String,
}

impl AccountDto {
    pub fn new(address: String, address_height: Vec<i32>, public_key: String, public_key_height: Vec<i32>, mosaics: Vec<crate::models::MosaicDto>, account_type: crate::models::AccountLinkTypeEnum, linked_account_key: String) -> AccountDto {
        AccountDto {
            address,
            address_height,
            public_key,
            public_key_height,
            mosaics,
            account_type,
            linked_account_key,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountIds {
    /// The array of public keys.
    #[serde(rename = "publicKeys", skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    /// The array of addresses.
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
}

impl AccountIds {
    pub fn new() -> AccountIds {
        AccountIds {
            public_keys: None,
            addresses: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::AccountMetaDto,
    #[serde(rename = "account")]
    pub account: crate::models::AccountDto,
}

impl AccountInfoDto {
    pub fn new(meta: crate::models::AccountMetaDto, account: crate::models::AccountDto) -> AccountInfoDto {
        AccountInfoDto {
            meta,
            account,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountLinkTransactionBodyDto {
    /// The public key of the remote account.
    #[serde(rename = "remoteAccountKey")]
    pub remote_account_key: String,
    #[serde(rename = "action")]
    pub action: crate::models::LinkActionEnum,
}

impl AccountLinkTransactionBodyDto {
    pub fn new(remote_account_key: String, action: crate::models::LinkActionEnum) -> AccountLinkTransactionBodyDto {
        AccountLinkTransactionBodyDto {
            remote_account_key,
            action,
        }
    }
}

/// AccountLinkTransactionDto : Delegates the account importance score to a proxy account.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountLinkTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    pub signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// The public key of the remote account.
    #[serde(rename = "remoteAccountKey")]
    pub remote_account_key: String,
    #[serde(rename = "action")]
    pub action: crate::models::LinkActionEnum,
}

impl AccountLinkTransactionDto {
    /// Delegates the account importance score to a proxy account.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, remote_account_key: String, action: crate::models::LinkActionEnum) -> AccountLinkTransactionDto {
        AccountLinkTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            remote_account_key,
            action,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountNamesDto {
    /// The address of the account in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    /// The mosaic linked namespace names.
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

impl AccountNamesDto {
    pub fn new(address: String, names: Vec<String>) -> AccountNamesDto {
        AccountNamesDto {
            address,
            names,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesDto {
    /// The address of the account in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::AccountPropertyDto>,
}

impl AccountPropertiesDto {
    pub fn new(address: String, properties: Vec<crate::models::AccountPropertyDto>) -> AccountPropertiesDto {
        AccountPropertiesDto {
            address,
            properties,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesInfoDto {
    #[serde(rename = "accountProperties")]
    pub account_properties: crate::models::AccountPropertiesDto,
}

impl AccountPropertiesInfoDto {
    pub fn new(account_properties: crate::models::AccountPropertiesDto) -> AccountPropertiesInfoDto {
        AccountPropertiesInfoDto {
            account_properties,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesModificationDto {
    #[serde(rename = "type")]
    pub _type: crate::models::AccountPropertiesModificationTypeEnum,
    /// The address, transaction type or mosaic id to filter.
    #[serde(rename = "values")]
    pub values: Vec<i32>,
}

impl AccountPropertiesModificationDto {
    pub fn new(_type: crate::models::AccountPropertiesModificationTypeEnum, values: Vec<i32>) -> AccountPropertiesModificationDto {
        AccountPropertiesModificationDto {
            _type,
            values,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesTransactionBodyDto {
    #[serde(rename = "propertyType")]
    pub property_type: crate::models::AccountPropertyTypeEnum,
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::AccountPropertiesModificationDto>,
}

impl AccountPropertiesTransactionBodyDto {
    pub fn new(property_type: crate::models::AccountPropertyTypeEnum, modifications: Vec<crate::models::AccountPropertiesModificationDto>) -> AccountPropertiesTransactionBodyDto {
        AccountPropertiesTransactionBodyDto {
            property_type,
            modifications,
        }
    }
}

/// AccountPropertiesTransactionDto : Transaction that prevents receiving transactions from undesired addresses, mosaics or sending certain transaction types.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertiesTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    pub signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "propertyType")]
    pub property_type: crate::models::AccountPropertyTypeEnum,
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::AccountPropertiesModificationDto>,
}

impl AccountPropertiesTransactionDto {
    /// Transaction that prevents receiving transactions from undesired addresses, mosaics or sending certain transaction types.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, property_type: crate::models::AccountPropertyTypeEnum, modifications: Vec<crate::models::AccountPropertiesModificationDto>) -> AccountPropertiesTransactionDto {
        AccountPropertiesTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            property_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountPropertyDto {
    #[serde(rename = "propertyType")]
    pub property_type: crate::models::AccountPropertyTypeEnum,
    /// The address, transaction type or mosaic id to filter.
    #[serde(rename = "values")]
    pub values: Vec<i32>,
}

impl AccountPropertyDto {
    pub fn new(property_type: crate::models::AccountPropertyTypeEnum, values: Vec<i32>) -> AccountPropertyDto {
        AccountPropertyDto {
            property_type,
            values,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedAccountLinkTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// The public key of the remote account.
    #[serde(rename = "remoteAccountKey")]
    pub remote_account_key: String,
    #[serde(rename = "action")]
    pub action: crate::models::LinkActionEnum,
}

impl EmbeddedAccountLinkTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, remote_account_key: String, action: crate::models::LinkActionEnum) -> EmbeddedAccountLinkTransactionDto {
        EmbeddedAccountLinkTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            remote_account_key,
            action,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedAccountPropertiesTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "propertyType")]
    pub property_type: crate::models::AccountPropertyTypeEnum,
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::AccountPropertiesModificationDto>,
}

impl EmbeddedAccountPropertiesTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, property_type: crate::models::AccountPropertyTypeEnum, modifications: Vec<crate::models::AccountPropertiesModificationDto>) -> EmbeddedAccountPropertiesTransactionDto {
        EmbeddedAccountPropertiesTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            property_type,
            modifications,
        }
    }
}
