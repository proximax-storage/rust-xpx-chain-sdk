use ::std::collections::HashMap;
use ::std::sync::Arc;

use hyper::{
    Body,
    header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderMap, USER_AGENT},
    StatusCode,
    Uri};
use serde_json;

use crate::apis::{
    error::{
        Error,
        SiriusError,
    },
    internally::{
        map_transaction_dto,
        map_transaction_dto_vec
    },
    sirius_client::ApiClient,
};

pub(crate) struct Request {
    method: hyper::Method,
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
    pub fn new(method: hyper::Method, path: String) -> Self {
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
        self.path_params.insert(basename, param);
        self
    }

    pub fn is_transaction(mut self) -> Self {
        self.is_transaction = true;
        self
    }

    pub fn is_transaction_vec(mut self) -> Self {
        self.is_transaction_vec = true;
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
        let headers: HeaderMap = HeaderMap::new();

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

        let uri: Uri = match uri_str.parse()
        {
            Err(e) => {
                return Err(Error::from(e));
            }
            Ok(u) => u,
        };

        let mut req_body = Body::empty();

        if let Some(body) = self.serialized_body.clone() {
            req_body = Body::from(body);
        }

        let mut req = hyper::Request::builder()
            .method(self.method)
            .uri(uri)
            .body(req_body)
            .expect("request builder");
        {
            let req_headers = req.headers_mut();
            if let Some(ref user_agent) = api.user_agent {
                req_headers.insert(USER_AGENT, user_agent.clone().parse()
                    .map_err(|_err| {
                        Error::from(format_err!("{}", _err))
                    })?);
            }

            req_headers.extend(headers);

//            for (key, val) in raw_headers {
//                req_headers.append(key, hyper::http::HeaderValue::from_str(&val).unwrap());
//            }

            if let Some(body) = self.serialized_body {
                req.headers_mut().insert(CONTENT_TYPE, "application/json".parse()
                    .map_err(|_err| {
                        Error::from(format_err!("{}", _err))
                    })?);

                req.headers_mut().insert(CONTENT_LENGTH, body.len().into());
            }

            let mut resp = api.client.request(req).await?;

            let status = resp.status_mut();

            match *status {
                StatusCode::OK | StatusCode::ACCEPTED => {
                    let body = hyper::body::to_bytes(resp).await?;

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
                    let body = hyper::body::to_bytes(resp).await?;

                    let err: SiriusError = serde_json::from_slice(&body)?;

                    let resp_err: Result<U, _> = Err(Error::from(err));

                    return resp_err;
                }
            }
        }
    }
}
