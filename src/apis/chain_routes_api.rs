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

#[derive(Clone)]
pub struct ChainRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> ChainRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        ChainRoutesApiClient {
            client,
        }
    }
}

impl<C: Connect> ChainRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_blockchain_height(self) -> Result<HeightInfo> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/chain/height".to_string(),
        );
        let dto: Result<HeightInfoDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }

    pub async fn get_blockchain_score(self) -> Result<BlockchainScore> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/chain/score".to_string(),
        );

        let dto: Result<BlockchainScoreDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }

    pub async fn get_blockchain_storage(self) -> Result<StorageInfo> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/diagnostic/storage".to_string(),
        );

        req.execute(self.client).await
    }
}
