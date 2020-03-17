use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::models::{
    blockchain::{BlockInfo, BlockInfoDto},
    transaction::{Transaction, TransactionDto, Transactions},
};

use super::{
    request as __internal_request,
    Result,
    sirius_client::ApiClient,
};

const BLOCK_BY_HEIGHT_ROUTE: &str = "/block/{height}";
const BLOCK_GET_TRANSACTION_ROUTE: &str = "/block/{height}/transactions";
const BLOCK_INFO_ROUTE: &str = "/blocks/{height}/limit/{limit}";

/// Block ApiClient routes.
///
#[derive(Clone)]
pub struct BlockRoutes<C: Connect> (Arc<ApiClient<C>>);

///  Block related endpoints.
///
impl<C: Connect> BlockRoutes<C>
    where
        C: Clone + Send + Sync + 'static
{
    pub(crate) fn new(client: Arc<ApiClient<C>>) -> Self {
        BlockRoutes(client)
    }

    /// Get [BlockInfo] information
    ///
    /// Gets a block from the chain that has the given height.
    ///
    /// # Inputs
    ///
    /// * `height` =    The height of the block.
    ///
    /// # Example
    /// ```
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
    ///
    ///    let block_by_height = client.block.get_block_by_height(1).await;
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
    /// Returns a Future `Result` whose okay value is an [BlockInfo] the block information or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_block_by_height(self, height: u64) -> Result<BlockInfo> {
        assert_ne!(height, 0, "Block height should not be zero.");

        let mut req = __internal_request::Request::new(
            Method::GET,
            BLOCK_BY_HEIGHT_ROUTE.to_string(),
        );

        req = req.with_path_param("height".to_string(), height.to_string());

        let dto: Result<BlockInfoDto> = req.execute(self.0).await;

        Ok(dto?.to_struct()?)
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
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
    ///
    ///    let blocks_by_height_with_limit = client.block.get_blocks_by_height_with_limit(1, 25).await;
    ///
    ///    match blocks_by_height_with_limit {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an Vector of [BlockInfo] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_blocks_by_height_with_limit(
        self, height: u64, mut limit: i32) -> Result<Vec<BlockInfo>> {
        assert_ne!(height, 0, "Block height should not be zero.");

        assert_ne!(limit, 0, "Limit should not be zero.");

        if limit < 25 {
            limit = 25;
        } else if limit < 50 {
            limit = 50;
        } else {
            limit = 100;
        }

        let mut req = __internal_request::Request::new(
            Method::GET,
            BLOCK_INFO_ROUTE.to_string(),
        );

        req = req.with_path_param("height".to_string(), height.to_string());

        req = req.with_path_param("limit".to_string(), limit.to_string());

        let dto: Vec<BlockInfoDto> = req.execute(self.0).await?;

        let mut blocks_info: Vec<BlockInfo> = Vec::with_capacity(dto.len());
        for block_info_dto in dto {
            blocks_info.push(block_info_dto.to_struct()?);
        }

        Ok(blocks_info)
    }

    ///
    /// Get `Transactions` from a block
    ///
    /// Gets a block from the chain that has the given height.
    ///
    /// # Inputs
    ///
    /// * `height`  =   The height of the block.
    ///
    /// * `pageSize` =  The number of transactions to return for each request.
    ///   `Default`: 10.
    ///
    /// * `id`  =   The transaction id up to which transactions are returned.
    ///   `Default`: "".
    ///
    /// # Example
    /// ```
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
    ///
    ///    let block_transactions = client.block.get_block_transactions(1, None, None).await;
    ///
    ///    match block_transactions {
    ///        Ok(tx) => {
    ///            for i in tx {
    ///                println!("{}", i)
    ///            }
    ///        },
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an Vector of `Transactions` included in a block
    /// for a given block height or whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_block_transactions(
        self, height: u64, page_size: Option<i32>, id: Option<&str>) -> Result<Transactions> {
        let mut req = __internal_request::Request::new(
            Method::GET,
            BLOCK_GET_TRANSACTION_ROUTE.to_string(),
        );

        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        req = req.with_path_param("height".to_string(), height.to_string());

        let dto: Vec<Box<dyn TransactionDto>> = req.execute(self.0).await?;

        let mut transactions_info: Vec<Box<dyn Transaction>> = Vec::with_capacity(dto.len());
        for transaction_dto in dto {
            transactions_info.push(transaction_dto.to_struct()?);
        }

        Ok(transactions_info)
    }
}

