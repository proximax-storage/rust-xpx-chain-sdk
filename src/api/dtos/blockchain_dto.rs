/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::blockchain::{BlockchainScore, HeightInfo};

use super::{AbstractTransactionDto, Uint64Dto, UpgradeDto};

#[derive(Serialize, Deserialize)]
pub(crate) struct HeightInfoDto {
    #[serde(rename = "height")]
    height: Uint64Dto,
}

impl HeightInfoDto {
    pub fn compact(&self) -> HeightInfo {
        HeightInfo {
            height: self.height.compact(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BlockchainScoreDto {
    #[serde(rename = "scoreHigh")]
    score_high: Uint64Dto,
    #[serde(rename = "scoreLow")]
    score_low: Uint64Dto,
}

impl BlockchainScoreDto {
    pub fn compact(&self) -> BlockchainScore {
        BlockchainScore {
            score_high: self.score_high.compact(),
            score_low: self.score_low.compact(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BlockchainUpgradeDto {
    #[serde(rename = "blockchainUpgrade")]
    blockchain_upgrade: UpgradeDto,
}

/// BlockchainUpgradeTransactionDto : Transaction that change version of blockchain.
#[derive(Serialize, Deserialize)]
pub(crate) struct BlockchainUpgradeTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    #[serde(rename = "upgradePeriod")]
    upgrade_period: Uint64Dto,
    #[serde(rename = "newBlockChainVersion")]
    new_block_chain_version: Uint64Dto,
}
