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

## Example
```rust
#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = xpx_chain_sdk::api::SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    const PUBLIC_KEY: &str = "c8f52a6ed98c5bcd52e090da0d1950d58b13d239e4cecc05f5d4acd706f5da75";

    let account_info = client.account_api().account_info(PUBLIC_KEY).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
```
