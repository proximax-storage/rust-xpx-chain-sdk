/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

//! Client implementation and builder.

use std::{
    fmt,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::future;
use hyper::http::uri::InvalidUri;
use hyper::Uri;
use tower::Service;

pub use api_node::ApiNode;

use crate::api;
use crate::api::routes::{
    account_routes_api::AccountRoutes, chain_routes_api::ChainRoutes,
    metadata_v2_routes_api::MetadataV2Routes, mosaic_routes_api::MosaicRoutes,
    namespace_routes_api::NamespaceRoutes, network_routes_api::NetworkRoutes,
    node_routes_api::NodeRoutes, resolver_routes_api::ResolverRoutes,
    transaction_routes_api::TransactionRoutes,
};
use crate::api::transport::service::connection::{Request, Response, TimeoutConnector};

use super::service::Connection;

mod api_node;

const DEFAULT_BUFFER_SIZE: usize = 1024;

/// A default batteries included `transport` client.
///
/// This provides a fully featured http client based on [`hyper::Client`]
/// and `tower` services.
///
/// # Multiplexing requests
///
/// Sending a request on a client requires a `&mut self` and thus can only send
/// one request in flight. This is intentional and is required to follow the `Service`
/// contract from the `tower` library which this client implementation is built on
/// top of.
///
/// `tower` itself has a concept of `poll_ready` which is the main mechanism to apply
/// back pressure. `poll_ready` takes a `&mut self` and when it returns `Poll::Ready`
/// we know the `Service` is able to accept only one request before we must `poll_ready`
/// again. Due to this fact any `async fn` that wants to poll for readiness and submit
/// the request must have a `&mut self` reference.
///
/// To work around this and to ease the use of the client, `Channel` provides a
/// `Clone` implementation that is _cheap_. This is because at the very top level
/// the client is backed by a `tower_buffer::Buffer` which runs the connection
/// in a background task and provides a `mpsc` client interface. Due to this
/// cloning the `Channel` type is cheap and encouraged.
#[derive(Clone)]
pub struct Client {
    inner: Connection,
}

impl Client {
    /// Create an [`ApiNode`] builder that can create [`Channel`]s.
    pub fn builder(uri: Uri) -> ApiNode {
        ApiNode::from(uri)
    }

    /// Create an `Endpoint` from a static string.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Client;
    /// Client::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// ```
    pub fn from_static(s: &'static str) -> ApiNode {
        let uri = Uri::from_static(s);
        Self::builder(uri)
    }

    /// Create an `Endpoint` from shared bytes.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Client;
    /// Client::from_shared("http://api-1.testnet2.xpxsirius.io:3000");
    /// ```
    pub fn from_shared(s: impl Into<Bytes>) -> Result<ApiNode, InvalidUri> {
        let uri = Uri::from_maybe_shared(s.into())?;
        Ok(Self::builder(uri))
    }

    pub(crate) fn new(connector: TimeoutConnector, endpoint: ApiNode) -> Self {
        let svc = Connection::lazy(connector, endpoint);

        Client { inner: svc }
    }

    pub(crate) async fn connect(
        connector: TimeoutConnector,
        endpoint: ApiNode,
    ) -> Result<Self, super::Error> {
        let svc = Connection::connect(connector, endpoint)
            .await
            .map_err(super::Error::from_source)?;

        Ok(Client { inner: svc })
    }
}

impl Client {
    fn __client(&self) -> Connection {
        self.inner.clone()
    }

    /// Get Account ApiClient routes.
    pub fn account_api(&self) -> Box<AccountRoutes> {
        Box::new(AccountRoutes::new(self.__client()))
    }

    /// Get Chain ApiClient routes.
    pub fn chain_api(&self) -> Box<ChainRoutes> {
        Box::new(ChainRoutes::new(self.__client()))
    }

    // /// Get Exchange ApiClient routes.
    // pub fn exchange_api(&self) -> Box<ExchangeRoutes> {
    //     Box::new(ExchangeRoutes::new(
    //         self.__client(),
    //         self.network_type(),
    //         *self.resolver_api(),
    //     ))
    // }

    /// Get Node ApiClient routes.
    pub fn node_api(&self) -> Box<NodeRoutes> {
        Box::new(NodeRoutes::new(self.__client()))
    }

    /// Get Network ApiClient routes.
    pub fn network_api(&self) -> Box<NetworkRoutes> {
        Box::new(NetworkRoutes::new(self.__client()))
    }

    /// Get Mosaic ApiClient routes.
    pub fn mosaic_api(&self) -> Box<MosaicRoutes> {
        Box::new(MosaicRoutes::new(self.__client()))
    }

    /// Get Namespace ApiClient routes.
    pub fn namespace_api(&self) -> Box<NamespaceRoutes> {
        Box::new(NamespaceRoutes::new(self.__client()))
    }

    /// Get Transaction ApiClient routes.
    pub fn transaction_api(&self) -> Box<TransactionRoutes> {
        Box::new(TransactionRoutes::new(self.__client()))
    }

    /// Get Resolver ApiClient routes.
    pub fn resolver_api(&self) -> Box<ResolverRoutes> {
        Box::new(ResolverRoutes::new(self.__client(), *self.namespace_api(), *self.mosaic_api()))
    }

    // /// Get Metadata ApiClient routes.
    // pub fn metadata_api(&self) -> Box<MetadataRoutes> {
    //     Box::new(MetadataRoutes::new(self.__client()))
    // }

    /// Get MetadataV2 ApiClient routes.
    pub fn metadata_v2_api(&self) -> Box<MetadataV2Routes> {
        Box::new(MetadataV2Routes::new(self.__client()))
    }
}

impl Service<Request> for Client {
    type Response = Response;
    type Error = api::error::Error;
    type Future = future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let fut = self.inner.call(request);
        Box::pin(async move { fut.await.map_err(Into::into) })
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Channel").finish()
    }
}
