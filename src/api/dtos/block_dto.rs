/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
    account::{PublicAccount, EMPTY_PUBLIC_KEY},
    blockchain::BlockInfo,
    network::extract_network_type,
    transaction::{extract_version, BlockchainTimestamp},
    Result,
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
pub struct BlockDto {
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

        let mut beneficiary_public_account = Option::default();
        if let Some(v) = dto.beneficiary {
            if v != EMPTY_PUBLIC_KEY {
                beneficiary_public_account =
                    Some(PublicAccount::from_public_key(&v, network_type)?);
            }
        }

        let mut fee_multiplier = 0;
        if let Some(v) = dto.fee_multiplier {
            fee_multiplier = v;
        }

        let mut block_receipts_hash = "".to_string();
        if let Some(v) = dto.block_receipts_hash {
            block_receipts_hash = v;
        }

        let mut state_hash = "".to_string();
        if let Some(v) = dto.state_hash {
            state_hash = v;
        }

        let mut fee_interest = 0;
        if let Some(v) = dto.fee_interest {
            fee_interest = v;
        }

        let mut fee_interest_denominator = 0;
        if let Some(v) = dto.fee_interest_denominator {
            fee_interest_denominator = v;
        }

        Ok(BlockInfo::new(
            network_type,
            dto.signature,
            signer_public_account,
            version,
            dto._type,
            dto.height.compact(),
            BlockchainTimestamp::new(*dto.timestamp.compact() as i64).to_timestamp(),
            dto.difficulty.compact(),
            num_transactions,
            fee_multiplier,
            generation_hash,
            dto.previous_block_hash,
            dto.block_transactions_hash,
            block_receipts_hash,
            state_hash,
            beneficiary_public_account,
            fee_interest,
            Uint64Dto(total_fee).compact(),
            fee_interest_denominator,
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct BlockInfoDto {
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
