use std::{fmt, io};
use std::task::{Context, Poll};

use http::Uri;
use hyper::client::conn::Builder;
use hyper::client::connect::Connection as HyperConnection;
use hyper::client::service::Connect as HyperConnect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower::{Layer, Service, ServiceBuilder, ServiceExt};
use tower::limit::{ConcurrencyLimitLayer, RateLimitLayer};
use tower::load::Load;
use tower::timeout::TimeoutLayer;
use tower::util::BoxService;

use super::body::BoxBody;
use super::node::Endpoint;
// use crate::body::BoxBody;
use super::reconnect::Reconnect;

pub(crate) type BoxFuture<T, E> =
std::pin::Pin<Box<dyn std::future::Future<Output=Result<T, E>> + Send + 'static>>;

pub(crate) type Request = http::Request<BoxBody>;
pub(crate) type Response = http::Response<hyper::Body>;


#[derive(Debug, Clone)]
pub enum MemtableError {
    Io(std::rc::Rc<io::Error>),
    FromUTF8(std::rc::Rc<std::string::FromUtf8Error>),
    NotFound,
}

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct Connection {
    pub(crate) inner: BoxService<Request, Response, Error>,
}

impl Connection {
    fn new<C>(connector: C, endpoint: Endpoint, is_lazy: bool) -> Self
        where
            C: Service<Uri> + Send + 'static,
            C::Error: Into<Error> + Send,
            C::Future: Unpin + Send,
            C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        let settings = Builder::new()
            .http2_only(false)
            .executor(endpoint.executor.clone())
            .clone();

        let stack = ServiceBuilder::new()
            // .layer_fn(|s| UserAgent::new(s, endpoint.user_agent.clone()))
            .option_layer(endpoint.timeout.map(TimeoutLayer::new))
            .option_layer(endpoint.concurrency_limit.map(ConcurrencyLimitLayer::new))
            .option_layer(endpoint.rate_limit.map(|(l, d)| RateLimitLayer::new(l, d)))
            .into_inner();

        let connector = HyperConnect::new(connector, settings);

        let conn = Reconnect::new(connector, endpoint.uri.clone(), is_lazy);

        let inner = stack.layer(conn);

        Self {
            inner: BoxService::new(inner),
        }
    }

    pub(crate) async fn connect<C>(connector: C, endpoint: Endpoint) -> Result<Self, Error>
        where
            C: Service<Uri> + Send + 'static,
            C::Error: Into<Error> + Send,
            C::Future: Unpin + Send,
            C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        Self::new(connector, endpoint, false).ready_oneshot().await
    }

    pub(crate) fn lazy<C>(connector: C, endpoint: Endpoint) -> Self
        where
            C: Service<Uri> + Send + 'static,
            C::Error: Into<Error> + Send,
            C::Future: Unpin + Send,
            C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        Self::new(connector, endpoint, true)
    }
}

impl Service<Request> for Connection {
    type Response = Response;
    type Error = Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Service::poll_ready(&mut self.inner, cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.inner.call(req)
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
