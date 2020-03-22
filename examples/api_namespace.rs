#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::account::Address;
use xpx_chain_sdk::namespace::NamespaceId;
use xpx_chain_sdk::sirius_client::SiriusClient;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

#[tokio::main]
async fn main() {
    let client = SiriusClient::new(NODE_URL, Client::new());

    let address = Address::from_raw("VCVF646H3M3C5CNIVWFZ734NC2WQXWYUKBGIZAB5").unwrap();

    let namespace_one = NamespaceId::from_name("rust.latam.colombia").unwrap();

    let namespace_two = NamespaceId::from("BFFB42A19116BDF6");

    let namespace_info = client.clone().namespace.get_namespace_info(namespace_one).await;
    match namespace_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    let from_account = client.clone().namespace.get_namespaces_from_account(address, None, None).await;
    match from_account {
        Ok(namespaces) => {
            namespaces.iter().for_each(|namespace_info| {
                println!("{}", namespace_info)
            })
        }
        Err(err) => eprintln!("{}", err)
    }

    let namespaces_names = client.namespace.get_namespaces_names(vec![namespace_one, namespace_two]).await;
    match namespaces_names {
        Ok(namespaces) => {
            namespaces.iter().for_each(|namespace_name| {
                println!("{}", namespace_name)
            })
        }
        Err(err) => eprintln!("{}", err),
    }
}