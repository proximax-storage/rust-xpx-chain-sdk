use crate::aes::Aes256;

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

use rand::RngCore;
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};
use std::string::{ToString, String};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Debug)]
pub struct Ecdhe {
    pub g: [u8;32],
    csprng: OsRng
}

impl Ecdhe {
    pub fn from(g: [u8;32]) -> Ecdhe {
        Ecdhe{g, csprng: OsRng::new().unwrap()}
    }

    fn xor_with_g(&self, other: &[u8]) -> [u8;32] {
        assert_eq!(other.len(), 32);
        let mut r = [0u8;32];
        for (i, (a, b)) in self.g.iter().zip(other.iter()).enumerate() {
            r[i] = a ^ b;
        }
        r
    }

    pub fn encrypt(&mut self, msg: &[u8]) -> std::vec::Vec<u8> {
        let mut salt = [0u8;32];
        self.csprng.fill_bytes(&mut salt);
        let key = self.xor_with_g(&salt);
        let key = Keccak256::digest(&key);

        let mut iv= [0u8;16];
        self.csprng.fill_bytes(&mut iv);

        let cipher = Aes256Cbc::new_var(&key, &iv).unwrap();
        let body = cipher.encrypt_vec(msg);
        let mut output = vec![];
        output.extend_from_slice(&salt);
        output.extend_from_slice(&iv);
        output.extend_from_slice(&body);
        output
    }

    pub fn decrypt(&self, enc_msg: &[u8]) -> Result<std::vec::Vec<u8>, String> {
        if enc_msg.len() < 48 {
            return Err(String::from("Too short encrypt message"))
        }
        let salt = &enc_msg[..32];
        let iv = &enc_msg[32..48];
        let body = &enc_msg[48..];

        let key = self.xor_with_g(salt);
        let key = Keccak256::digest(&key);

        let cipher = Aes256Cbc::new_var(&key, &iv).unwrap();
        match cipher.decrypt_vec(body) {
            Ok(msg ) => Ok(msg),
            Err(err) => Err(err.to_string())
        }
    }
}
