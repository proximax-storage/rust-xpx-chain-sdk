use ::std::fmt::Debug;
use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::{
    apis::sirius_client::ApiClient,
    models::blockchain::{
        BlockchainScore,
        BlockchainScoreDto,
        HeightInfo,
        HeightInfoDto,
        StorageInfo,
    },
};

use super::{request as __internal_request, Result};

/// Chain ApiClient routes.
///
#[derive(Clone)]
pub struct ChainRoutes<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> ChainRoutes<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        ChainRoutes {
            client,
        }
    }
}

/// Chain related endpoints.
///
impl<C: Connect> ChainRoutes<C> where
    C: Clone + Send + Sync + 'static
{
    /// Get the current height of the chain
    ///
    /// # Example
    ///
    /// ```
    ///use hyper::Client;
    ///use xpx_chain_sdk::apis::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
    ///
    ///    let chain_height = client.chain.get_blockchain_height().await;
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
    /// Returns a Future `Result` whose okay value is the current height of the blockchain or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_blockchain_height(self) -> Result<HeightInfo> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/chain/height".to_string(),
        );
        let dto: Result<HeightInfoDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
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
    ///use hyper::Client;
    ///use xpx_chain_sdk::apis::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
    ///
    ///    let chain_score = client.chain.get_blockchain_score().await;
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
    /// Returns a Future `Result` whose okay value is [BlockchainScore] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_blockchain_score(self) -> Result<BlockchainScore> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/chain/score".to_string(),
        );

        let dto: Result<BlockchainScoreDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }

    /// Get the storage information of the node.
    ///
    ///
    /// # Example
    ///
    /// ```
    ///use hyper::Client;
    ///use xpx_chain_sdk::apis::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
    ///
    ///    let chain_score = client.chain.get_blockchain_storage().await;
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
    /// Returns a Future `Result` whose okay value is [StorageInfo] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_blockchain_storage(self) -> Result<StorageInfo> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/diagnostic/storage".to_string(),
        );

        req.execute(self.client).await
    }
}
