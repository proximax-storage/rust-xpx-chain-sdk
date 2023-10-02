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
