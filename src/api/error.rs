/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::result;

use tower::BoxError;

use crate::api;

/// Result type of all Websocket library calls.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiriusError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    SiriusError(#[from] SiriusError),
    #[error("{0}")]
    Hyper(#[from] hyper::Error),
    #[error("{0}")]
    Transport(#[from] api::transport::Error),
    #[error("{0}")]
    BuildUninitializedField(#[from] derive_builder::UninitializedFieldError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    Tower(#[from] BoxError),
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Error::Other(anyhow!(msg))
    }
}

impl core::fmt::Display for SiriusError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{{ code: \"{}\", message: \"{}\" }}", self.code, self.message)
    }
}
