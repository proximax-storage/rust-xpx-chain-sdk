/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

#[doc(inline)]
// #[cfg(feature = "node")]
pub use self::client::{ApiNode, Client};
pub use self::error::Error;

// pub use self::service::http_timeout::TimeoutExpired;
pub(crate) mod client;

mod error;
pub(crate) mod service;

type BoxFuture<T, E> =
std::pin::Pin<Box<dyn std::future::Future<Output=Result<T, E>> + Send + 'static>>;
