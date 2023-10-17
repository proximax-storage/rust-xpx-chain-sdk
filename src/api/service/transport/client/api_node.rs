/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::{
	convert::{TryFrom, TryInto},
	fmt,
	future::Future,
	pin::Pin,
	str::FromStr,
	time::Duration,
};

use bytes::Bytes;
use hyper::header::HeaderValue;
use hyper::Uri;

use crate::api::{
	self,
	transport::{Error, service},
};

use super::Client;

/// Channel builder.
///
/// This struct is used to build and configure HTTP clients.
#[derive(Clone, Builder)]
#[builder(create_empty = "empty", build_fn(error = "crate::api::error::Error"))]
pub struct ApiNode {
    #[builder(setter(custom), default = "Uri::from_static(\"http:://127.0.0.1:3000\")")]
    pub(crate) uri: Uri,
    #[builder(setter(strip_option), default)]
    pub(crate) user_agent: Option<HeaderValue>,
    /// Apply a timeout to each request.
    ///
    /// # Notes
    ///
    /// This does **not** set the timeout metadata (`http-timeout` header) on
    /// the request, meaning the server will not be informed of this timeout,
    /// for that use [`Request::set_timeout`].
    ///
    /// [`Request::set_timeout`]: crate::Request::set_timeout
    #[builder(setter(strip_option), default)]
    pub(crate) timeout: Option<Duration>,
    /// Apply a concurrency limit to each request.
    #[builder(setter(strip_option), default)]
    pub(crate) concurrency_limit: Option<usize>,
    /// Apply a rate limit to each request.
    #[builder(setter(strip_option), default)]
    pub(crate) rate_limit: Option<(u64, Duration)>,
    /// Set whether TCP keepalive messages are enabled on accepted connections.
    ///
    /// If `None` is specified, keepalive is disabled, otherwise the duration
    /// specified will be the time to remain idle before sending TCP keepalive
    /// probes.
    ///
    /// Default is no keepalive (`None`)
    #[builder(setter(strip_option), default)]
    pub(crate) tcp_keep_alive: Option<Duration>,
    /// Set the value of `TCP_NODELAY` option for accepted connections. Enabled by default.
    #[builder(default = "true")]
    pub(crate) tcp_nodelay: bool,
    /// Apply a timeout to connecting to the uri.
    ///
    /// Defaults to no timeout.
    #[builder(setter(strip_option), default)]
    connect_timeout: Option<Duration>,
    #[builder(setter(custom), default = "service::SharedExec::tokio()")]
    pub(crate) executor: service::SharedExec,
    /// Determines the maximum number of retries allowed and is used to control the retry logic
    /// based on the remaining retry attempts.
    ///
    /// Defaults 3 retries.
    #[builder(setter(strip_option), default = "Some(3)")]
    pub(crate) max_num_retry: Option<usize>,
    #[builder(setter(strip_option), default = "true")]
    pub(crate) retry_service_unavailable_requests: bool,
}

impl ApiNodeBuilder {
    /// The uri method sets the uri field.
    pub fn uri<R: Into<String>>(&mut self, value: R) -> &mut Self {
        let dst = Uri::from_maybe_shared(value.into()).unwrap_or_default();
        self.uri = Some(dst);
        self
    }

    /// Sets the executor used to spawn async tasks.
    ///
    /// Uses `tokio::spawn` by default.
    pub fn executor<E>(mut self, executor: E) -> Self
        where
            E: hyper::rt::Executor<Pin<Box<dyn Future<Output=()> + Send>>> + Send + Sync + 'static,
    {
        self.executor = Some(service::SharedExec::new(executor));
        self
    }
}

impl ApiNode {
    /// Build a ApiNode object.
    pub fn builder() -> ApiNodeBuilder {
        ApiNodeBuilder::default()
    }

    // FIXME: determine if we want to expose this or not. This is really
    // just used in codegen for a shortcut.
    #[doc(hidden)]
    pub fn new<D>(dst: D) -> Result<Self, Error>
        where
            D: TryInto<Self>,
            D::Error: Into<api::Error>,
    {
        let me = dst.try_into().map_err(|e| Error::from_source(e.into()))?;
        Ok(me)
    }

    /// Convert an `ApiNode` from a static string.
    ///
    /// # Panics
    ///
    /// This function panics if the argument is an invalid URI.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// ```
    pub fn from_static(s: &'static str) -> Self {
        let dst = Uri::from_static(s);
        Self::from(dst)
    }

    /// Convert an `ApiNode` from shared bytes.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// let _ = ApiNode::from_shared("http://api-1.testnet2.xpxsirius.io:3000".to_string());
    /// ```
    pub fn from_shared(s: impl Into<Bytes>) -> Result<Self, Error> {
        let uri = Uri::from_maybe_shared(s.into()).map_err(|e| Error::new_invalid_uri().with(e))?;
        Ok(Self::from(uri))
    }

    /// Create a channel from this config.
    pub async fn connect(&self) -> Result<Client, Error> {
        let connector = self.__build_connector();
        self.__build_client(connector).await
    }

    /// Create a channel from this config.
    ///
    /// The channel returned by this method does not attempt to connect to the ApiNode until first
    /// use.
    pub fn connect_lazy(&self) -> Client {
        let connector = self.__build_connector();

        let mut connector = hyper_timeout::TimeoutConnector::new(connector);
        connector.set_connect_timeout(self.connect_timeout);
        Client::new(connector, self.clone())
    }

    fn __build_connector(&self) -> service::Connector {
        let mut http = hyper::client::HttpConnector::new();
        http.enforce_http(false);
        http.set_nodelay(self.tcp_nodelay);
        http.set_keepalive(self.tcp_keep_alive);

        let connector = service::connector(http);
        connector
    }

    async fn __build_client(&self, connector: service::Connector) -> Result<Client, Error> {
        let mut connector = hyper_timeout::TimeoutConnector::new(connector);
        connector.set_connect_timeout(self.connect_timeout);
        Client::connect(connector, self.clone()).await
    }

    /// Get the ApiNode uri.
    ///
    /// ```
    /// # use hyper::Uri;
    /// use xpx_chain_sdk::api::ApiNode;
    /// let node = ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    ///
    /// assert_eq!(node.uri(), &Uri::from_static("http://api-1.testnet2.xpxsirius.io:3000"));
    /// ```
    pub fn uri(&self) -> &Uri {
        &self.uri
    }
}

impl From<Uri> for ApiNode {
    fn from(uri: Uri) -> Self {
        Self {
            uri,
            user_agent: Some(HeaderValue::from_static("RustSdk")),
            concurrency_limit: None,
            rate_limit: None,
            timeout: None,
            tcp_keep_alive: None,
            tcp_nodelay: true,
            connect_timeout: None,
            executor: service::SharedExec::tokio(),
            max_num_retry: Some(3),
            retry_service_unavailable_requests: true,
        }
    }
}

impl TryFrom<Bytes> for ApiNode {
    type Error = Error;

    fn try_from(t: Bytes) -> Result<Self, Self::Error> {
        Self::from_shared(t)
    }
}

impl TryFrom<String> for ApiNode {
    type Error = Error;

    fn try_from(t: String) -> Result<Self, Self::Error> {
        Self::from_shared(t.into_bytes())
    }
}

impl TryFrom<&'static str> for ApiNode {
    type Error = Error;

    fn try_from(t: &'static str) -> Result<Self, Self::Error> {
        Self::from_shared(t.as_bytes())
    }
}

impl fmt::Debug for ApiNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiNode").finish()
    }
}

impl FromStr for ApiNode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}
