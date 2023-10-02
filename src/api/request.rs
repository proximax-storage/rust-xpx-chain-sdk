/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use hyper::{Method, StatusCode};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use tower::ServiceExt;

use {::std::collections::HashMap, serde_json};

use crate::api;
use crate::api::error::{Error, SiriusError};
use crate::api::service::service::ReplayBody;
use crate::api::transport::service::Connection;

#[derive(Clone)]
pub(crate) struct ApiRequest {
    method: Method,
    path: String,
    query_params: Option<String>,
    path_params: HashMap<String, String>,
    // TODO: multiple body params are possible technically, but not supported here.
    serialized_body: Option<String>,
}

impl ApiRequest {
    pub fn new(method: Method, path: String) -> Self {
        ApiRequest {
            method,
            path,
            query_params: None,
            path_params: HashMap::new(),
            serialized_body: None,
        }
    }

    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_query_param(mut self, param: String) -> Self {
        self.query_params = Some(param);
        self
    }

    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        let param = param.replace('"', "");
        self.path_params.insert(basename, param);
        self
    }

    pub async fn execute<U>(&self, connection: Connection) -> api::error::Result<U>
        where
                for<'de> U: serde::Deserialize<'de>,
    {
        let mut path = self.path.to_owned();

        self.path_params.iter().for_each(|(key, val)| {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{}}}", key), &val);
        });

        let mut uri_str = format!("{}", path);

        if let Some(ref params) = self.query_params {
            uri_str.push_str(&format!("?{}", params));
        };

        let req = if let Some(ref body) = self.serialized_body {
            let max_body_len = body.len();
            let mut req = hyper::Request::builder()
                .uri(uri_str.as_str())
                .method(self.method.clone())
                .body(ReplayBody::try_new(hyper::Body::from(body.clone()), max_body_len).unwrap())
                .unwrap();
            req.headers_mut().insert(
                CONTENT_TYPE,
                "application/json".parse().map_err(|err| Error::from(format_err!("{}", err)))?,
            );

            req.headers_mut().insert(CONTENT_LENGTH, body.len().into());
            req
        } else {
            hyper::Request::builder()
                .uri(uri_str.as_str())
                .method(self.method.clone())
                .body(ReplayBody::try_new(hyper::Body::empty(), 8).unwrap())
                .unwrap()
        };

        let resp = connection.oneshot(req).await?;

        let status = resp.status();

        if status.as_u16() > 226 || status.as_u16() < 200 {
            return Err(Error::from(anyhow!("Internal error statusCode {}", status)));
        }
        let body = resp.into_body();

        let bytes = hyper::body::to_bytes(body).await?;

        match status {
            StatusCode::OK | StatusCode::ACCEPTED => {
                let res: U = serde_json::from_slice(&bytes)?;
                Ok(res)
            }
            _ => {
                let err: SiriusError = serde_json::from_slice(&bytes)?;
                Err(Error::from(err))
            }
        }
    }
}
