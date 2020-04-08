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
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>>;
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct AbstractTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: u32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
}

impl AbstractTransactionDto {
    pub(crate) fn new(
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
            _type,
            max_fee,
            deadline,
        }
    }

    pub(crate) fn to_struct(&self, info: TransactionInfo) -> crate::Result<AbstractTransaction> {
        let dto = self;

        let network_type = extract_network_type(self.version);

        let version = extract_version(self.version);

        let signer = PublicAccount::from_public_key(&dto.signer, network_type)?;

        let mut deadline = None;
        if let Some(item) = &dto.deadline {
            let timestamp = BlockchainTimestamp::new(item.to_struct().to_u64() as i64);
            deadline = Some(Deadline::from(timestamp))
        }

        let mut max_fee = None;
        if let Some(item) = &dto.max_fee {
            max_fee = Some(item.to_struct());
        }

        let transaction_type = EntityTypeEnum::from(dto._type);

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
pub(crate) struct TransactionMetaDto {
    height: Uint64Dto,
    index: u32,
    id: String,
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
    pub fn to_struct(&self) -> TransactionInfo {
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

        TransactionInfo {
            height: dto.height.to_struct(),
            index: dto.index,
            id: dto.id.clone(),
            transaction_hash,
            merkle_component_hash,
            agregate_hash,
            aggregate_id,
            unique_aggregate_hash,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransactionStatusDto {
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
    pub fn to_struct(&self) -> TransactionStatus {
        let dto = &self.to_owned();

        let mut deadline = None;
        if let Some(value) = &dto.deadline {
            let blockchain_timestamp = BlockchainTimestamp::new(value.to_struct().to_u64() as i64);
            deadline = Some(Deadline::from(blockchain_timestamp));
        };

        let mut height = None;
        if let Some(value) = &dto.height {
            height = Some(value.to_struct());
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
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.to_struct();

        let abs = AbstractTransactionDto::new(
            dto.signature,
            dto.signer,
            dto.version,
            dto._type,
            dto.max_fee,
            dto.deadline,
        )
        .to_struct(info)?;

        let mut mosaics: Vec<Mosaic> = vec![];

        for mosaic in dto.mosaics {
            mosaics.push(mosaic.to_struct());
        }

        let recipient = Address::from_encoded(&dto.recipient)?;

        Ok(Box::new(TransferTransaction {
            abs_transaction: abs,
            recipient,
            mosaics,
            message: dto.message.to_struct(),
        }))
    }
}

/// TransferTransactionDto : Transaction that transfers mosaics and messages to another account.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransferTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: u32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    pub recipient: String,
    pub mosaics: Vec<MosaicDto>,
    pub message: MessageDto,
}

/// HashLockTransactionDto :
/// Transaction to lock funds before sending an aggregate bonded transaction.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HashLockTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: u32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    pub mosaic_id: Uint64Dto,
    pub amount: Uint64Dto,
    pub duration: Uint64Dto,
    pub hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HashLockTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: HashLockTransactionDto,
}

#[typetag::serde]
impl TransactionDto for HashLockTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.to_struct();

        let abs = AbstractTransactionDto::new(
            dto.signature,
            dto.signer,
            dto.version,
            dto._type,
            dto.max_fee,
            dto.deadline,
        )
        .to_struct(info)?;

        let mosaic = Mosaic::new(
            MosaicId::from(dto.mosaic_id.to_struct()),
            dto.amount.to_struct(),
        );

        Ok(Box::new(LockFundsTransaction {
            abs_transaction: abs,
            mosaic,
            duration: dto.duration.to_struct(),
            signed_transaction: SignedTransaction::new(
                EntityTypeEnum::AggregateBonded,
                "".to_string(),
                dto.hash.to_string(),
            ),
        }))
    }
}
