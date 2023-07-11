use std::{
    fmt,
    task::{Context, Poll},
};

use http::{HeaderValue, Uri};
use http::uri::{Authority, Scheme};
use hyper::client::connect::Connection as HyperConnection;
use tokio::io::{AsyncRead, AsyncWrite};
use tower::load::Load;
use tower::util::BoxCloneService;
use tower::{
    layer::Layer, limit::concurrency::ConcurrencyLimitLayer, BoxError, ServiceBuilder, ServiceExt,
};
use tower_service::Service;

use crate::api::transport::service::http_timeout::HttpTimeout;
use crate::api::transport::service::{retry, Connector, ReplayBody};
use crate::api::transport::ApiNode;

use super::super::BoxFuture;
use super::{AddOrigin, UserAgent};

pub type Request = hyper::Request<ReplayBody>;
pub type Response = hyper::Response<hyper::Body>;
pub type TimeoutConnector = hyper_timeout::TimeoutConnector<Connector>;

#[derive(Clone)]
pub struct Connection {
    // inner: BoxCloneService<Request, Response, crate::api::Error>,
    inner:  hyper::Client<TimeoutConnector, ReplayBody>,
    scheme: Option<Scheme>,
    authority: Option<Authority>,
}

impl Connection {
    fn new(connector: TimeoutConnector, endpoint: ApiNode) -> Self {
        let executor = endpoint.executor.clone();
        let inner = hyper::Client::builder()
            .executor(executor)
            .build::<TimeoutConnector, ReplayBody>(connector);

        // let stack = ServiceBuilder::new()
        //     .boxed_clone()
        //     .layer_fn(|s| UserAgent::new(s, endpoint.user_agent.clone()))
        //     .layer_fn(|s| AddOrigin::new(s, endpoint.uri.clone()))
        //     .layer_fn(|s| HttpTimeout::new(s, endpoint.timeout))
        //     .option_layer(endpoint.concurrency_limit.map(ConcurrencyLimitLayer::new))
        //     // .option_layer(endpoint.rate_limit.map(|(l, d)| RateLimitLayer::new(l, d)))
        //     .into_inner();
        //
        // let policy = retry::RetryPolicy(endpoint.max_num_retry.unwrap_or(3));
        // let conn = tower::retry::Retry::new(policy, inner);
        // let inner = stack.layer(conn);

        let http::uri::Parts {
            scheme, authority, ..
        } = endpoint.uri.into_parts();

        Self {
            inner,
            scheme,
            authority,
        }
    }

    pub(crate) async fn connect(
        connector: TimeoutConnector,
        endpoint: ApiNode,
    ) -> Result<Self, crate::api::Error> {
        Self::new(connector, endpoint).ready_oneshot().await
    }

    pub(crate) fn lazy(connector: TimeoutConnector, endpoint: ApiNode) -> Self {
        Self::new(connector, endpoint)
    }
}

impl Service<Request> for Connection {
    type Response = Response;
    type Error = crate::api::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Service::poll_ready(&mut self.inner, cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        if self.scheme.is_none() || self.authority.is_none() {
            let err = crate::api::transport::Error::new_invalid_uri();
            return Box::pin(async move { Err::<Self::Response, _>(err.into()) });
        }

        // Split the request into the head and the body.
        let (mut head, body) = request.into_parts();

        // Update the the request URI
        head.uri = {
            // Split the request URI into parts.
            let mut uri: http::uri::Parts = head.uri.into();
            // Update the URI parts, setting hte scheme and authority
            uri.scheme = self.scheme.clone();
            uri.authority = self.authority.clone();

            Uri::from_parts(uri).expect("valid uri")
        };
        head.headers.append(
            "Host",
            HeaderValue::from_bytes(head.uri.host().as_ref().unwrap().as_bytes())
                .expect("host header should be valid"),
        );

        let request = Request::from_parts(head, body);

        let fut = self.inner.call(request);

        Box::pin(async move { fut.await.map_err(Into::into) })

    }
}

impl Load for Connection {
    type Metric = usize;

    fn load(&self) -> Self::Metric {
        0
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection").finish()
    }
}
