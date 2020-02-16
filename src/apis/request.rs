extern crate url;

use std::collections::HashMap;
use std::sync::Arc;

use hyper::{Body, Client, http, StatusCode};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use serde::Serializer;
use serde_json;

use crate::apis::error::{Error, SiriusError};
use crate::apis::sirius_client::ApiClient;

use self::url::form_urlencoded;

pub(crate) struct Request {
    method: hyper::Method,
    path: String,
    query_params: HashMap<String, String>,
    no_return_type: bool,
    path_params: HashMap<String, String>,
    form_params: HashMap<String, String>,
    header_params: HashMap<String, String>,
    // TODO: multiple body params are possible technically, but not supported here.
    serialized_body: Option<String>,
}

impl Request {
    pub fn new(method: hyper::Method, path: String) -> Self {
        Request {
            method,
            path,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            form_params: HashMap::new(),
            header_params: HashMap::new(),
            serialized_body: None,
            no_return_type: false,
        }
    }

    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_header_param(mut self, basename: String, param: String) -> Self {
        self.header_params.insert(basename, param);
        self
    }

    pub fn with_query_param(mut self, basename: String, param: String) -> Self {
        self.query_params.insert(basename, param);
        self
    }

    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        self.path_params.insert(basename, param);
        self
    }

    pub fn with_form_param(mut self, basename: String, param: String) -> Self {
        self.form_params.insert(basename, param);
        self
    }

    pub fn returns_nothing(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    pub async fn execute<'a, C, U>(self, api: Arc<ApiClient<C>>) -> Result<U, Error<serde_json::Value>>
        where
            C: hyper::client::connect::Connect + Send + Clone + Sync + 'static,
            for<'de> U: serde::Deserialize<'de>,
    {

        // raw_headers is for headers we don't know the proper type of (e.g. custom api key
        // headers); headers is for ones we do know the type of.
        let mut raw_headers = HashMap::new();
        let mut headers: hyper::header::HeaderMap = hyper::header::HeaderMap::new();

        let mut path = self.path;
        for (k, v) in self.path_params {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{}}}", k), &v);
        }

        for (k, v) in self.header_params {
            raw_headers.insert(k, v);
        }

        let mut query_string = ::url::form_urlencoded::Serializer::new("".to_owned());

        for (key, val) in self.query_params {
            query_string.append_pair(&key, &val);
        }

        let mut uri_str = format!("{}{}", api.base_path, path);

        let query_string_str = query_string.finish();
        if query_string_str != "" {
            uri_str += "?";
            uri_str += &query_string_str;
        }

        let uri: hyper::Uri = match uri_str.parse()
        {
            Err(e) => {
                return Err(Error::UriError(e));
            }
            Ok(u) => u,
        };
        println!("{:?}", uri);

        let mut req = hyper::Request::builder()
            .method(self.method)
            .uri(uri)
            .body(Body::empty())
            .expect("request builder");
        {
            let req_headers = req.headers_mut();
            if let Some(ref user_agent) = api.user_agent {
                req_headers.insert(USER_AGENT, user_agent.clone().parse().unwrap());
            }

            req_headers.extend(headers);
//
//            for (key, val) in raw_headers {
//                req_headers.append(key, hyper::http::HeaderValue::from_str(&val).unwrap());
//            }

            req.headers_mut().insert(CONTENT_TYPE, "application/json".parse().unwrap());

            if let Some(body) = self.serialized_body {
                req.headers_mut().insert(CONTENT_TYPE, "application/json".parse().unwrap());
                req.headers_mut().insert(CONTENT_LENGTH, body.len().into());
//                req.into_body();
            }

            let no_ret_type = self.no_return_type;

            let mut resp = api.client.request(req).await?;

            let status = resp.status_mut();

            match *status {
                StatusCode::NOT_FOUND => {
                    let body = hyper::body::to_bytes(resp).await?;

                    let _err: SiriusError = serde_json::from_slice(&body).unwrap();

                    let _resp_err: Result<U, _> = Result::Err(Error::SiriusError(_err));

                    return _resp_err;
                }
                _ => {
                    let body = hyper::body::to_bytes(resp).await?;

                    let res: U = serde_json::from_slice(&body)?;

                    Ok(res)
                }
            }
        }
    }
}
