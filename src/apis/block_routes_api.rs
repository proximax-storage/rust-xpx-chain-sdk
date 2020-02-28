use ::std::fmt::Debug;
use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::models::{
    blockchain::{BlockInfo, BlockInfoDto},
    transaction::TransactionInfoDto,
};

use super::{
    request as __internal_request,
    Result,
    sirius_client::ApiClient,
};

#[derive(Clone)]
pub struct BlockRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> BlockRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        BlockRoutesApiClient {
            client,
        }
    }
}

impl<C: Connect> BlockRoutesApiClient<C>
    where
        C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_block_by_height(self, height: u64) -> Result<BlockInfo> {
        assert_ne!(height, 0, "Block height should not be zero.");

        let mut req = __internal_request::Request::new(
            Method::GET,
            "/block/{height}".to_string(),
        );

        req = req.with_path_param("height".to_string(), height.to_string());

        let dto: Result<BlockInfoDto> = req.execute(self.client).await;

        Ok(dto?.to_struct()?)
    }

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
            "/blocks/{height}/limit/{limit}".to_string(),
        );

        req = req.with_path_param("height".to_string(), height.to_string());

        req = req.with_path_param("limit".to_string(), limit.to_string());

        let dto: Vec<BlockInfoDto> = req.execute(self.client).await?;

        let mut blocks_info: Vec<BlockInfo> = Vec::with_capacity(dto.len());
        for i in dto {
            let block_info = i;
            blocks_info.push(block_info.to_struct()?);
        }

        Ok(blocks_info)
    }

    pub async fn get_block_transactions(
        self, height: u64, page_size: Option<i32>, id: Option<&str>) -> Result<()> {
        let mut req = __internal_request::Request::new(
            Method::GET,
            "/block/{height}/transactions".to_string(),
        );

        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        req = req.with_path_param("height".to_string(), height.to_string());

        let _dto: Result<Vec<TransactionInfoDto>> = req.execute(self.client).await;

        unimplemented!()
    }
}

