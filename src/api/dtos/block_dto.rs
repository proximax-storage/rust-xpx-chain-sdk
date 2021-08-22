/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::std::str::FromStr;

use crate::{
    account::{EMPTY_PUBLIC_KEY, PublicAccount},
    blockchain::BlockInfo,
    network::extract_network_type,
    Result,
    transaction::{BlockchainTimestamp, HashValue, internal::extract_version},
};

use super::Uint64Dto;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockMetaDto {
    hash: String,
    generation_hash: String,
    total_fee: Uint64Dto,
    num_transactions: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BlockDto {
    signature: String,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u32,
    pub height: Uint64Dto,
    timestamp: Uint64Dto,
    difficulty: Uint64Dto,
    #[serde(skip_serializing_if = "Option::is_none")]
    fee_multiplier: Option<i32>,
    previous_block_hash: String,
    block_transactions_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_receipts_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_hash: Option<String>,
    beneficiary: Option<String>,
    fee_interest: Option<u32>,
    fee_interest_denominator: Option<u32>,
}

impl BlockDto {
    pub fn compact(
        self,
        generation_hash: String,
        num_transactions: u64,
        total_fee: [u32; 2],
    ) -> Result<BlockInfo> {
        let dto = self;

        let network_type = extract_network_type(dto.version as u32);

        let signer_public_account = PublicAccount::from_public_key(&dto.signer, network_type)?;

        let version = extract_version(dto.version as u32);

        let beneficiary_public_account = if let Some(v) = dto.beneficiary {
            if v != EMPTY_PUBLIC_KEY {
                Some(PublicAccount::from_public_key(&v, network_type)?)
            } else {
                None
            }
        } else {
            Option::default()
        };

        let fee_multiplier = if let Some(v) = dto.fee_multiplier {
            v
        } else {
            0
        };

        let block_receipts_hash = if let Some(v) = dto.block_receipts_hash {
            HashValue::from_str(&v)?
        } else {
            HashValue::zero()
        };

        let state_hash = if let Some(v) = dto.state_hash {
            HashValue::from_str(&v)?
        } else {
            HashValue::zero()
        };

        let fee_interest = if let Some(v) = dto.fee_interest { v } else { 0 };

        let fee_interest_denominator = if let Some(v) = dto.fee_interest_denominator {
            v
        } else {
            0
        };

        Ok(BlockInfo {
            network_type,
            signature: dto.signature,
            signer: signer_public_account,
            version,
            ver_type: dto._type,
            height: dto.height.compact(),
            timestamp: BlockchainTimestamp::new(*dto.timestamp.compact() as i64).to_timestamp(),
            difficulty: dto.difficulty.compact(),
            num_transactions,
            fee_multiplier,
            generation_hash: HashValue::from_str(&generation_hash)?,
            previous_block_hash: HashValue::from_str(&dto.previous_block_hash)?,
            block_transactions_hash: HashValue::from_str(&dto.block_transactions_hash)?,
            block_receipts_hash,
            state_hash,
            beneficiary: beneficiary_public_account,
            fee_interest,
            total_fee: Uint64Dto(total_fee).compact(),
            fee_interest_denominator,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BlockInfoDto {
    #[serde(rename = "meta")]
    meta: BlockMetaDto,
    #[serde(rename = "block")]
    block: BlockDto,
}

impl BlockInfoDto {
    pub fn compact(self) -> Result<BlockInfo> {
        self.block.compact(
            self.meta.generation_hash,
            self.meta.num_transactions,
            self.meta.total_fee.0,
        )
    }
}

#[derive(Serialize, Deserialize)]
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
