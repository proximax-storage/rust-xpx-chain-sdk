use std::fmt;
use std::task::{Context, Poll};

use http::Uri;
use tower::make::MakeConnection;
use tower_service::Service;

use super::io::BoxedIo;
use super::super::BoxFuture;

pub(crate) fn connector<C>(inner: C) -> Connector<C> {
    Connector { inner }
}

pub(crate) struct Connector<C> {
    inner: C,
}

impl<C> Service<Uri> for Connector<C>
    where
        C: MakeConnection<Uri>,
        C::Connection: Unpin + Send + 'static,
        C::Future: Send + 'static,
        crate::api::Error: From<C::Error> + Send + 'static,
{
    type Response = BoxedIo;
    type Error = crate::api::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        MakeConnection::poll_ready(&mut self.inner, cx).map_err(Into::into)
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        let connect = self.inner.make_connection(uri);

        Box::pin(async move {
            let io = connect.await?;

            Ok(BoxedIo::new(io))
        })
    }
}

/// Error returned when trying to connect to an HTTPS endpoint without TLS enabled.
#[derive(Debug)]
pub(crate) struct HttpsUriWithoutTlsSupport(());

impl fmt::Display for HttpsUriWithoutTlsSupport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connecting to HTTPS without TLS enabled")
    }
}

// std::error::Error only requires a type to impl Debug and Display
impl std::error::Error for HttpsUriWithoutTlsSupport {}
