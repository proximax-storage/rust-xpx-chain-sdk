/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::sync::Arc, reqwest::Method};

use crate::api::routes::const_routes::NETWORK_INFO;
use crate::network::NetworkInfo;
use crate::{
    api::{error::Result, request as __internal_request, sirius_client::ApiClient},
    node::{NodeInfo, NodeTime},
};

/// Network ApiClient routes.
///
#[derive(Clone)]
pub struct NetworkRoutes(Arc<ApiClient>);

/// Network related endpoints.
///
impl NetworkRoutes {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        NetworkRoutes(client)
    }

    fn __client(&self) -> Arc<ApiClient> {
        Arc::clone(&self.0)
    }

    /// Get the current network type of the chain.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
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
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_network_type(&self) -> Result<NetworkInfo> {
        let req = __internal_request::Request::new(Method::GET, NETWORK_INFO.to_string());

        req.execute(self.__client()).await
    }
}
