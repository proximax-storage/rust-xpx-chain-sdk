use std::fmt::Debug;
use std::sync::Arc;

use hyper::client::connect::Connect;

use crate::apis::sirius_client::ApiClient;

use super::request as __internal_request;
use crate::models::node::{NodeInfo, NodeTime, NodeTimeDto};

#[derive(Debug, Clone)]
pub struct NodeRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> NodeRoutesApiClient<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> NodeRoutesApiClient<C> {
        let clone = client.clone();

        NodeRoutesApiClient {
            client: clone,
        }
    }
}

impl<C: Connect> NodeRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_node_info(self) -> super::Result<NodeInfo> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/node/info".to_string());

        req.execute(self.client).await
    }

    pub async fn get_node_time(self) -> super::Result<NodeTime> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/node/time".to_string());

        let dto: super::Result<NodeTimeDto> = req.execute(self.client).await;

        Ok(dto.unwrap().to_struct())
    }
}
