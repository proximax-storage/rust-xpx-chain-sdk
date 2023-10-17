/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

pub use self::transport::*;
pub mod transport;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
/// By default, the Err value is of type [`transport::Error`] but this can be overridden if desired.
pub type Result<T, E = Error> = std::result::Result<T, E>;
