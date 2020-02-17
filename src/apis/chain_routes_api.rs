use std::fmt::Debug;
use std::sync::Arc;

use hyper::client::connect::Connect;

use crate::apis::sirius_client::ApiClient;
use crate::models::blockchain::{
    BlockchainScore,
    BlockchainScoreDto,
    HeightInfo,
    HeightInfoDto,
    StorageInfo,
};

use super::request as __internal_request;

#[derive(Debug, Clone)]
pub struct ChainRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> ChainRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> ChainRoutesApiClient<C> {
        let clone = client.clone();

        ChainRoutesApiClient {
            client: clone,
        }
    }
}

impl<C: Connect> ChainRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_blockchain_height(self) -> super::Result<HeightInfo> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/chain/height".to_string(),
        );
        let dto: super::Result<HeightInfoDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }

    pub async fn get_blockchain_score(self) -> super::Result<BlockchainScore> {
        let req = __internal_request::Request::new(
            hyper::Method::GET,
            "/chain/score".to_string(),
        );

        let dto: super::Result<BlockchainScoreDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }

    pub async fn get_blockchain_storage(self) -> super::Result<StorageInfo> {
        let req = __internal_request::Request::new(
            hyper::Method::GET,
            "/diagnostic/storage".to_string(),
        );

        req.execute(self.client).await
    }
}
