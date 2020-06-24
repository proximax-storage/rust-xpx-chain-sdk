// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use sdk::{
    account::{Address, PublicAccount},
    mosaic::{Mosaic, MosaicId},
    network::extract_network_type,
    transaction::{
        deadline::{BlockchainTimestamp, Deadline},
        internal::extract_version,
        AbstractTransaction, EntityTypeEnum, LockFundsTransaction, SignedTransaction, Transaction,
        TransactionInfo, TransactionStatus, TransferTransaction,
    },
};

use super::{MessageDto, MosaicDto, Uint64Dto};

/// HashAlgorithmEnum : The hash algorithm used to hash te proof: * 0 (Op_Sha3_256)  - The proof is hashed using sha3 256. * 1 (Op_Keccak_256)  - The proof is hashed using Keccak (ETH compatibility). * 2 (Op_Hash_160)  - The proof is hashed twice: first with Sha-256 and then with RIPEMD-160 (bitcoin’s OP_HASH160). * 3 (Op_Hash_256)  - The proof is hashed twice with Sha-256 (bitcoin’s OP_HASH256).
/// The hash algorithm used to hash te proof: * 0 (Op_Sha3_256)  - The proof is hashed using sha3 256. * 1 (Op_Keccak_256)  - The proof is hashed using Keccak (ETH compatibility). * 2 (Op_Hash_160)  - The proof is hashed twice: first with Sha-256 and then with RIPEMD-160 (bitcoin’s OP_HASH160). * 3 (Op_Hash_256)  - The proof is hashed twice with Sha-256 (bitcoin’s OP_HASH256).
#[derive(Serialize, Deserialize)]
pub enum HashAlgorithmEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
}

#[typetag::serde]
pub trait TransactionDto {
    fn compact(&self) -> crate::Result<Box<dyn Transaction>>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AbstractTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and
    /// can be used to validate tha the entity data was not modified by a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// The public key of the entity signer formatted as hexadecimal.
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier:
    /// * 0x68 (MAIN_NET) - PUBLIC main network.
    /// * 0x98 (TEST_NET) - PUBLIC test network.
    /// * 0x60 (MIJIN) - PRIVATE network.
    /// * 0x90 (MIJIN_TEST) - PRIVATE test network.
    pub version: u32,
    pub r#type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
}

impl AbstractTransactionDto {
    pub fn new(
        signature: Option<String>,
        signer: String,
        version: u32,
        _type: u16,
        max_fee: Option<Uint64Dto>,
        deadline: Option<Uint64Dto>,
    ) -> Self {
        AbstractTransactionDto {
            signature,
            signer,
            version,
            r#type: _type,
            max_fee,
            deadline,
        }
    }

    pub fn compact(&self, info: TransactionInfo) -> crate::Result<AbstractTransaction> {
        let dto = self;

        let network_type = extract_network_type(self.version as u32);

        let version = extract_version(self.version as u32);

        let signer = PublicAccount::from_public_key(&dto.signer, network_type)?;

        let mut deadline = None;
        if let Some(item) = &dto.deadline {
            let timestamp = BlockchainTimestamp::new(*item.compact() as i64);
            deadline = Some(Deadline::from(timestamp))
        }

        let mut max_fee = None;
        if let Some(item) = &dto.max_fee {
            max_fee = Some(item.compact());
        }

        let transaction_type = EntityTypeEnum::from(dto.r#type);

        Ok(AbstractTransaction::new(
            Some(info),
            network_type,
            dto.signature.clone(),
            signer,
            version,
            transaction_type,
            max_fee,
            deadline,
        ))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetaDto {
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    height: Option<Uint64Dto>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    index: Option<u32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    transaction_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merkle_component_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_aggregate_hash: Option<String>,
}

impl TransactionMetaDto {
    pub fn compact(&self) -> TransactionInfo {
        let dto = self.clone();

        let mut agregate_hash = None;
        if let Some(t) = dto.aggregate_hash.clone() {
            agregate_hash = Some(t)
        }

        let mut aggregate_id = None;
        if let Some(t) = dto.aggregate_id.clone() {
            aggregate_id = Some(t)
        }

        let mut unique_aggregate_hash = None;
        if let Some(t) = dto.unique_aggregate_hash.clone() {
            unique_aggregate_hash = Some(t)
        }

        let mut transaction_hash = None;
        if let Some(t) = dto.transaction_hash.clone() {
            transaction_hash = Some(t)
        }

        let mut merkle_component_hash = None;
        if let Some(t) = dto.merkle_component_hash.clone() {
            merkle_component_hash = Some(t)
        }

        let mut index = 0;
        if let Some(i) = dto.index {
            index = i
        }

        let mut id = "".to_string();
        if let Some(i) = dto.id {
            id = i
        }

        let mut height = sdk::Uint64::default();
        if let Some(h) = dto.height {
            height = h.compact()
        }

        TransactionInfo {
            height,
            index,
            id,
            hash: transaction_hash,
            merkle_component_hash,
            agregate_hash,
            aggregate_id,
            unique_aggregate_hash,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatusDto {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    hash: Option<String>,
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    deadline: Option<Uint64Dto>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    height: Option<Uint64Dto>,
}

impl TransactionStatusDto {
    pub fn compact(&self) -> TransactionStatus {
        let dto = &self.to_owned();

        let mut deadline = None;
        if let Some(value) = &dto.deadline {
            let blockchain_timestamp = BlockchainTimestamp::new(*value.compact() as i64);
            deadline = Some(Deadline::from(blockchain_timestamp));
        };

        let mut height = None;
        if let Some(value) = &dto.height {
            height = Some(value.compact());
        };

        TransactionStatus::new(
            dto.group.clone().unwrap(),
            dto.status.clone(),
            dto.hash.clone().unwrap(),
            deadline,
            height,
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransferTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: TransferTransactionDto,
}

#[typetag::serde]
impl TransactionDto for TransferTransactionInfoDto {
    fn compact(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.compact();

        let abs_transaction = dto.r#abstract.compact(info)?;

        let mut mosaics: Vec<Mosaic> = vec![];
        if let Some(value) = &dto.mosaics {
            for mosaic in value {
                mosaics.push(mosaic.compact());
            }
        };

        let recipient = Address::from_encoded(&dto.recipient)?;

        Ok(Box::new(TransferTransaction {
            abs_transaction,
            recipient,
            mosaics,
            message: dto.message.compact(),
        }))
    }
}

/// TransferTransactionDto : Transaction that transfers mosaics and messages to another account.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransferTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    pub recipient: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosaics: Option<Vec<MosaicDto>>,
    pub message: MessageDto,
}

/// HashLockTransactionDto :
/// Transaction to lock funds before sending an aggregate bonded transaction.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HashLockTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    mosaic_id: Uint64Dto,
    amount: Uint64Dto,
    duration: Uint64Dto,
    hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HashLockTransactionInfoDto {
    meta: TransactionMetaDto,
    transaction: HashLockTransactionDto,
}

#[typetag::serde]
impl TransactionDto for HashLockTransactionInfoDto {
    fn compact(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.compact();

        let abs_transaction = dto.r#abstract.compact(info)?;

        let mosaic = Mosaic::new(
            MosaicId::from(dto.mosaic_id.compact()),
            dto.amount.compact(),
        );

        Ok(Box::new(LockFundsTransaction {
            abs_transaction,
            mosaic,
            duration: dto.duration.compact(),
            signed_transaction: SignedTransaction::new(
                EntityTypeEnum::AggregateBonded,
                "".to_string(),
                dto.hash.to_string(),
            ),
        }))
    }
}