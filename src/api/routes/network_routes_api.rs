/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use http::Request;
use tower::buffer::Buffer;

use reqwest::Method;

use crate::api::body::BoxBody;
use crate::api::routes::const_routes::NETWORK_INFO;
use crate::api::Client;
use crate::api::{error::Result, request as __internal_request};
use crate::network::NetworkInfo;

/// Network ApiClient routes.
///
#[derive(Clone)]
pub struct NetworkRoutes(Client);

/// Network related endpoints.
///
impl NetworkRoutes {
    pub(crate) fn new(client: Client) -> Self {
        NetworkRoutes(client)
    }

    fn __client(&self) -> Client {
        self.0.clone()
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
        let req = __internal_request::ApiRequest::new(Method::GET, NETWORK_INFO.to_string());

        req.execute(self.__client()).await
    }
}
