use std::fmt::Debug;
use std::sync::Arc;

use hyper::client::connect::Connect;

use crate::apis::sirius_client::ApiClient;
use crate::models::blockchain::{BlockInfo,BlockInfoDto};
use crate::models::transaction::TransactionInfoDto;

use super::request as __internal_request;

#[derive(Debug, Clone)]
pub struct BlockRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> BlockRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> BlockRoutesApiClient<C> {
        let clone = client.clone();

        BlockRoutesApiClient {
            client: clone,
        }
    }
}

impl<C: Connect> BlockRoutesApiClient<C>
    where
        C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_block_by_height(self, height: u64) -> super::Result<BlockInfo> {
        assert_ne!(height, 0, "Block height should not be zero.");

        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/block/{height}".to_string(),
        );

        req = req.with_path_param("height".to_string(), height.to_string());

        let dto: super::Result<BlockInfoDto> = req.execute(self.client).await;

        Ok(dto.unwrap().to_struct()?)
    }

    pub async fn get_blocks_by_height_with_limit(
        self, height: u64, mut limit: i32) -> super::Result<Vec<BlockInfo>> {
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
            hyper::Method::GET,
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
        self, height: u64, page_size: Option<i32>, id: Option<&str>) -> super::Result<()> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/block/{height}/transactions".to_string(),
        );

        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        req = req.with_path_param("height".to_string(), height.to_string());

        let _dto: super::Result<Vec<TransactionInfoDto>> = req.execute(self.client).await;

        unimplemented!()
    }
}

