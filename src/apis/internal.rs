use std::any::Any;

use failure::_core::fmt::Debug;

use crate::models::utils::is_hex;

type Result<T> = ::std::result::Result<T, failure::Error>;

pub(super) fn valid_hash(hash: &str) -> Result<bool> {
    ensure!(
        !hash.is_empty(),
        "transaction_hashes is empty."
    );

    ensure!(
        is_hex(hash),
        "hash {} it's not hex.", hash
    );

    ensure!(hash.len() == 64, "hash {} invalid len.", hash);

    Ok(true)
}

pub(super) fn valid_vec_len<T>(vector: &Vec<T>) -> Result<bool> where T: Debug
{
    ensure!(
        !vector.is_empty(),
        "vector {:?} is empty.", vector
    );
    Ok(true)
}

pub(super) fn valid_vec_hash(vector: &Vec<&str>) -> Result<bool> {
    for hash in vector {
        valid_hash(hash)?;
    }
    Ok(true)
}
