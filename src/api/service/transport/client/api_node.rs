use std::{
    convert::{TryFrom, TryInto},
    fmt,
    future::Future,
    pin::Pin,
    str::FromStr,
    time::Duration,
};

use bytes::Bytes;
use http::{HeaderValue, uri::Uri};
use hyper::rt::Executor;
use tower::make::MakeConnection;

use crate::api::transport::Error;
use crate::api::transport::service::SharedExec;

use super::Client;
use super::super::service;

/// Channel builder.
///
/// This struct is used to build and configure HTTP clients.
#[derive(Clone)]
pub struct ApiNode {
    pub(crate) uri: Uri,
    pub(crate) origin: Option<Uri>,
    pub(crate) user_agent: Option<HeaderValue>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) concurrency_limit: Option<usize>,
    pub(crate) rate_limit: Option<(u64, Duration)>,
    pub(crate) buffer_size: Option<usize>,
    pub(crate) tcp_keep_alive: Option<Duration>,
    pub(crate) tcp_nodelay: bool,
    pub(crate) connect_timeout: Option<Duration>,
    pub(crate) executor: SharedExec,
}

impl ApiNode {
    // FIXME: determine if we want to expose this or not. This is really
    // just used in codegen for a shortcut.
    #[doc(hidden)]
    pub fn new<D>(dst: D) -> Result<Self, Error>
        where
            D: TryInto<Self>,
            D::Error: Into<crate::api::Error>,
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
        let uri = Uri::from_static(s);
        Self::from(uri)
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

    /// Set a custom user-agent header.
    ///
    /// `user_agent` will be prepended to Sirius default user-agent string (`sirius/x.x.x`).
    /// It must be a value that can be converted into a valid  `http::HeaderValue` or building
    /// the ApiNode will fail.
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// # let mut builder = ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// builder.user_agent("Greeter").expect("Greeter should be a valid header value");
    /// ```
    pub fn user_agent<T>(self, user_agent: T) -> Result<Self, Error>
        where
            T: TryInto<HeaderValue>,
    {
        user_agent
            .try_into()
            .map(|ua| ApiNode {
                user_agent: Some(ua),
                ..self
            })
            .map_err(|_| Error::new_invalid_user_agent())
    }

    /// Set a custom origin.
    ///
    /// Override the `origin`, mainly useful when you are reaching a Server/LoadBalancer
    /// which serves multiple services at the same time.
    /// It will play the role of SNI (Server Name Indication).
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// # let mut builder = ApiNode::from_static("https://proxy.com");
    /// builder.origin("http://api-1.testnet2.xpxsirius.io:3000".parse().expect("http://example.com must be a valid URI"));
    /// // origin: "http://api-1.testnet2.xpxsirius.io:3000"
    /// ```
    pub fn origin(self, origin: Uri) -> Self {
        ApiNode {
            origin: Some(origin),
            ..self
        }
    }

    /// Apply a timeout to each request.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// # use std::time::Duration;
    /// # let mut builder = ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// builder.timeout(Duration::from_secs(5));
    /// ```
    ///
    /// # Notes
    ///
    /// This does **not** set the timeout metadata (`http-timeout` header) on
    /// the request, meaning the server will not be informed of this timeout,
    /// for that use [`Request::set_timeout`].
    ///
    /// [`Request::set_timeout`]: crate::Request::set_timeout
    pub fn timeout(self, dur: Duration) -> Self {
        ApiNode {
            timeout: Some(dur),
            ..self
        }
    }

    /// Apply a timeout to connecting to the uri.
    ///
    /// Defaults to no timeout.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// # use std::time::Duration;
    /// # let mut builder = ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// builder.connect_timeout(Duration::from_secs(5));
    /// ```
    pub fn connect_timeout(self, dur: Duration) -> Self {
        ApiNode {
            connect_timeout: Some(dur),
            ..self
        }
    }

    /// Set whether TCP keepalive messages are enabled on accepted connections.
    ///
    /// If `None` is specified, keepalive is disabled, otherwise the duration
    /// specified will be the time to remain idle before sending TCP keepalive
    /// probes.
    ///
    /// Default is no keepalive (`None`)
    ///
    pub fn tcp_keep_alive(self, tcp_keep_alive: Option<Duration>) -> Self {
        ApiNode {
            tcp_keep_alive,
            ..self
        }
    }

    /// Apply a concurrency limit to each request.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// # let mut builder = ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// builder.concurrency_limit(256);
    /// ```
    pub fn concurrency_limit(self, limit: usize) -> Self {
        ApiNode {
            concurrency_limit: Some(limit),
            ..self
        }
    }

    /// Apply a rate limit to each request.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// # use std::time::Duration;
    /// # let mut builder = ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// builder.rate_limit(32, Duration::from_secs(1));
    /// ```
    pub fn rate_limit(self, limit: u64, duration: Duration) -> Self {
        ApiNode {
            rate_limit: Some((limit, duration)),
            ..self
        }
    }

    /// Set the value of `TCP_NODELAY` option for accepted connections. Enabled by default.
    pub fn tcp_nodelay(self, enabled: bool) -> Self {
        ApiNode {
            tcp_nodelay: enabled,
            ..self
        }
    }

    pub fn buffer_size(self, size: usize) -> Self {
        ApiNode {
            buffer_size: Some(size),
            ..self
        }
    }

    /// Sets the executor used to spawn async tasks.
    ///
    /// Uses `tokio::spawn` by default.
    pub fn executor<E>(mut self, executor: E) -> Self
        where
            E: Executor<Pin<Box<dyn Future<Output=()> + Send>>> + Send + Sync + 'static,
    {
        self.executor = SharedExec::new(executor);
        self
    }

    /// Create a channel from this config.
    pub async fn connect(&self) -> Result<Client, Error> {
        let mut http = hyper::client::connect::HttpConnector::new();
        http.enforce_http(false);
        http.set_nodelay(self.tcp_nodelay);
        http.set_keepalive(self.tcp_keep_alive);

        let connector = service::connector(http);

        if let Some(connect_timeout) = self.connect_timeout {
            let mut connector = hyper_timeout::TimeoutConnector::new(connector);
            connector.set_connect_timeout(Some(connect_timeout));
            Client::connect(connector, self.clone()).await
        } else {
            Client::connect(connector, self.clone()).await
        }
    }

    /// Create a channel from this config.
    ///
    /// The channel returned by this method does not attempt to connect to the ApiNode until first
    /// use.
    pub fn connect_lazy(&self) -> Client {
        let mut http = hyper::client::connect::HttpConnector::new();
        http.enforce_http(false);
        http.set_nodelay(self.tcp_nodelay);
        http.set_keepalive(self.tcp_keep_alive);

        let connector = service::connector(http);

        if let Some(connect_timeout) = self.connect_timeout {
            let mut connector = hyper_timeout::TimeoutConnector::new(connector);
            connector.set_connect_timeout(Some(connect_timeout));
            Client::new(connector, self.clone())
        } else {
            Client::new(connector, self.clone())
        }
    }

    /// Connect with a custom connector.
    ///
    /// This allows you to build a [Channel](struct.Channel.html) that uses a non-HTTP transport.
    /// See the `uds` example for an example on how to use this function to build channel that
    /// uses a Unix socket transport.
    ///
    /// The [`connect_timeout`](ApiNode::connect_timeout) will still be applied.
    pub async fn connect_with_connector<C>(&self, connector: C) -> Result<Client, Error>
        where
            C: MakeConnection<Uri> + Send + 'static,
            C::Connection: Unpin + Send + 'static,
            C::Future: Send + 'static,
            crate::api::Error: From<C::Error> + Send + 'static,
    {
        let connector = service::connector(connector);

        if let Some(connect_timeout) = self.connect_timeout {
            let mut connector = hyper_timeout::TimeoutConnector::new(connector);
            connector.set_connect_timeout(Some(connect_timeout));
            Client::connect(connector, self.clone()).await
        } else {
            Client::connect(connector, self.clone()).await
        }
    }

    /// Connect with a custom connector lazily.
    ///
    /// This allows you to build a [Channel](struct.Channel.html) that uses a non-HTTP transport
    /// connect to it lazily.
    ///
    /// See the `uds` example for an example on how to use this function to build channel that
    /// uses a Unix socket transport.
    pub fn connect_with_connector_lazy<C>(&self, connector: C) -> Client
        where
            C: MakeConnection<Uri> + Send + 'static,
            C::Connection: Unpin + Send + 'static,
            C::Future: Send + 'static,
            crate::api::Error: From<C::Error> + Send + 'static,
    {
        let connector = service::connector(connector);

        Client::new(connector, self.clone())
    }

    /// Get the ApiNode uri.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::ApiNode;
    /// # use http::Uri;
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
            origin: None,
            user_agent: None,
            concurrency_limit: None,
            rate_limit: None,
            timeout: None,
            buffer_size: None,
            tcp_keep_alive: None,
            tcp_nodelay: true,
            connect_timeout: None,
            executor: SharedExec::tokio(),
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
