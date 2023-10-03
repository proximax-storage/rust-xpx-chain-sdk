/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::task::{Context, Poll};

use hyper::http::{header::USER_AGENT, HeaderValue, Request};
use tower::Service;

const SIRIUS_USER_AGENT: &str = concat!("/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub(crate) struct UserAgent<T> {
    inner: T,
    user_agent: HeaderValue,
}

impl<T> UserAgent<T> {
    pub(crate) fn new(inner: T, user_agent: Option<HeaderValue>) -> Self {
        let user_agent = user_agent
            .map(|value| {
                let mut buf = Vec::new();
                buf.extend(value.as_bytes());
                // buf.push(b' ');
                buf.extend(SIRIUS_USER_AGENT.as_bytes());
                HeaderValue::from_bytes(&buf).expect("user-agent should be valid")
            })
            .unwrap_or_else(|| HeaderValue::from_static(SIRIUS_USER_AGENT));

        Self { inner, user_agent }
    }
}

impl<T, ReqBody> Service<Request<ReqBody>> for UserAgent<T>
    where
        T: Service<Request<ReqBody>>,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        req.headers_mut().insert(USER_AGENT, self.user_agent.clone());

        self.inner.call(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Svc;

    #[test]
    fn sets_default_if_no_custom_user_agent() {
        assert_eq!(
            UserAgent::new(Svc, None).user_agent,
            HeaderValue::from_static(SIRIUS_USER_AGENT)
        )
    }

    #[test]
    fn prepends_custom_user_agent_to_default() {
        assert_eq!(
            UserAgent::new(Svc, Some(HeaderValue::from_static("Greeter 1.1"))).user_agent,
            HeaderValue::from_str(&format!("Greeter 1.1{}", SIRIUS_USER_AGENT)).unwrap()
        )
    }
}
