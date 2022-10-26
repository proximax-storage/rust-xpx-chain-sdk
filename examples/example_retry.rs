use futures_util::{pin_mut, stream, StreamExt};
use reqwest::{Error, Method, Request, Response};
use std::future::Future;
use std::process::Output;
use std::time::Duration;
use tower::balance::p2c::Balance;
use tower::layer::util::{Identity, Stack};
use tower::limit::{RateLimit, RateLimitLayer};
use tower::util::{BoxService, ServiceFn};
use tower::{BoxError, Service, ServiceBuilder, ServiceExt};

#[tokio::main]
async fn main() {
    let req2 = Request::new(
        Method::GET,
        "http://demo-api-1.edlx.io:3000/node/info"
            .to_string()
            .parse()
            .unwrap(),
    );

    let req3 = Request::new(
        Method::GET,
        "http://demo-api-3.edlx.io:3000/node/info"
            .to_string()
            .parse()
            .unwrap(),
    );

    // let mut svc1: BoxService<Request, Response, BoxError> = ServiceBuilder::new()
    //     .boxed()
    //     .load_shed()
    //     .concurrency_limit(64)
    //     .timeout(Duration::from_secs(10))
    //     .service_fn(handle);

    let mut svc1 = create_service_client()
        .timeout(Duration::from_secs(10))
        .service(tower::service_fn(handle));
    //
    // let svc2 = create_service_client().service(tower::service_fn(move |req| {
    //     reqwest::Client::new().execute(req)
    // }));
    let resp = match svc1.call(req2).await {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let body = resp.bytes().await;

    println!("{:?}", body);

    // let response = service.ready().await?.call(req3).await?;

    // // Spread load evenly across the two services
    // let p2c = Balance::new(tower::discover::ServiceList::new(vec![svc1, svc2]));
    //
    // let reqs = stream::iter(vec![req2, req3]);
    //
    // // Issue all the requests that come in.
    // // Some will go to svc1, some will go to svc2.
    // pin_mut!(reqs);
    // // let mut responses = p2c.call_all(reqs);
    // // while let Some(rsp) = responses.next().await {
    // //     // ...
    // // }

    // do stuff with svc
}

async fn handle(request: Request) -> Result<Response, Error> {
    println!("PPPPPPPPPPPPPPPPPPPPP");
    reqwest::Client::new().execute(request).await
}

fn create_service_client() -> ServiceBuilder<Stack<RateLimitLayer, Identity>> {
    tower::ServiceBuilder::new().rate_limit(1, std::time::Duration::new(10, 0)) // 1 requests every 10 seconds
}

type ClosureFut = Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>>>;
type ServiceClosure = Box<dyn FnMut(Request) -> ClosureFut>;

fn return_service() -> RateLimit<ServiceFn<ServiceClosure>> {
    let client = reqwest::Client::new();
    let dyn_service_fn = Box::new(move |req| Box::new(client.execute(req)) as ClosureFut);
    create_service_client().service(tower::service_fn(dyn_service_fn))
}
