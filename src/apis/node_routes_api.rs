use std::fmt::Debug;
use std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::{
    apis::sirius_client::ApiClient,
    models::node::{
        NodeInfo,
        NodeTime,
        NodeTimeDto,
    },
};

use super::{request as __internal_request, Result};

#[derive(Clone)]
pub struct NodeRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> NodeRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        NodeRoutesApiClient {
            client,
        }
    }
}

impl<C: Connect> NodeRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_node_info(self) -> Result<NodeInfo> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/node/info".to_string());

        req.execute(self.client).await
    }

    pub async fn get_node_time(self) -> Result<NodeTime> {
        let req = __internal_request::Request::new(
            Method::GET,
            "/node/time".to_string());

        let dto: Result<NodeTimeDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }
}
