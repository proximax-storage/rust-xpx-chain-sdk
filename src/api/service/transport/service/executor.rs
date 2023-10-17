/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::{future::Future, sync::Arc};

use futures_util::future::BoxFuture;
pub(crate) use hyper::rt::Executor;
use tracing::Instrument;

#[derive(Copy, Clone)]
struct TokioExec;

impl<F> Executor<F> for TokioExec
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{
    #[inline]
    fn execute(&self, fut: F) {
        tokio::spawn(fut.in_current_span());
    }
}

#[derive(Clone)]
pub struct SharedExec {
    inner: Arc<dyn Executor<BoxFuture<'static, ()>> + Send + Sync + 'static>,
}

impl SharedExec {
    pub(crate) fn new<E>(exec: E) -> Self
        where
            E: Executor<BoxFuture<'static, ()>> + Send + Sync + 'static,
    {
        Self { inner: Arc::new(exec) }
    }

    pub(crate) fn tokio() -> Self {
        Self::new(TokioExec)
    }
}

impl Executor<BoxFuture<'static, ()>> for SharedExec {
    fn execute(&self, fut: BoxFuture<'static, ()>) {
        self.inner.execute(fut)
    }
}
