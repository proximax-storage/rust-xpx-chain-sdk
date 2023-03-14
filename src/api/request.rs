/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{Method, StatusCode};
use tower::ServiceExt;

use {::std::collections::HashMap, serde_json};

use crate::api;
use crate::api::error::{Error, SiriusError};
use crate::api::{Client, QueryParam};

#[derive(Clone)]
pub(crate) struct ApiRequest {
    method: Method,
    path: String,
    query_params: Vec<QueryParam>,
    path_params: HashMap<String, String>,
    header_params: HashMap<String, String>,
    // TODO: multiple body params are possible technically, but not supported here.
    serialized_body: Option<String>,
}

impl ApiRequest {
    pub fn new(method: Method, path: String) -> Self {
        ApiRequest {
            method,
            path,
            query_params: vec![],
            path_params: HashMap::new(),
            header_params: HashMap::new(),
            serialized_body: None,
        }
    }

    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_query_param(mut self, param: QueryParam) -> Self {
        self.query_params.push(param);
        self
    }

    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        let param = param.replace('"', "");
        self.path_params.insert(basename, param);
        self
    }

    pub async fn execute<U>(self, api: Client) -> api::error::Result<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        // raw_headers is for headers we don't know the proper type of (e.g. custom api key
        // headers); headers is for ones we do know the type of.
        let mut raw_headers = HashMap::new();

        let mut path = self.path;

        self.path_params.into_iter().for_each(|(key, val)| {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{}}}", key), &val);
        });

        self.header_params.into_iter().for_each(|(key, val)| {
            raw_headers.insert(key, val);
        });

        let mut uri_str = format!("{}", path);

        if !self.query_params.is_empty() {
            let mut buf = String::from("?");

            // add given query to the pairs
            for param in self.query_params.iter() {
                let (key, val) = param.key_value_to_str();
                buf.push_str((format!("{}={}&", key, val)).as_str());
            }

            let beta_offset = buf.len();

            // Remove the range up until the β from the string
            let buf: String = buf.drain(..beta_offset - 1).collect();
            uri_str.push_str(&format!("{}", buf));
        };

        let req = if let Some(body) = self.serialized_body {
            let mut req = hyper::Request::builder()
                .uri(uri_str.as_str())
                .method(self.method.clone())
                .body(hyper::Body::from(body.clone()))
                .unwrap();
            req.headers_mut().insert(
                CONTENT_TYPE,
                "application/json"
                    .parse()
                    .map_err(|err| Error::from(format_err!("{}", err)))?,
            );

            req.headers_mut().insert(CONTENT_LENGTH, body.len().into());
            req
        } else {
            hyper::Request::builder()
                .uri(uri_str.as_str())
                .method(self.method.clone())
                .body(hyper::Body::empty())
                .unwrap()
        };

        let resp = api.oneshot(req).await?;

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
