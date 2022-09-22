use xpx_chain_sdk::api::SiriusClient;

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new_from_urls(&node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    //let account_id: &str = "VC6LFNKEQQEI5DOAA2OJLL4XRPDNPLRJDH6T2B7X";
    let account_id: &str = "5649D09FB884424AB5E3ED16B965CF69E3048A5E641287C319AC3DE995C97FB0";

    let account_info = client.account_api().account_info(account_id).await;
    match account_info {
        Ok(resp) => {
            let algo = resp.mosaics[0].asset_id.clone() == resp.mosaics[0].asset_id.clone();

            println!("{}", algo) },
        Err(err) => eprintln!("{}", err),
    }
}
