#[doc(inline)]
// #[cfg(feature = "node")]
pub use self::client::{ApiNode, Client};
pub use self::error::Error;

// pub use self::service::http_timeout::TimeoutExpired;
pub(crate) mod client;

mod error;
pub(crate) mod service;

type BoxFuture<T, E> =
	std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send + 'static>>;
