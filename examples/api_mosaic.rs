#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;
use xpx_chain_sdk::models::mosaic::MosaicId;

#[tokio::main]
async fn main() {
    let node = "http://bctestnet3.brimstone.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let mosaic01 = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();

    let mosaic_info = client.clone().mosaic.get_mosaic_info(
        mosaic01).await;

    match mosaic_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => panic!("{:?}", err),
    }

    let mosaic02 = MosaicId::from_hex("13bfc518e40549d7").unwrap();
    let mosaic03 = MosaicId::from_hex("6208AE4D56451357").unwrap();

    let mosaics_info = client.clone().mosaic.get_mosaics_info(
        vec![mosaic02.clone(), mosaic03.clone()]).await;

    match mosaics_info {
        Ok(resp) => {
            for mosaic in resp {
                println!("{}", mosaic)
            }
        }
        Err(err) => panic!("{:?}", err),
    }

    let mosaics_names = client.clone().mosaic.get_mosaics_names(
        vec![mosaic02, mosaic03]).await;

    match mosaics_names {
        Ok(resp) => {
            for mosaic in resp {
                println!("{}", mosaic)
            }
        }
        Err(err) => panic!("{:?}", err),
    }
}