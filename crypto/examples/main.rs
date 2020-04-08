extern crate hex;
extern crate xpx_crypto;

use xpx_crypto::PublicKey;
use xpx_crypto::SecretKey;
use xpx_crypto::Keypair;
use xpx_crypto::Signature;

fn main() {
    let sk_hex = hex::decode("68f50e10e5b8be2b7e9ddb687a667d6e94dd55fe02b4aed8195f51f9a242558b").unwrap();

    let message: &[u8] = b"ProximaX Limited";
    let plomo = "ProximaX Limited";


    let secret_key: SecretKey = SecretKey::from_bytes(&sk_hex).unwrap();
    println!("PrivateKey: {:?}", hex::encode(secret_key.to_bytes()));

    let public_key: PublicKey = PublicKey::from(&secret_key);
    println!("PublicKey: \t{:?}", hex::encode(public_key.to_bytes()));

    let key_pair = Keypair{ secret: secret_key, public: public_key };

    println!("PublicKey: \t{:?}", key_pair.public);


    let sig: Signature = key_pair.sign(&message);
    println!("Sig: \t\t{:?}", hex::encode(sig.to_bytes().to_vec()));
    println!("Verify: \t{}", key_pair.verify(&message, &sig).is_ok());
}
