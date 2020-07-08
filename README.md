<p align="center"><a href="https://www.rust-lang.org" target="_blank" rel="noopener noreferrer"><img width="300" src="https://user-images.githubusercontent.com/29048783/72949565-a56bab00-3d56-11ea-90e8-c02966600775.png" alt="Rust logo"></a></p>

## Official ProximaX Sirius Blockchain SDK Library in Rust.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The ProximaX Sirius Catapult Chain Rust SDK works as a lightweight Rust library for interacting with the Sirius Blockchain. It provides a complete library set coverage, and supports synchronous and asynchronous requests. 

### Usage
First, add this to your `Cargo.toml`:

```toml
[dependencies]
xpx-chain-sdk = { git = "https://github.com/proximax-storage/rust-xpx-chain-sdk"}
```

### Example
```rust
#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = xpx_chain_sdk::api::SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    //let account_id: &str = "VC6LFNKEQQEI5DOAA2OJLL4XRPDNPLRJDH6T2B7X";
    let account_id: &str = "5649D09FB884424AB5E3ED16B965CF69E3048A5E641287C319AC3DE995C97FB0";

    let account_info = client.account_api().account_info(account_id).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
```

For more examples see [wiki](https://github.com/proximax-storage/rust-xpx-chain-sdk/wiki).
