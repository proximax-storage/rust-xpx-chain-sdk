use crate::models::{
    account::{Address, PublicAccount},
    blockchain::EmbeddedBlockchainUpgradeTransactionDto,
    message::MessageDto,
    mosaic::{Mosaic, MosaicDto},
    network::extract_network_type,
    uint_64::Uint64Dto,
};
use crate::models::transaction::TransactionInfo;

use super::{
    AbstractTransaction, deadline::{
        BlockchainTimestamp, Deadline,
    }, EntityTypeEnum, internal::extract_version, Transaction,
    TransactionStatus,
    TransferTransaction,
};

#[typetag::serde]
pub(crate) trait TransactionDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>>;
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct AbstractTransactionDto {
    pub signature: String,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
}

impl AbstractTransactionDto {
    pub(crate) fn new(signature: String,
                      signer: String,
                      version: i32,
                      _type: u16,
                      max_fee: Uint64Dto,
                      deadline: Uint64Dto,
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

        let signer = PublicAccount::from_public_key(
            &dto.signer, network_type)?;

        let blockchain_timestamp = BlockchainTimestamp::new(
            dto.deadline.to_struct().0 as i64);

        let deadline = Deadline::from(blockchain_timestamp);
        let max_fee = dto.max_fee.to_struct();

        let transaction_type = EntityTypeEnum::from(dto._type as u64);

        Ok(AbstractTransaction::new(Some(info),
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

#[derive(Serialize, Deserialize)]
pub(crate) struct TransactionInfoDto {
    #[serde(rename = "meta")]
    meta: TransactionMetaDto,
    #[serde(rename = "transaction")]
    transaction: EmbeddedBlockchainUpgradeTransactionDto,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransactionMetaDto {
    height: Uint64Dto,
    hash: String,
    merkle_component_hash: String,
    index: u32,
    id: String,
    aggregate_id: Option<String>,
    aggregate_hash: Option<String>,
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

        TransactionInfo {
            height: dto.height.to_struct(),
            index: dto.index,
            id: dto.id.clone(),
            transaction_hash: dto.hash.clone(),
            merkle_component_hash: dto.merkle_component_hash.clone(),
            agregate_hash,
            aggregate_id
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
            let blockchain_timestamp = BlockchainTimestamp::new(
                value.to_struct().0 as i64);
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
            height
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
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct(info)?;


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
    pub signature: String,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
    pub recipient: String,
    pub mosaics: Vec<MosaicDto>,
    pub message: MessageDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbeddedTransactionInfoDto {
    #[serde(rename = "meta")]
    pub meta: EmbeddedTransactionMetaDto,
    #[serde(rename = "transaction")]
    pub transaction: EmbeddedBlockchainUpgradeTransactionDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbeddedTransactionMetaDto {
    #[serde(rename = "height")]
    pub height: Uint64Dto,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "merkle_component_hash")]
    pub merkle_component_hash: String,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "id")]
    pub id: String,
}
