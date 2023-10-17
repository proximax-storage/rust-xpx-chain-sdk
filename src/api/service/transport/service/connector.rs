/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper::Uri;
use tokio_rustls::rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
use tokio_rustls::TlsConnector as RustlsConnector;
use tower::make::MakeConnection;
use tower::Service;

use super::io::BoxedIo;
use super::super::BoxFuture;

pub(crate) fn connector(inner: hyper::client::HttpConnector) -> Connector {
    Connector::new(inner)
}

#[derive(Clone)]
pub struct Connector {
    inner: hyper::client::HttpConnector,
}

impl Connector {
    pub(crate) fn new(inner: hyper::client::HttpConnector) -> Self {
        Self { inner }
    }

    fn tls_or_default(
        &self,
        scheme: Option<&str>,
        host: Option<&str>,
    ) -> (String, Option<RustlsConnector>) {
        let host = match (scheme, host) {
            (Some("https"), Some(host)) => host,
            _ => return (String::new(), None),
        };

        let mut root_cert_store = RootCertStore::empty();
        root_cert_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(
            |ta| {
                OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            },
        ));

        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        (host.to_string(), Some(RustlsConnector::from(Arc::new(config))))
    }
}

impl Service<Uri> for Connector {
    type Response = BoxedIo;
    type Error = crate::api::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        MakeConnection::poll_ready(&mut self.inner, cx).map_err(Into::into)
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        let (domain, tls) = self.tls_or_default(uri.scheme_str(), uri.host());

        let is_https = uri.scheme_str() == Some("https");
        let connect = self.inner.make_connection(uri);

        Box::pin(async move {
            let io = connect.await?;

            {
                if let Some(tls) = tls {
                    let conn = BoxedIo::new(tls.connect(domain.as_str().try_into()?, io).await?);
                    return Ok(BoxedIo::new(conn));
                } else if is_https {
                    return Err(HttpsUriWithoutTlsSupport(()).into());
                }
            }
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
