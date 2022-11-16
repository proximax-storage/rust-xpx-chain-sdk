use super::client::Client;
use super::connection::Error;
use super::executor::SharedExec;
use anyhow::anyhow;
use bytes::Bytes;
use http::{HeaderValue, Uri};
use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use crate::api::transport;

/// Channel builder.
///
/// This struct is used to build and configure HTTP/2 channels.
#[derive(Clone)]
pub struct Endpoint {
    pub(crate) uri: Uri,
    pub(crate) user_agent: Option<HeaderValue>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) concurrency_limit: Option<usize>,
    pub(crate) rate_limit: Option<(u64, Duration)>,
    #[cfg(feature = "tls")]
    pub(crate) tls: Option<TlsConnector>,
    pub(crate) buffer_size: Option<usize>,
    pub(crate) tcp_keepalive: Option<Duration>,
    pub(crate) tcp_nodelay: bool,
    pub(crate) connect_timeout: Option<Duration>,
    pub(crate) executor: SharedExec,
}

impl Endpoint {
    /// Convert an `Endpoint` from a static string.
    ///
    /// # Panics
    ///
    /// This function panics if the argument is an invalid URI.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Endpoint;
    /// Endpoint::from_static("https://example.com");
    /// ```
    pub fn from_static(s: &'static str) -> Self {
        let uri = Uri::from_static(s);
        Self::from(uri)
    }

    /// Convert an `Endpoint` from shared bytes.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Endpoint;
    /// Endpoint::from_shared("https://example.com".to_string());
    /// ```
    pub fn from_shared(s: impl Into<Bytes>) -> Result<Self, anyhow::Error> {
        let uri = Uri::from_maybe_shared(s.into()).map_err(|e| anyhow!(e))?;
        Ok(Self::from(uri))
    }

    /// Apply a timeout to each request.
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use xpx_chain_sdk::api::Endpoint;
    /// # let mut builder = Endpoint::from_static("https://example.com");
    /// builder.timeout(Duration::from_secs(5));
    /// ```
    ///
    /// # Notes
    ///
    /// This does **not** set the timeout metadata (`grpc-timeout` header) on
    /// the request, meaning the server will not be informed of this timeout,
    /// for that use [`Request::set_timeout`].
    ///
    /// [`Request::set_timeout`]: crate::Request::set_timeout
    pub fn timeout(self, dur: Duration) -> Self {
        Endpoint {
            timeout: Some(dur),
            ..self
        }
    }

    /// Apply a timeout to connecting to the uri.
    ///
    /// Defaults to no timeout.
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use xpx_chain_sdk::api::Endpoint;
    /// # let mut builder = Endpoint::from_static("https://example.com");
    /// builder.connect_timeout(Duration::from_secs(5));
    /// ```
    pub fn connect_timeout(self, dur: Duration) -> Self {
        Endpoint {
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
    pub fn tcp_keepalive(self, tcp_keepalive: Option<Duration>) -> Self {
        Endpoint {
            tcp_keepalive,
            ..self
        }
    }

    /// Apply a concurrency limit to each request.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Endpoint;
    /// # let mut builder = Endpoint::from_static("https://example.com");
    /// builder.concurrency_limit(256);
    /// ```
    pub fn concurrency_limit(self, limit: usize) -> Self {
        Endpoint {
            concurrency_limit: Some(limit),
            ..self
        }
    }

    /// Apply a rate limit to each request.
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use xpx_chain_sdk::api::Endpoint;
    /// # let mut builder = Endpoint::from_static("https://example.com");
    /// builder.rate_limit(32, Duration::from_secs(1));
    /// ```
    pub fn rate_limit(self, limit: u64, duration: Duration) -> Self {
        Endpoint {
            rate_limit: Some((limit, duration)),
            ..self
        }
    }

    /// Set that all sockets have `SO_NODELAY` set to the supplied value `nodelay`.
    ///
    /// Default is `false`.
    pub fn set_nodelay(self, nodelay: bool) -> Self {
        Endpoint {
            tcp_nodelay: nodelay,
            ..self
        }
    }

    /// Create a channel from this config.
    pub async fn connect(&self) -> Result<Client, Error> {
        let mut http = hyper::client::connect::HttpConnector::new();
        http.enforce_http(false);
        http.set_nodelay(self.tcp_nodelay);
        http.set_keepalive(self.tcp_keepalive);

        #[cfg(feature = "tls")]
        let connector = service::connector(http, self.tls.clone());

        #[cfg(not(feature = "tls"))]
        let connector = transport::connector(http);

        if let Some(connect_timeout) = self.connect_timeout {
            let mut connector = hyper_timeout::TimeoutConnector::new(connector);
            connector.set_connect_timeout(Some(connect_timeout));
            Client::connect(connector, self.clone()).await
        } else {
            Client::connect(connector, self.clone()).await
        }
    }

    /// Get the endpoint uri.
    ///
    /// ```
    /// # use http::Uri;
    /// # use xpx_chain_sdk::api::Endpoint;
    /// let endpoint = Endpoint::from_static("https://example.com");
    ///
    /// assert_eq!(endpoint.uri(), &Uri::from_static("https://example.com"));
    /// ```
    pub fn uri(&self) -> &Uri {
        &self.uri
    }
}

impl From<Uri> for Endpoint {
    fn from(uri: Uri) -> Self {
        Self {
            uri,
            user_agent: None,
            concurrency_limit: None,
            rate_limit: None,
            timeout: None,
            #[cfg(feature = "tls")]
            tls: None,
            buffer_size: None,
            tcp_keepalive: None,
            tcp_nodelay: false,
            connect_timeout: None,
            executor: SharedExec::tokio(),
        }
    }
}

// impl TryFrom<Bytes> for Endpoint {
//     type Error = Error;
//
//     fn try_from(t: Bytes) -> Result<Self, Self::Error> {
//         Self::from_shared(t)
//     }
// }

// impl TryFrom<String> for Endpoint {
//     type Error = Error;
//
//     fn try_from(t: String) -> Result<Self, Self::Error> {
//         Self::from_shared(t.into_bytes())
//     }
// }
//
// impl TryFrom<&'static str> for Endpoint {
//     type Error = Error;
//
//     fn try_from(t: &'static str) -> Result<Self, Self::Error> {
//         Self::from_shared(t.as_bytes())
//     }
// }

impl fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Endpoint").finish()
    }
}

// impl FromStr for Endpoint {
//     type Err = Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Self::try_from(s.to_string())
//     }
// }
