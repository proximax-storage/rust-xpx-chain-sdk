//! Batteries included server and client.
//!
//! This module provides a set of batteries included, fully featured and
//! fast set of HTTP/2 server and client's. These components each provide a or
//! `rustls` tls backend when the respective feature flag is enabled, and
//! provides builders to configure transport behavior.
//!
//! # Features
//!
//! - TLS support via [rustls].
//! - Load balancing
//! - Timeouts
//! - Concurrency Limits
//! - Rate limiting
//!
//! # Examples
//!
//! ## Client
//!
//! ```no_run
//! # use tonic::transport::{Channel, Certificate, ClientTlsConfig};
//! # use std::time::Duration;
//! # use tonic::body::BoxBody;
//! # use tonic::client::GrpcService;;
//! # use http::Request;
//! # #[cfg(feature = "rustls")]
//! # async fn do_thing() -> Result<(), Box<dyn std::error::Error>> {
//! let cert = std::fs::read_to_string("ca.pem")?;
//!
//! let mut channel = Channel::from_static("http://api-1.testnet2.xpxsirius.io:3000")
//!     .tls_config(ClientTlsConfig::new()
//!         .ca_certificate(Certificate::from_pem(&cert))
//!         .domain_name("example.com".to_string()))?
//!     .timeout(Duration::from_secs(5))
//!     .rate_limit(5, Duration::from_secs(1))
//!     .concurrency_limit(256)
//!     .connect()
//!     .await?;
//!
//! channel.call(Request::new(BoxBody::empty())).await?;
//! # Ok(())
//! # }
//! ```

pub use hyper::{Body, Uri};

#[doc(inline)]
// #[cfg(feature = "node")]
pub use self::client::{ApiNode, Client};
pub use self::error::Error;

// pub use self::service::grpc_timeout::TimeoutExpired;
pub(crate) mod client;

mod error;
mod service;

type BoxFuture<T, E> =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send + 'static>>;
