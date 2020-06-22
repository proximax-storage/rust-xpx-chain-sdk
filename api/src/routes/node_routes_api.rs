// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use ::std::sync::Arc;

use reqwest::Method;

use sdk::node::{NodeInfo, NodeTime};

use crate::{dtos::NodeTimeDto, request as __internal_request, sirius_client::ApiClient, Result};

use super::{NODE_INFO, NODE_TIME};

/// Node ApiClient routes.
///
#[derive(Clone)]
pub struct NodeRoutes(Arc<ApiClient>);

/// Node related endpoints.
///
impl NodeRoutes {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        NodeRoutes(client)
    }

    fn __client(self) -> Arc<ApiClient> {
        self.0.to_owned()
    }

    /// Get the node information.
    /// Supplies additional information about the application running on a node.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_api::SiriusClient;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let node_info = client.node.get_node_info().await;
    ///
    ///    match node_info {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [NodeInfo] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_node_info(self) -> Result<NodeInfo> {
        let req = __internal_request::Request::new(Method::GET, NODE_INFO.to_string());

        req.execute(self.__client()).await
    }

    /// Get the node time.
    /// Gets the node time at the moment the reply was sent and received.
    ///
    /// # Example
    ///
    /// ```
    ///use xpx_chain_api::SiriusClient;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    ///
    ///    let node_time = client.node.get_node_time().await;
    ///
    ///    match node_time {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [NodeTime] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_node_time(self) -> Result<NodeTime> {
        let req = __internal_request::Request::new(Method::GET, NODE_TIME.to_string());

        let dto: Result<NodeTimeDto> = req.execute(self.__client()).await;

        Ok(dto?.compact())
    }
}
