use ::std::any::Any;
use ::std::fmt;

use crate::models::{
    account::{Address, PublicAccount},
    blockchain::EmbeddedBlockchainUpgradeTransactionDto,
    message::{MessageDto, MessageType::PlainMessageType, PlainMessage},
    mosaic::{Mosaic, MosaicDto},
    network::network_internal::extract_network_type,
    uint_64::Uint64Dto,
};

use super::{
    AbstractTransaction, deadline::{
        BlockchainTimestamp, Deadline,
    }, EntityTypeEnum, internal::extract_version, Transaction,
    TransactionStatus,
    TransferTransaction,
};

#[typetag::serde]
pub trait TransactionDto where
    Self: fmt::Debug,
{
    fn version(&self) -> i32;
    fn to_struct(&self) -> Box<dyn Transaction>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractTransactionDto {
    pub signature: String,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
}

impl AbstractTransactionDto {
    fn new(signature: String,
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

    fn to_struct(&self) -> AbstractTransaction {
        let dto = self;

        let network_type = extract_network_type(self.version);

        let version = extract_version(self.version);

        let signer = PublicAccount::from_public_key(
            &dto.signer, network_type,
        ).unwrap();

        let blockchain_timestamp = BlockchainTimestamp::new(
            dto.deadline.to_struct().0 as i64
        );

        let deadline = Deadline::from(blockchain_timestamp);
        let max_fee = dto.max_fee.to_struct();

        let transaction_type = EntityTypeEnum::from(dto._type as u64);

        AbstractTransaction::new(None,
                                 dto.signature.clone(),
                                 signer,
                                 version,
                                 transaction_type,
                                 max_fee,
                                 deadline,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TransactionInfoDto {
    #[serde(rename = "meta")]
    meta: TransactionMetaDto,
    #[serde(rename = "transaction")]
    transaction: EmbeddedBlockchainUpgradeTransactionDto,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetaDto {
    height: Uint64Dto,
    hash: String,
    merkle_component_hash: String,
    index: u32,
    id: String,
    aggregate_id: Option<String>,
    aggregate_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn to_struct(&self) -> TransactionStatus {
        let dto = &self.to_owned();

        let deadline = loop {
            match &dto.deadline {
                Some(d) => break BlockchainTimestamp::new(d.to_struct().0 as i64),
                _ => {}
            }
        };

        TransactionStatus::new(
            dto.group.clone().unwrap(),
            dto.status.clone(),
            dto.hash.clone().unwrap(),
            Deadline::from(deadline),
            dto.height.clone().unwrap().to_struct(),
        )
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: TransferTransactionDto,
}

#[typetag::serde]
impl TransactionDto for TransferTransactionInfoDto {
    fn version(&self) -> i32 {
        self.transaction.version
    }

    fn to_struct(&self) -> Box<dyn Transaction> {
        let dto = self.transaction.clone();

        let abs = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct();


        let mut mosaics: Vec<Mosaic> = vec![];

        for mosaic in dto.mosaics {
            mosaics.push(mosaic.to_struct());
        }

        let recipient = Address::from_encoded(&dto.recipient).unwrap();

        Box::new(TransferTransaction {
            abs_transaction: abs,
            recipient,
            mosaics,
            message: dto.message.to_struct(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// TransferTransactionDto : Transaction that transfers mosaics and messages to another account.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransactionDto {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedTransactionInfoDto {
    #[serde(rename = "meta")]
    pub meta: EmbeddedTransactionMetaDto,
    #[serde(rename = "transaction")]
    pub transaction: EmbeddedBlockchainUpgradeTransactionDto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedTransactionMetaDto {
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
