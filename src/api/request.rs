/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::{collections::HashMap, sync::Arc},
    reqwest::{
        header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderMap, USER_AGENT},
        Method, StatusCode, Url,
    },
    serde_json,
};

use crate::models::error::{Error, SiriusError};

use super::{
    internally::{map_transaction_dto, map_transaction_dto_vec},
    sirius_client::ApiClient,
};

#[derive(Clone)]
pub(crate) struct Request {
    method: Method,
    path: String,
    query_params: HashMap<String, String>,
    path_params: HashMap<String, String>,
    header_params: HashMap<String, String>,
    // TODO: multiple body params are possible technically, but not supported here.
    serialized_body: Option<String>,
    is_transaction: bool,
    is_transaction_vec: bool,
}

impl Request {
    pub fn new(method: reqwest::Method, path: String) -> Self {
        Request {
            method,
            path,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            header_params: HashMap::new(),
            serialized_body: None,
            is_transaction: false,
            is_transaction_vec: false,
        }
    }

    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_query_param(mut self, basename: String, param: String) -> Self {
        self.query_params.insert(basename, param);
        self
    }

    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        let param = param.replace('"', "");
        self.path_params.insert(basename, param);
        self
    }

    pub fn set_transaction(mut self) -> Self {
        self.is_transaction = true;
        self
    }

    pub fn set_transaction_vec(mut self) -> Self {
        self.is_transaction_vec = true;
        self
    }

    pub async fn execute<U>(self, api: Arc<ApiClient>) -> crate::models::Result<U>
        where
                for<'de> U: serde::Deserialize<'de>,
    {
        // raw_headers is for headers we don't know the proper type of (e.g. custom api key
        // headers); headers is for ones we do know the type of.
        let mut raw_headers = HashMap::new();
        let headers: HeaderMap = HeaderMap::new();

        let mut path = self.path;

        self.path_params.into_iter().for_each(|(key, val)| {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{}}}", key), &val);
        });

        self.header_params.into_iter().for_each(|(key, val)| {
            raw_headers.insert(key, val);
        });

        let uri_str = format!("{}{}", api.base_path, path);

        let mut url = Url::parse(&uri_str)
            .map_err(|e| format!("could not parse url: {:?}", e))
            .unwrap();

        if !self.query_params.is_empty() {
            let existing: Vec<(String, String)> = url
                .query_pairs()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();

            // final pairs
            let mut pairs: Vec<(&str, &str)> = Vec::new();

            // add first existing
            for pair in &existing {
                pairs.push((&pair.0, &pair.1));
            }

            // add given query to the pairs
            for (key, val) in self.query_params.iter() {
                pairs.push((key, val));
            }

            // set new pairs
            url.query_pairs_mut()
                .clear()
                .extend_pairs(pairs.iter().map(|&(k, v)| (&k[..], &v[..])));
        };

        // create request
        let builder = api.client.request(self.method, url.as_str()).body(
            self.serialized_body
                .clone()
                .unwrap_or_else(|| "".to_owned()),
        );

        let mut req = builder.build()?;

        if let Some(body) = self.serialized_body {
            req.headers_mut().insert(
                CONTENT_TYPE,
                "application/json"
                    .parse()
                    .map_err(|err| Error::from(format_err!("{}", err)))?,
            );

            req.headers_mut().insert(CONTENT_LENGTH, body.len().into());
        }

        let req_headers = req.headers_mut();
        if let Some(ref user_agent) = api.user_agent {
            req_headers.insert(
                USER_AGENT,
                user_agent
                    .parse()
                    .map_err(|err| Error::from(format_err!("{}", err)))?,
            );
        }

        req_headers.extend(headers);

        let resp = api.client.execute(req).await?;

        let status = resp.status();

        let body = resp.bytes().await?;

        match status {
            StatusCode::OK | StatusCode::ACCEPTED => {
                if self.is_transaction {
                    let map_dto = map_transaction_dto(body)?;
                    let res: U = serde_json::from_str(&map_dto)?;
                    Ok(res)
                } else if self.is_transaction_vec {
                    let map_dto_vec = map_transaction_dto_vec(body)?;
                    let res: U = serde_json::from_str(&map_dto_vec)?;
                    Ok(res)
                } else {
                    let res: U = serde_json::from_slice(&body)?;
                    Ok(res)
                }
            }
            _ => {
                let err: SiriusError = serde_json::from_slice(&body)?;
                Err(Error::from(err))
            }
        }
    }
}
