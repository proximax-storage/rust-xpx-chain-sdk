/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use hyper::Method;

use crate::{
    api::{BlockchainScoreDto, error::Result, HeightInfoDto, request as __internal_request},
    blockchain::{BlockchainScore, HeightInfo, StorageInfo},
};
use crate::api::BlockInfoDto;
use crate::api::routes::const_routes::{BLOCK_BY_HEIGHT_ROUTE, BLOCK_INFO_ROUTE};
use crate::api::transport::service::Connection;
use crate::blockchain::BlockInfo;

use super::{CHAIN_HEIGHT_ROUTE, CHAIN_SCORE_ROUTE, CHAIN_STORAGE_ROUTE};

/// Chain ApiClient routes.
///
#[derive(Clone)]
pub struct ChainRoutes(Connection);

/// Chain related endpoints.
///
impl ChainRoutes {
    pub(crate) fn new(client: Connection) -> Self {
        ChainRoutes(client)
    }

    fn __client(&self) -> Connection {
        self.0.clone()
    }

    /// Get the current `Height` of the chain
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::ApiNode;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    ///    let chain_height = client.chain_api().get_block_height().await;
    ///
    ///    match chain_height {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is the current `Height` of the blockchain or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_block_height(&self) -> Result<HeightInfo> {
        let req = __internal_request::ApiRequest::new(Method::GET, CHAIN_HEIGHT_ROUTE.to_string());
        let dto: Result<HeightInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact())
    }

    /// Get the current score of the chain.
    ///
    /// Gets the current score of the blockchain. The higher the score, the better the chain.
    /// During synchronization, nodes try to get the best blockchain in the network.
    /// The score for a block is derived from its difficulty and the time (in seconds)
    /// that has elapsed since the last block:
    /// * block score = difficulty − time elapsed since last block
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::ApiNode;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    /// let chain_score = client.chain_api().get_block_score().await;
    ///
    ///    match chain_score {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is `BlockchainScore` or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_block_score(&self) -> Result<BlockchainScore> {
        let req = __internal_request::ApiRequest::new(Method::GET, CHAIN_SCORE_ROUTE.to_string());

        let dto: Result<BlockchainScoreDto> = req.execute(self.__client()).await;

        Ok(dto?.compact())
    }

    /// Get the storage information of the node.
    ///
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::ApiNode;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    /// let chain_score = client.chain_api().get_block_storage().await;
    ///
    ///    match chain_score {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is `StorageInfo` or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_block_storage(&self) -> Result<StorageInfo> {
        let req = __internal_request::ApiRequest::new(Method::GET, CHAIN_STORAGE_ROUTE.to_string());

        req.execute(self.__client()).await
    }

    /// Get `BlockInfo` information
    ///
    /// Gets a block from the chain that has the given height.
    ///
    /// # Inputs
    ///
    /// * `height` =    The height of the block.
    ///
    /// # Example
    /// ```
    ///
    ///use xpx_chain_sdk::api::ApiNode;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    /// let block_by_height = client.chain_api().get_block_by_height(1).await;
    ///
    ///    match block_by_height {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `BlockInfo` the block information or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_block_by_height(&self, height: u64) -> Result<BlockInfo> {
        assert_ne!(height, 0, "Block height should not be zero.");

        let mut req =
            __internal_request::ApiRequest::new(Method::GET, BLOCK_BY_HEIGHT_ROUTE.to_string());

        req = req.with_path_param("height".to_string(), height.to_string());

        let dto: Result<BlockInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact()?)
    }

    ///
    /// Get `blocks` information
    ///
    /// Gets up to limit number of blocks after given block height.
    ///
    /// # Inputs
    ///
    /// * `height` =    The height of the block.
    /// If height -1 is not a multiple of the limit provided,
    /// the inferior closest multiple + 1 is used instead.
    ///
    /// * `limit` =    The number of blocks to be returned.
    /// Allowed limit: 25 50 75 100.
    ///
    ///
    /// # Example
    /// ```
    ///
    ///use xpx_chain_sdk::api::ApiNode;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    /// let blocks_by_height_with_limit = client.chain_api().get_blocks_by_height_with_limit(1, 25).await;
    ///
    ///    match blocks_by_height_with_limit {
    ///        Ok(resp_info) => println!("{:#?}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an Vector of `BlockInfo` or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_blocks_by_height_with_limit(
        &self,
        height: u64,
        limit: i32,
    ) -> Result<Vec<BlockInfo>> {
        assert_ne!(height, 0, "Block height should not be zero.");

        assert_ne!(limit, 0, "Limit should not be zero.");

        let limit = if limit < 25 {
            25
        } else if limit < 50 {
            50
        } else {
            100
        };

        let mut req =
            __internal_request::ApiRequest::new(Method::GET, BLOCK_INFO_ROUTE.to_string());

        req = req.with_path_param("height".to_string(), height.to_string());

        req = req.with_path_param("limit".to_string(), limit.to_string());

        let dto: Vec<BlockInfoDto> = req.execute(self.__client()).await?;

        let mut blocks_info: Vec<BlockInfo> = vec![];
        for block_inf in dto.into_iter() {
            blocks_info.push(block_inf.compact()?);
        }

        Ok(blocks_info)
    }

    /// Get receipts from a block.
    ///
    pub async fn get_block_receipts(&self) {
        todo!()
    }

    /// Get the merkle path for a given a transaction and block.
    ///
    pub async fn get_merkle_transaction(&self) {
        todo!()
    }
}
