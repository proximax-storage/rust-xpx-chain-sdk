#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::mosaic::MosaicId;
use xpx_chain_sdk::sirius_client::SiriusClient;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

#[tokio::main]
async fn main() {

    let client = SiriusClient::new(NODE_URL, Client::new());

    let mosaic_one = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();

    let mosaic_info = client.clone().mosaic.get_mosaic_info(
        mosaic_one).await;

    match mosaic_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => panic!("{}", err),
    }

    let mosaic_two = MosaicId::from_hex("13bfc518e40549d7").unwrap();
    let mosaic_three = MosaicId::from_hex("6208AE4D56451357").unwrap();

    let mosaics_info = client.clone().mosaic.get_mosaics_info(
        vec![mosaic_two.clone(), mosaic_three.clone()]).await;

    match mosaics_info {
        Ok(mosaics) => {
            mosaics.iter().for_each(|mosaic_info|{
                println!("{}", mosaic_info)
            })
        }
        Err(err) => panic!("{}", err),
    }

    let mosaics_names = client.clone().mosaic.get_mosaics_names(
        vec![mosaic_two, mosaic_three]).await;

    match mosaics_names {
        Ok(mosaic) => {
            mosaic.iter().for_each(|name|{
                println!("{}", name)
            })
        }
        Err(err) => panic!("{}", err),
    }
}