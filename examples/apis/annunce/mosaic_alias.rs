#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::alias::AliasActionType;
use xpx_chain_sdk::mosaic::MosaicId;
use xpx_chain_sdk::namespace::NamespaceId;
use xpx_chain_sdk::transaction::{Deadline, MosaicAliasTransaction};

const PRIVATE_KEY: &str = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let generation_hash = client.generation_hash();

    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();

    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let mosaic_id = MosaicId::from_hex("7622B206326972CF").unwrap();

    let namespace_id = NamespaceId::from_name("rust").unwrap();

    let alias_transaction = MosaicAliasTransaction::new(
        deadline,
        mosaic_id,
        namespace_id,
        AliasActionType::AliasLink,
        network_type,
    );

    if let Err(err) = &alias_transaction {
        panic!("{}", err)
    }

    let sig_transaction = account.sign(alias_transaction.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_tx.get_hash());

    let response = client
        .transaction_api()
        .announce(&sig_tx)
        .await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
