/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use hyper::Method;

use crate::{
    api::{error::Result, NodeTimeDto, request as __internal_request},
    node::{NodeInfo, NodeTime},
};
use crate::api::NodeInfoDto;
use crate::api::transport::service::Connection;

use super::{NODE_INFO, NODE_PEERS, NODE_TIME};

/// Node ApiClient routes.
///
#[derive(Clone)]
pub struct NodeRoutes(Connection);

/// Node related endpoints.
///
impl NodeRoutes {
    pub(crate) fn new(client: Connection) -> Self {
        NodeRoutes(client)
    }

    fn __client(&self) -> Connection {
        self.0.clone()
    }

    /// Get the node information.
    /// Supplies additional information about the application running on a node.
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
    /// let node_info = client.node_api().get_node_info().await;
    ///
    /// match node_info {
    ///     Ok(resp_info) => println!("{}", resp_info),
    ///     Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [NodeInfo] or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        let req = __internal_request::ApiRequest::new(Method::GET, NODE_INFO.to_string());

        let resp: NodeInfoDto = req.execute(self.__client()).await?;

        Ok(resp.compact())
    }

    /// Get the node time.
    /// Gets the node time at the moment the reply was sent and received.
    ///
    /// # Example
    ///
    /// ```
    ///use xpx_chain_sdk::api::ApiNode;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    /// let node_time = client.node_api().get_node_time().await;
    /// match node_time {
    ///     Ok(resp_info) => println!("{}", resp_info),
    ///     Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [NodeTime] or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_node_time(&self) -> Result<NodeTime> {
        let req = __internal_request::ApiRequest::new(Method::GET, NODE_TIME.to_string());

        let dto: Result<NodeTimeDto> = req.execute(self.__client()).await;

        Ok(dto?.compact())
    }

    /// Get node peers.
    ///
    /// # Example
    ///
    /// ```
    ///use xpx_chain_sdk::api::ApiNode;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    /// let node_peers = client.node_api().get_node_peers().await;
    /// match node_peers {
    ///     Ok(resp_info) => println!("{:?}", resp_info),
    ///     Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an vector of `NodeInfo` or whose error
    /// value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_node_peers(&self) -> Result<Vec<NodeInfo>> {
        let req = __internal_request::ApiRequest::new(Method::GET, NODE_PEERS.to_string());

        let resp: Vec<NodeInfoDto> = req.execute(self.__client()).await?;

        Ok(resp.iter().map(NodeInfoDto::compact).collect::<Vec<_>>())
    }
}
