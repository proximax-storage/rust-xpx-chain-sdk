use std::fmt;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project::pin_project;
use tower::make::MakeService;
use tower_service::Service;

use crate::api::Error;

/// Wrapper for a service that automatically reconnects in case of connection errors.
pub(crate) struct Reconnect<M, Target>
where
    M: Service<Target> + Clone,
    M::Error: Into<Error> + Clone,
{
    mk_service: M,
    state: State<M::Future, M::Response>,
    target: Target,
    error: Option<crate::api::Error>,
    has_been_connected: bool,
    is_lazy: bool,
}

/// Possible states of the `Reconnect` service.
#[derive(Debug)]
enum State<F, S> {
    Idle,
    Connecting(F),
    Connected(S),
}

impl<M, Target> Reconnect<M, Target>
where
    M: Service<Target> + Clone,
    M::Error: Into<Error> + Clone,
{
    /// Creates a new `Reconnect` service wrapper.
    pub(crate) fn new(mk_service: M, target: Target, is_lazy: bool) -> Self {
        Reconnect {
            mk_service,
            state: State::Idle,
            target,
            error: None,
            has_been_connected: false,
            is_lazy,
        }
    }
}

impl<M, Target, S, Request> Service<Request> for Reconnect<M, Target>
where
    M: Service<Target, Response = S> + Clone,
    S: Service<Request> + Clone,
    M::Future: Unpin,
    Error: From<M::Error> + From<S::Error>,
    Target: Clone,
    <M as tower_service::Service<Target>>::Error: Into<crate::api::Error> + Clone,
{
    type Response = S::Response;
    type Error = Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut state;

        if self.error.is_some() {
            return Poll::Ready(Ok(()));
        }

        loop {
            match &mut self.state {
                State::Idle => {
                    trace!("poll_ready; idle");
                    match self.mk_service.poll_ready(cx) {
                        Poll::Ready(r) => r?,
                        Poll::Pending => {
                            trace!("poll_ready; MakeService not ready");
                            return Poll::Pending;
                        }
                    }

                    let fut = self.mk_service.make_service(self.target.clone());
                    self.state = State::Connecting(fut);
                    continue;
                }
                State::Connecting(f) => {
                    trace!("poll_ready; connecting");
                    match Pin::new(f).poll(cx) {
                        Poll::Ready(Ok(service)) => {
                            state = State::Connected(service);
                        }
                        Poll::Pending => {
                            trace!("poll_ready; not ready");
                            return Poll::Pending;
                        }
                        Poll::Ready(Err(e)) => {
                            trace!("poll_ready; error");

                            state = State::Idle;

                            if !(self.has_been_connected || self.is_lazy) {
                                return Poll::Ready(Err(e.into()));
                            } else {
                                let error = e.into();
                                debug!("reconnect::poll_ready: {:?}", error);
                                self.error = Some(error);
                                break;
                            }
                        }
                    }
                }
                State::Connected(inner) => {
                    trace!("poll_ready; connected");

                    self.has_been_connected = true;

                    match inner.poll_ready(cx) {
                        Poll::Ready(Ok(())) => {
                            trace!("poll_ready; ready");
                            return Poll::Ready(Ok(()));
                        }
                        Poll::Pending => {
                            trace!("poll_ready; not ready");
                            return Poll::Pending;
                        }
                        Poll::Ready(Err(_)) => {
                            trace!("poll_ready; error");
                            state = State::Idle;
                        }
                    }
                }
            }

            self.state = state;
        }

        self.state = state;
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request) -> Self::Future {
        trace!("Reconnect::call");
        if let Some(error) = self.error.take() {
            debug!("error: {}", error);
            return ResponseFuture::error(error);
        }

        let mut service = match &mut self.state {
            State::Connected(service) => service.clone(),
            _ => panic!("service not ready; poll_readymust be called first"),
        };

        let fut = service.call(request);
        ResponseFuture::new(fut)
    }
}

impl<M, Target> fmt::Debug for Reconnect<M, Target>
where
    M: Service<Target> + fmt::Debug + Clone,
    M::Future: fmt::Debug,
    M::Response: fmt::Debug,
    Target: fmt::Debug,
    <M as tower_service::Service<Target>>::Error: Into<Error> + Clone,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Reconnect")
            .field("mk_service", &self.mk_service)
            .field("state", &self.state)
            .field("target", &self.target)
            .finish()
    }
}

/// Future that resolves to the response or failure to connect.
#[pin_project]
#[derive(Debug)]
pub(crate) struct ResponseFuture<F> {
    #[pin]
    inner: F,
}

impl<F> ResponseFuture<F> {
    pub(crate) fn new(inner: F) -> Self {
        ResponseFuture { inner }
    }

    pub(crate) fn error(error: crate::api::Error) -> Self {
        unimplemented!()
        // ResponseFuture {
        //     inner: futures_util::future::err(error),
        // }
    }
}

impl<F, T, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: Into<Error>,
{
    type Output = Result<T, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx).map_err(Into::into)
    }
}
