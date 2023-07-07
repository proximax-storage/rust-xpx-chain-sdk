use std::{
    fmt,
    task::{Context, Poll},
};

use http::Uri;
use hyper::client::connect::Connection as HyperConnection;
use tokio::io::{AsyncRead, AsyncWrite};
use tower::load::Load;
use tower::util::BoxCloneService;
use tower::{layer::Layer, limit::concurrency::ConcurrencyLimitLayer, ServiceBuilder, ServiceExt};
use tower_service::Service;

use crate::api::transport::service::http_timeout::HttpTimeout;
use crate::api::transport::ApiNode;

use super::super::BoxFuture;
use super::{AddOrigin, UserAgent};

pub(crate) type Request = hyper::Request<hyper::Body>;
pub(crate) type Response = hyper::Response<hyper::Body>;

#[derive(Clone)]
pub struct Connection {
    inner: BoxCloneService<Request, Response, crate::api::Error>,
}

impl Connection {
    fn new<C>(connector: C, endpoint: ApiNode, is_lazy: bool) -> Self
    where
        C: Service<Uri> + Send + 'static + Clone + Sync,
        C::Error: Into<crate::api::Error> + Send,
        C::Future: Unpin + Send,
        C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        let executor = endpoint.executor.clone();

        let stack = ServiceBuilder::new()
            .layer_fn(|s| UserAgent::new(s, endpoint.user_agent.clone()))
            .layer_fn(|s| {
                let origin = endpoint.origin.as_ref().unwrap_or(&endpoint.uri).clone();
                AddOrigin::new(s, origin)
            })
            .layer_fn(|s| HttpTimeout::new(s, endpoint.timeout))
            .option_layer(endpoint.concurrency_limit.map(ConcurrencyLimitLayer::new))
            // .option_layer(endpoint.rate_limit.map(|(l, d)| RateLimitLayer::new(l, d)))
            .into_inner();

        let svc = hyper::Client::builder()
            .executor(executor)
            .build::<C, hyper::Body>(connector);
        // let conn = Reconnect::new(connector, endpoint.uri.clone(), is_lazy);

        let inner = stack.layer(svc);

        Self {
            inner: BoxCloneService::new(inner),
        }
    }

    pub(crate) async fn connect<C>(
        connector: C,
        endpoint: ApiNode,
    ) -> Result<Self, crate::api::Error>
    where
        C: Service<Uri> + Send + 'static + Clone + Sync,
        C::Error: Into<crate::api::Error> + Send,
        C::Future: Unpin + Send,
        C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        Self::new(connector, endpoint, false).ready_oneshot().await
    }

    pub(crate) fn lazy<C>(connector: C, endpoint: ApiNode) -> Self
    where
        C: Service<Uri> + Send + 'static + Clone + Sync,
        C::Error: Into<crate::api::Error> + Send,
        C::Future: Unpin + Send,
        C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        Self::new(connector, endpoint, true)
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
        self.inner.call(request)
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
