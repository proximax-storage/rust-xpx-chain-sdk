use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{Request, Response, Uri};
use http::uri::InvalidUri;
use hyper::client::connect::Connection as HyperConnection;
use hyper::rt::Executor;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc::{channel, Sender};
use tower::{
    BoxError,
    buffer::Buffer,
    discover::{Change, Discover},
    Service, util::{BoxService, Either},
};
use tower::balance::p2c::Balance;
use tower::buffer::future::ResponseFuture;
use tower::limit::ConcurrencyLimit;
use tower::load::{CompleteOnResponse, PeakEwmaDiscover};

use crate::api::connection;
use crate::api::routes::account_routes_api::AccountRoutes;
use crate::api::routes::chain_routes_api::ChainRoutes;
// use crate::api::routes::exchange_routes_api::ExchangeRoutes;
use crate::api::routes::mosaic_routes_api::MosaicRoutes;
// use crate::api::routes::namespace_routes_api::NamespaceRoutes;
use crate::api::routes::network_routes_api::NetworkRoutes;
use crate::api::routes::node_routes_api::NodeRoutes;
// use crate::api::routes::resolver_routes_api::ResolverRoutes;
use crate::api::routes::transaction_routes_api::TransactionRoutes;
use crate::blockchain::BlockInfo;

use super::body::BoxBody;
use super::connection::{Connection, Error};
use super::discover::DynamicServiceStream;
use super::executor::SharedExec;
use super::node::Endpoint;

// use crate::network::NetworkType;
// use crate::TransactionHash;

type Svc = Either<Connection, BoxService<Request<BoxBody>, Response<hyper::Body>, Error>>;
// type Svc = BoxService<Request<BoxBody>, Response<hyper::Body>, Error>;

const DEFAULT_BUFFER_SIZE: usize = 1024;

const EWMA_DEFAULT_RTT: Duration = Duration::from_millis(30);
const EWMA_DECAY: Duration = Duration::from_secs(1);

// /// A default batteries included `transport` channel.
// ///
// /// This provides a fully featured http2 gRPC client based on [`hyper::Client`]
// /// and `tower` services.
// ///
// /// # Multiplexing requests
// ///
// /// Sending a request on a channel requires a `&mut self` and thus can only send
// /// one request in flight. This is intentional and is required to follow the `Service`
// /// contract from the `tower` library which this channel implementation is built on
// /// top of.
// ///
// /// `tower` itself has a concept of `poll_ready` which is the main mechanism to apply
// /// back pressure. `poll_ready` takes a `&mut self` and when it returns `Poll::Ready`
// /// we know the `Service` is able to accept only one request before we must `poll_ready`
// /// again. Due to this fact any `async fn` that wants to poll for readiness and submit
// /// the request must have a `&mut self` reference.
// ///
// /// To work around this and to ease the use of the channel, `Channel` provides a
// /// `Clone` implementation that is _cheap_. This is because at the very top level
// /// the channel is backed by a `tower_buffer::Buffer` which runs the connection
// /// in a background task and provides a `mpsc` channel interface. Due to this
// /// cloning the `Channel` type is cheap and encouraged.
#[derive(Clone)]
pub struct Client {
    pub svc: Buffer<Svc, Request<BoxBody>>,
}

// /// A future that resolves to an HTTP response.
// ///
// /// This is returned by the `Service::call` on [`Channel`].
// pub struct ResponseFuture {
//     inner: buffer::future::ResponseFuture<<Svc as Service<Request<BoxBody>>>::Future>,
// }

impl Client {
    /// Create an [`Endpoint`] builder that can create [`Channel`]s.
    pub fn builder(uri: Uri) -> Endpoint {
        Endpoint::from(uri)
    }

    /// Create an `Endpoint` from a static string.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Client;
    /// Client::from_static("https://example.com");
    /// ```
    pub fn from_static(s: &'static str) -> Endpoint {
        let uri = Uri::from_static(s);
        Self::builder(uri)
    }

    /// Create an `Endpoint` from shared bytes.
    ///
    /// ```
    /// # use xpx_chain_sdk::api::Client;
    /// Client::from_shared("https://example.com");
    /// ```
    pub fn from_shared(s: impl Into<Bytes>) -> Result<Endpoint, InvalidUri> {
        let uri = Uri::from_maybe_shared(s.into())?;
        Ok(Self::builder(uri))
    }

    /// Balance a list of [`Endpoint`]'s.
    ///
    /// This creates a [`Channel`] that will load balance across all the
    /// provided endpoints.
    pub fn balance_list(list: impl Iterator<Item=Endpoint>) -> Self {
        let (channel, tx) = Self::balance_channel(DEFAULT_BUFFER_SIZE);
        list.for_each(|endpoint| {
            tx.try_send(Change::Insert(endpoint.uri.clone(), endpoint))
                .unwrap();
        });

        channel
    }

    /// Balance a list of [`Endpoint`]'s.
    ///
    /// This creates a [`Channel`] that will listen to a stream of change events and will add or remove provided endpoints.
    pub fn balance_channel<K>(capacity: usize) -> (Self, Sender<Change<K, Endpoint>>)
        where
            K: Hash + Eq + Send + Clone + 'static,
    {
        Self::balance_channel_with_executor(capacity, SharedExec::tokio())
    }

    /// Balance a list of [`Endpoint`]'s.
    ///
    /// This creates a [`Channel`] that will listen to a stream of change events and will add or remove provided endpoints.
    ///
    /// The [`Channel`] will use the given executor to spawn async tasks.
    pub fn balance_channel_with_executor<K, E>(
        capacity: usize,
        executor: E,
    ) -> (Self, Sender<Change<K, Endpoint>>)
        where
            K: Hash + Eq + Send + Clone + 'static,
            E: Executor<Pin<Box<dyn Future<Output=()> + Send>>> + Send + Sync + 'static,
    {
        let (tx, rx) = channel(capacity);
        let list = DynamicServiceStream::new(rx);
        (Self::balance(list, DEFAULT_BUFFER_SIZE, executor), tx)
    }
    pub(crate) fn balance<D, E>(discover: D, buffer_size: usize, executor: E) -> Self
        where
            D: Discover<Service=Connection> + Unpin + Send + 'static,
            D::Error: Into<Error>,
            D::Key: Hash + Send + Clone,
            E: Executor<BoxFuture<'static, ()>> + Send + Sync + 'static,
    {
        let discover = PeakEwmaDiscover::new(
            discover,
            EWMA_DEFAULT_RTT,
            EWMA_DECAY,
            CompleteOnResponse::default(),
        );

        let svc = Balance::new(discover);

        let svc = ConcurrencyLimit::new(svc, 30);

        let svc = BoxService::new(svc);
        let (svc, worker) = Buffer::pair(Either::B(svc), buffer_size);
        executor.execute(Box::pin(worker));

        Client { svc }
    }

    // pub(crate) fn new<C>(connector: C, endpoint: Endpoint) -> Self
    //     where
    //         C: Service<Uri> + Send + 'static,
    //         C::Error: Into<Error> + Send,
    //         C::Future: Unpin + Send,
    //         C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    // {
    //     let buffer_size = endpoint.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);
    //     let executor = endpoint.executor.clone();
    //
    //     let svc = Connection::lazy(connector, endpoint);
    //     let (svc, worker) = Buffer::pair(Either::A(svc), buffer_size);
    //     executor.execute(Box::pin(worker));
    //
    //     Channel { svc }
    // }

    pub(crate) async fn connect<C>(connector: C, endpoint: Endpoint) -> Result<Self, Error>
        where
            C: Service<Uri> + Send + 'static,
            C::Error: Into<Error> + Send,
            C::Future: Unpin + Send,
            C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
    {
        let buffer_size = endpoint.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);
        let executor = endpoint.executor.clone();

        let svc = Connection::connect(connector, endpoint).await?;
        let (svc, worker) = Buffer::pair(Either::A(svc), buffer_size);
        executor.execute(Box::pin(worker));
        Ok(Client { svc })
    }
}

impl Client {
    fn __client(&self) -> Buffer<Svc, Request<BoxBody>> {
        self.svc.clone()
    }

    /// Get Account ApiClient routes.
    pub fn account_api(&self) -> Arc<AccountRoutes> {
        Arc::new(AccountRoutes::new(self.__client()))
    }

    /// Get Chain ApiClient routes.
    pub fn chain_api(&self) -> Arc<ChainRoutes> {
        Arc::new(ChainRoutes::new(self.__client()))
    }

    // /// Get Exchange ApiClient routes.
    // pub fn exchange_api(&self) -> Arc<ExchangeRoutes> {
    //     Arc::new(ExchangeRoutes::new(
    //         self.__client(),
    //         self.network_type().clone(),
    //         self.resolver_api().clone(),
    //     ))
    // }

    /// Get Node ApiClient routes.
    pub fn node_api(&self) -> Arc<NodeRoutes> {
        Arc::new(NodeRoutes::new(self.__client()))
    }

    /// Get Network ApiClient routes.
    pub fn network_api(&self) -> Arc<NetworkRoutes> {
        Arc::new(NetworkRoutes::new(self.__client()))
    }

    /// Get Mosaic ApiClient routes.
    pub fn mosaic_api(&self) -> Arc<MosaicRoutes> {
        Arc::new(MosaicRoutes::new(self.__client()))
    }

    // /// Get Namespace ApiClient routes.
    // pub fn namespace_api(&self) -> Arc<NamespaceRoutes> {
    //     Arc::new(NamespaceRoutes::new(self.__client(), self.network_type()))
    // }

    /// Get Transaction ApiClient routes.
    pub fn transaction_api(&self) -> Arc<TransactionRoutes> {
        Arc::new(TransactionRoutes::new(self.__client()))
    }

    // /// Get Resolver ApiClient routes.
    // pub fn resolver_api(&self) -> Arc<ResolverRoutes> {
    //     Arc::new(ResolverRoutes::new(
    //         self.__client(),
    //         (*self.namespace_api()).clone(),
    //         (*self.mosaic_api()).clone(),
    //     ))
    // }

    // /// Get Metadata ApiClient routes.
    // pub fn metadata_api(&self) -> Box<MetadataRoutes> {
    //     Box::new(MetadataRoutes::new(self.__client()))
    // }

    pub async fn network_info(&self) -> crate::api::error::Result<BlockInfo> {
        self.chain_api().get_block_by_height(1).await
    }
}

impl Service<Request<BoxBody>> for Client {
    type Response = Response<hyper::Body>;
    type Error = BoxError;
    // type Future = ();
    type Future = ResponseFuture<
        Either<
            connection::BoxFuture<Self::Response, Error>,
            connection::BoxFuture<Self::Response, Error>,
        >,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Service::poll_ready(&mut self.svc, cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        self.svc.call(req)
    }
}
