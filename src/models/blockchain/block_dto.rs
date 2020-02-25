use crate::models::account::{EMPTY_PUBLIC_KEY, PublicAccount};
use crate::models::network::network_internal::extract_network_type;
use crate::models::transaction::BlockchainTimestamp;
use crate::models::transaction::internal::extract_version;
use crate::models::uint_64::Uint64Dto;
use crate::Result;

use super::block_model::BlockInfo;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockMetaDto {
    hash: String,
    generation_hash: String,
    sub_cache_merkle_roots: Vec<String>,
    total_fee: Uint64Dto,
    num_transactions: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_statements: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockDto {
    signature: String,
    signer: String,
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    height: Uint64Dto,
    timestamp: Uint64Dto,
    difficulty: Uint64Dto,
    fee_multiplier: i32,
    previous_block_hash: String,
    block_transactions_hash: String,
    block_receipts_hash: String,
    state_hash: String,
    beneficiary: String,
    fee_interest: u32,
    fee_interest_denominator: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BlockInfoDto {
    #[serde(rename = "meta")]
    meta: BlockMetaDto,
    #[serde(rename = "block")]
    block: BlockDto,
}

impl BlockInfoDto {
    pub(crate) fn to_struct(self) -> Result<BlockInfo> {
        let dto = self.block;

        let network_type = extract_network_type(dto.version);

        let signer_public_account = PublicAccount::from_public_key(
            &dto.signer, network_type)?;

        let version = extract_version(dto.version);

        let mut beneficiary_public_account = Option::default();
        if dto.beneficiary != EMPTY_PUBLIC_KEY {
            beneficiary_public_account = Some(PublicAccount::from_public_key(
                &dto.beneficiary, network_type)?);
        }

        Ok(BlockInfo::new(
            network_type,
            dto.signature,
            signer_public_account,
            version,
            dto._type,
            dto.height.to_struct(),
            BlockchainTimestamp::new(
                dto.timestamp.to_struct().0 as i64).to_timestamp(),
            dto.difficulty.to_struct(),
            self.meta.num_transactions,
            dto.fee_multiplier,
            self.meta.generation_hash,
            dto.previous_block_hash,
            dto.block_transactions_hash,
            dto.block_receipts_hash,
            dto.state_hash,
            beneficiary_public_account,
            dto.fee_interest,
            self.meta.total_fee.to_struct(),
            dto.fee_interest_denominator,
        ))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockDtoAllOf {
    height: Uint64Dto,
    timestamp: Uint64Dto,
    difficulty: Uint64Dto,
    /// The fee multiplier applied to transactions contained in block.
    fee_multiplier: i32,
    /// The hash of the previous block.
    previous_block_hash: String,
    /// The transactions included in a block are hashed forming a merkle tree. The root of the tree summarizes them.
    block_transactions_hash: String,
    /// The collection of receipts  are hashed into a merkle tree and linked  to a block. The block header stores the root hash.
    block_receipts_hash: String,
    /// For each block, the state of the blockchain is stored in RocksDB,  forming a patricia tree. The root of the tree summarizes the state of the blockchain for the given block.
    state_hash: String,
    /// The public key of the optional beneficiary designated by harvester.
    beneficiary: String,
    /// The part of the transaction fee harvester is willing to get. From 0 up to FeeInterestDenominator. The customer gets (FeeInterest / FeeInterestDenominator)'th part of the maximum transaction fee.
    fee_interest: i32,
    /// Denominator of the transaction fee.
    fee_interest_denominator: i32,
}
