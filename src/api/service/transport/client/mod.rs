//! Client implementation and builder.

use std::{
    fmt,
    future::Future,
    hash::Hash,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use http::{
    uri::{InvalidUri, Uri},
    Request, Response,
};
use hyper::client::connect::Connection as HyperConnection;
use hyper::rt::Executor;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::mpsc::{channel, Sender},
};
use tower::balance::p2c::Balance;
use tower::{
    buffer::{self, Buffer},
    discover::{Change, Discover},
    util::{BoxService, Either},
    Service,
};

pub use api_node::ApiNode;

use crate::api::body::BoxBody;
use crate::api::routes::{
    account_routes_api::AccountRoutes, chain_routes_api::ChainRoutes,
    mosaic_routes_api::MosaicRoutes, namespace_routes_api::NamespaceRoutes,
    network_routes_api::NetworkRoutes, node_routes_api::NodeRoutes,
    resolver_routes_api::ResolverRoutes, transaction_routes_api::TransactionRoutes,
};

use super::service::{Connection, DynamicServiceStream, SharedExec};

mod api_node;

pub type Svc =
    Either<Connection, BoxService<Request<BoxBody>, Response<hyper::Body>, crate::api::Error>>;

const DEFAULT_BUFFER_SIZE: usize = 1024;

/// A default batteries included `transport` client.
///
/// This provides a fully featured http2 gRPC client based on [`hyper::Client`]
/// and `tower` services.
///
/// # Multiplexing requests
///
/// Sending a request on a client requires a `&mut self` and thus can only send
/// one request in flight. This is intentional and is required to follow the `Service`
/// contract from the `tower` library which this client implementation is built on
/// top of.
///
/// `tower` itself has a concept of `poll_ready` which is the main mechanism to apply
/// back pressure. `poll_ready` takes a `&mut self` and when it returns `Poll::Ready`
/// we know the `Service` is able to accept only one request before we must `poll_ready`
/// again. Due to this fact any `async fn` that wants to poll for readiness and submit
/// the request must have a `&mut self` reference.
///
/// To work around this and to ease the use of the client, `Channel` provides a
/// `Clone` implementation that is _cheap_. This is because at the very top level
/// the client is backed by a `tower_buffer::Buffer` which runs the connection
/// in a background task and provides a `mpsc` client interface. Due to this
/// cloning the `Channel` type is cheap and encouraged.
#[derive(Clone)]
pub struct Client {
    pub svc: Buffer<Svc, Request<BoxBody>>,
}

/// A future that resolves to an HTTP response.
///
/// This is returned by the `Service::call` on [`Channel`].
pub struct ResponseFuture {
    inner: buffer::future::ResponseFuture<<Svc as Service<Request<BoxBody>>>::Future>,
}

impl Client {
    /// Create an [`ApiNode`] builder that can create [`Channel`]s.
    pub fn builder(uri: Uri) -> ApiNode {
        ApiNode::from(uri)
    }

    /// Create an `Endpoint` from a static string.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Client;
    /// Client::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    /// ```
    pub fn from_static(s: &'static str) -> ApiNode {
        let uri = Uri::from_static(s);
        Self::builder(uri)
    }

    /// Create an `Endpoint` from shared bytes.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Client;
    /// Client::from_shared("http://api-1.testnet2.xpxsirius.io:3000");
    /// ```
    pub fn from_shared(s: impl Into<Bytes>) -> Result<ApiNode, InvalidUri> {
        let uri = Uri::from_maybe_shared(s.into())?;
        Ok(Self::builder(uri))
    }

    /// Balance a list of [`ApiNode`]'s.
    ///
    /// This creates a [`Channel`] that will load balance across all the
    /// provided endpoints.
    pub fn balance_list(list: impl Iterator<Item = ApiNode>) -> Self {
        let (client, tx) = Self::balance_channel(DEFAULT_BUFFER_SIZE);
        list.for_each(|endpoint| {
            tx.try_send(Change::Insert(endpoint.uri.clone(), endpoint))
                .unwrap();
        });

        client
    }

    /// Balance a list of [`ApiNode`]'s.
    ///
    /// This creates a [`Channel`] that will listen to a stream of change events and will add or remove provided endpoints.
    pub fn balance_channel<K>(capacity: usize) -> (Self, Sender<Change<K, ApiNode>>)
    where
        K: Hash + Eq + Send + Clone + 'static,
    {
        Self::balance_channel_with_executor(capacity, SharedExec::tokio())
    }

    /// Balance a list of [`ApiNode`]'s.
    ///
    /// This creates a [`Channel`] that will listen to a stream of change events and will add or remove provided endpoints.
    ///
    /// The [`Channel`] will use the given executor to spawn async tasks.
    pub fn balance_channel_with_executor<K, E>(
        capacity: usize,
        executor: E,
    ) -> (Self, Sender<Change<K, ApiNode>>)
    where
        K: Hash + Eq + Send + Clone + 'static,
        E: Executor<Pin<Box<dyn Future<Output = ()> + Send>>> + Send + Sync + 'static,
    {
        let (tx, rx) = channel(capacity);
        let list = DynamicServiceStream::new(rx);
        (Self::balance(list, DEFAULT_BUFFER_SIZE, executor), tx)
    }

    pub(crate) fn new<C>(connector: C, endpoint: ApiNode) -> Self
    where
        C: Service<Uri> + Send + 'static,
        C::Error: Into<crate::api::Error> + Send,
        C::Future: Unpin + Send,
        C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        let buffer_size = endpoint.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);
        let executor = endpoint.executor.clone();

        let svc = Connection::lazy(connector, endpoint);
        let (svc, worker) = Buffer::pair(Either::A(svc), buffer_size);
        executor.execute(Box::pin(worker));

        Client { svc }
    }

    pub(crate) async fn connect<C>(connector: C, endpoint: ApiNode) -> Result<Self, super::Error>
    where
        C: Service<Uri> + Send + 'static,
        C::Error: Into<crate::api::Error> + Send,
        C::Future: Unpin + Send,
        C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        let buffer_size = endpoint.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);
        let executor = endpoint.executor.clone();

        let svc = Connection::connect(connector, endpoint)
            .await
            .map_err(super::Error::from_source)?;
        let (svc, worker) = Buffer::pair(Either::A(svc), buffer_size);
        executor.execute(Box::pin(worker));

        Ok(Client { svc })
    }

    pub(crate) fn balance<D, E>(discover: D, buffer_size: usize, executor: E) -> Self
    where
        D: Discover<Service = Connection> + Unpin + Send + 'static,
        D::Error: Into<crate::api::Error>,
        D::Key: Hash + Send + Clone,
        E: Executor<futures_core::future::BoxFuture<'static, ()>> + Send + Sync + 'static,
    {
        let svc = Balance::new(discover);

        let svc = BoxService::new(svc);
        let (svc, worker) = Buffer::pair(Either::B(svc), buffer_size);
        executor.execute(Box::pin(worker));

        Client { svc }
    }
}

impl Service<http::Request<BoxBody>> for Client {
    type Response = http::Response<super::Body>;
    type Error = super::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Service::poll_ready(&mut self.svc, cx).map_err(super::Error::from_source)
    }

    fn call(&mut self, request: http::Request<BoxBody>) -> Self::Future {
        let inner = Service::call(&mut self.svc, request);

        ResponseFuture { inner }
    }
}

impl Future for ResponseFuture {
    type Output = Result<Response<hyper::Body>, super::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let val = futures_util::ready!(Pin::new(&mut self.inner).poll(cx))
            .map_err(super::Error::from_source)?;
        Ok(val).into()
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Channel").finish()
    }
}

impl fmt::Debug for ResponseFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResponseFuture").finish()
    }
}

impl Client {
    fn __client(&self) -> Client {
        self.clone()
    }

    /// Get Account ApiClient routes.
    pub fn account_api(&self) -> Box<AccountRoutes> {
        Box::new(AccountRoutes::new(self.__client()))
    }

    /// Get Chain ApiClient routes.
    pub fn chain_api(&self) -> Box<ChainRoutes> {
        Box::new(ChainRoutes::new(self.__client()))
    }

    // /// Get Exchange ApiClient routes.
    // pub fn exchange_api(&self) -> Box<ExchangeRoutes> {
    //     Box::new(ExchangeRoutes::new(
    //         self.__client(),
    //         self.network_type(),
    //         *self.resolver_api(),
    //     ))
    // }

    /// Get Node ApiClient routes.
    pub fn node_api(&self) -> Box<NodeRoutes> {
        Box::new(NodeRoutes::new(self.__client()))
    }

    /// Get Network ApiClient routes.
    pub fn network_api(&self) -> Box<NetworkRoutes> {
        Box::new(NetworkRoutes::new(self.__client()))
    }

    /// Get Mosaic ApiClient routes.
    pub fn mosaic_api(&self) -> Box<MosaicRoutes> {
        Box::new(MosaicRoutes::new(self.__client()))
    }

    /// Get Namespace ApiClient routes.
    pub fn namespace_api(&self) -> Box<NamespaceRoutes> {
        Box::new(NamespaceRoutes::new(self.__client()))
    }

    /// Get Transaction ApiClient routes.
    pub fn transaction_api(&self) -> Box<TransactionRoutes> {
        Box::new(TransactionRoutes::new(self.__client()))
    }

    /// Get Resolver ApiClient routes.
    pub fn resolver_api(&self) -> Box<ResolverRoutes> {
        Box::new(ResolverRoutes::new(
            self.__client(),
            *self.namespace_api(),
            *self.mosaic_api(),
        ))
    }

    // /// Get Metadata ApiClient routes.
    // pub fn metadata_api(&self) -> Box<MetadataRoutes> {
    //     Box::new(MetadataRoutes::new(self.__client()))
    // }
}
