/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::task::{Context, Poll};

use futures_util::future;
use tower::{ServiceBuilder, ServiceExt};
use tower::limit::ConcurrencyLimit;
// use tower::load::Load;
use tower::retry::Retry;
use tower::Service;

use crate::api::service::service::retry::RetryPolicy;
use crate::api::transport::ApiNode;
use crate::api::transport::service::{Connector, ReplayBody};
use crate::api::transport::service::http_timeout::HttpTimeout;

use super::AddOrigin;

pub type Request = hyper::Request<ReplayBody>;
pub type Response = hyper::Response<hyper::Body>;
pub type TimeoutConnector = hyper_timeout::TimeoutConnector<Connector>;
pub type ConnectionInnerSvc = AddOrigin<
    HttpTimeout<ConcurrencyLimit<Retry<RetryPolicy, hyper::Client<TimeoutConnector, ReplayBody>>>>,
>;

#[derive(Clone)]
pub struct Connection {
    inner: ConnectionInnerSvc,
}

impl Connection {
    fn new(connector: TimeoutConnector, endpoint: ApiNode) -> Self {
        let executor = endpoint.executor.clone();
        let svc = hyper::Client::builder()
            .executor(executor)
            .build::<TimeoutConnector, ReplayBody>(connector);

        let inner = ServiceBuilder::new()
            // .load_shed()
            .layer_fn(|s| AddOrigin::new(s, endpoint.uri.clone()))
            .layer_fn(|s| HttpTimeout::new(s, endpoint.timeout))
            .concurrency_limit(endpoint.concurrency_limit.unwrap_or(100))
            // .option_layer(endpoint.rate_limit.map(|(l, d)| RateLimitLayer::new(l, d)))
            .retry(RetryPolicy::new(
                endpoint.max_num_retry.unwrap_or_default(),
                endpoint.retry_service_unavailable_requests,
            ))
            .service(svc);

        Self { inner }
    }

    pub(crate) async fn connect(
        connector: TimeoutConnector,
        endpoint: ApiNode,
    ) -> Result<Self, crate::api::error::Error> {
        Self::new(connector, endpoint).ready_oneshot().await
    }

    pub(crate) fn lazy(connector: TimeoutConnector, endpoint: ApiNode) -> Self {
        Self::new(connector, endpoint)
    }
}

impl Service<Request> for Connection {
    type Response = Response;
    type Error = crate::api::error::Error;
    type Future = future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Service::poll_ready(&mut self.inner, cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let fut = self.inner.call(request);
        Box::pin(async move { fut.await.map_err(Into::into) })
    }
}
//
// impl Load for Connection {
//     type Metric = usize;
//
//     fn load(&self) -> Self::Metric {
//         0
//     }
// }
