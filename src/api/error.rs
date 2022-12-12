/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::{
        borrow::Cow,
        fmt::{Display, Formatter},
        num, result,
    },
    tokio_tungstenite::tungstenite::Error as WsError,
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
    Tungsten(WsError),
    Failure(anyhow::Error),
    Url(Cow<'static, str>),
    Parse(num::ParseIntError),
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

impl From<WsError> for Error {
    fn from(err: WsError) -> Self {
        Error::Tungsten(err)
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

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::Parse(err)
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
            Tungsten(e) => write!(f, "{:?}", e),
            Failure(e) => write!(f, "{}", e),
            Url(ref msg) => write!(f, "{}", msg),
            Parse(e) => write!(f, "{}", e),
        }
    }
}
