use crate::api::transport::service::connection::{Request, Response};
use futures_util::future;
use tower::retry::Policy;

/// RetryPolicy represents a custom retry policy for requests.
///
/// It implements the `tower::retry::Policy` trait, which defines the behavior
/// of retries based on the results of previous requests.
/// The internal parameter usize in the RetryPolicy struct represents the maximum number of retries allowed for a request.
/// It specifies how many times the request can be attempted in case of errors before the retry process is considered complete.
/// When creating an instance of RetryPolicy, a usize value should be provided, indicating the number of retries allowed.
/// This value is stored in the RetryPolicy struct and is used in the implementation of the retry method.
/// In the retry method, the field self.0 is used to check if there are still retries available before attempting another retry.
/// If the value of self.0 is greater than zero, it is decremented by one to indicate that an attempt has been made,
/// and a new instance of RetryPolicy with the decremented value is returned. This allows keeping track of the remaining number of retries.
/// If self.0 is zero, indicating that no retries are available, no further retries will be attempted,
/// and None is returned to indicate that no more retries should be performed.
/// In summary, the internal usize parameter in RetryPolicy determines the maximum number of retries allowed and
/// is used to control the retry logic based on the remaining retry attempts.
#[derive(Debug, Clone)]
pub struct RetryPolicy(pub usize);

impl Policy<Request, Response, hyper::Error> for RetryPolicy {
    type Future = future::Ready<Self>;

    /// Determines whether a retry should be performed based on the result of the previous request.
    ///
    /// This method is called for each request to decide whether to retry or not.
    ///
    /// # Arguments
    ///
    /// * `req`: The original request that was made.
    /// * `result`: The result of the previous request. It is a `Result` containing either
    ///             a reference to the successful response or a reference to the error that occurred.
    ///
    /// # Returns
    ///
    /// * `None`: If the result is successful (`Ok`), indicating that no retry should be performed.
    /// * `Some`: If the result is an error (`Err`), indicating that a retry may be performed.
    ///           It contains a future that resolves to a new instance of `RetryPolicy` with a
    ///           decremented retry count, indicating the intention to retry.
    fn retry(&self, _: &Request, result: Result<&Response, &hyper::Error>) -> Option<Self::Future> {
        match result {
            Ok(_) => {
                // Treat all `Response`s as success,
                // so don't retry...
                None
            }
            Err(err) => {
                tracing::debug!("error: {}", err);

                // if err.is_connect() {
                //     // If it's a connection error, don't retry.
                //     return None;
                // }

                // Treat all other errors as failures...
                // But we limit the number of attempts...
                if self.0 > 0 {
                    // Try again!
                    Some(future::ready(RetryPolicy(self.0 - 1)))
                } else {
                    // Used all our attempts, no retry...
                    None
                }
            }
        }
    }

    /// Clones the original request for a retry.
    ///
    /// This method is called when a retry is about to be performed.
    ///
    /// # Arguments
    ///
    /// * `req`: The original request that was made.
    ///
    /// # Returns
    ///
    /// * `Some`: A cloned instance of the original request, indicating that a retry should be performed.
    /// * `None`: If cloning the request is not possible or not needed, indicating that no retry should be performed.
    fn clone_request(&self, req: &Request) -> Option<Request> {
        let mut clone = hyper::Request::new(req.body().clone());

        // Copy relevant fields from the original request.
        *clone.method_mut() = req.method().clone();
        *clone.uri_mut() = req.uri().clone();
        *clone.headers_mut() = req.headers().clone();
        *clone.version_mut() = req.version();

        Some(clone)
    }
}
