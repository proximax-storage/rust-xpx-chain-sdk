/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use hyper::Method;

use crate::api::{error::Result, request as __internal_request};
use crate::api::routes::const_routes::NETWORK_INFO;
use crate::api::transport::service::Connection;
use crate::network::NetworkInfo;

/// Network ApiClient routes.
///
#[derive(Clone)]
pub struct NetworkRoutes(Connection);

/// Network related endpoints.
///
impl NetworkRoutes {
    pub(crate) fn new(client: Connection) -> Self {
        NetworkRoutes(client)
    }

    fn __connection(&self) -> Connection {
        self.0.clone()
    }

    /// Get the current network type of the chain.
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
    /// let network_type = client.network_api().get_network_type().await;
    ///
    /// match network_type {
    ///     Ok(resp_info) => println!("{}", resp_info),
    ///     Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `NetworkInfo` or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_network_type(&self) -> Result<NetworkInfo> {
        let req = __internal_request::ApiRequest::new(Method::GET, NETWORK_INFO.to_string());
        // let connection = self.__connection();
        req.execute(self.__connection()).await
    }
}
