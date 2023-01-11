/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::std::{
    fmt::{Display, Formatter},
    result,
};

use crate::api;

/// Result type of all Websocket library calls.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiriusError {
    pub code: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    SiriusError(SiriusError),
    Hyper(hyper::Error),
    Transport(api::transport::Error),
    Failure(anyhow::Error),
}

impl From<api::transport::Error> for Error {
    fn from(err: api::transport::Error) -> Self {
        Error::Transport(err)
    }
}

impl From<SiriusError> for Error {
    fn from(err: SiriusError) -> Self {
        Error::SiriusError(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Failure(err)
    }
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Error::Failure(anyhow!(msg))
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        use Error::*;
        match self.to_owned() {
            SiriusError(e) => write!(f, "{{ code: \"{}\", message: \"{}\" }}", e.code, e.message),
            Transport(e) => write!(f, "{}", e),
            Hyper(e) => write!(f, "{}", e),
            Serde(e) => write!(f, "{}", e),
            Failure(e) => write!(f, "{}", e),
        }
    }
}
