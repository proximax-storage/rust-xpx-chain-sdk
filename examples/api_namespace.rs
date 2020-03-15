#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::namespace::NamespaceId;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

#[tokio::main]
async fn main() {

    let client = SiriusClient::new(NODE_URL, Client::new());

    let namespace_one = NamespaceId::from_name("prx").unwrap();

    let namespace_info = client.clone().namespace.get_namespace_info(
        namespace_one).await;

    match namespace_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => panic!("{:?}", err),
    }

    let namespace_two = NamespaceId::from("BFFB42A19116BDF6");

    let namespaces_names = client.clone().namespace.get_namespaces_names(
        vec![namespace_one, namespace_two]).await;

    match namespaces_names {
        Ok(resp) => {
            for nemespace in resp {
                println!("{}", nemespace)
            }
        }
        Err(err) => panic!("{:?}", err),
    }
}