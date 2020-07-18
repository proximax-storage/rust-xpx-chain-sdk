/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    std::{
        borrow::Cow,
        fmt::{Display, Formatter},
        result,
    },
    tokio_tungstenite::tungstenite::Error as WsError,
};

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
    Reqwest(reqwest::Error),
    Tungsten(WsError),
    Failure(failure::Error),
    Url(Cow<'static, str>),
}

impl ::failure::Fail for Error {}

impl From<reqwest::Error> for Error {
    fn from(reqwest: reqwest::Error) -> Self {
        Error::Reqwest(reqwest)
    }
}

impl From<SiriusError> for Error {
    fn from(sirius: SiriusError) -> Self {
        Error::SiriusError(sirius)
    }
}

impl From<WsError> for Error {
    fn from(ws: WsError) -> Self {
        Error::Tungsten(ws)
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde: serde_json::Error) -> Self {
        Error::Serde(serde)
    }
}

impl From<failure::Error> for Error {
    fn from(failure: failure::Error) -> Self {
        Error::Failure(failure)
    }
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Error::Failure(failure::err_msg(msg))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
        match self.to_owned() {
            Error::SiriusError(e) => {
                write!(f, "{{ code: \"{}\", message: \"{}\" }}", e.code, e.message)
            }
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Serde(e) => write!(f, "{}", e),
            Error::Tungsten(e) => write!(f, "{}", e),
            Error::Failure(e) => write!(f, "{}", e),
            Error::Url(ref msg) => write!(f, "{}", msg),
        }
    }
}
