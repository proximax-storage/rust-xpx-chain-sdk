use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::Local;
use chrono::prelude::DateTime;
use serde::{Serialize, Serializer};

use crate::models::Uint64;

/// It is Friday, 01 April 2016 00:00:00 +0000 UTC (1459468800000 milliseconds since the epoch time).
static TIMESTAMP_NEMESIS_BLOCK_MILLISECONDS: i64 = 1459468800 * 1000;
static MILLISECONDS: i64 = 1000000;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockchainTimestamp(i64);

impl BlockchainTimestamp {
    pub fn new(milliseconds: i64) -> Self {
        BlockchainTimestamp(milliseconds)
    }

    /// returns new timestamp from passed milliseconds value
    pub fn to_timestamp(&self) -> Timestamp {
        Timestamp::new(self.0 + TIMESTAMP_NEMESIS_BLOCK_MILLISECONDS)
    }

    pub fn to_uint64(&self) -> Uint64 {
        Uint64::new(self.0 as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Timestamp(SystemTime);

impl Timestamp {
    /// returns new timestamp from passed milliseconds value
    pub fn new(milliseconds: i64) -> Self {
        // Creates a new SystemTime from the specified number of whole seconds
        Timestamp(UNIX_EPOCH + Duration::from_nanos((milliseconds * MILLISECONDS) as u64))
    }

    pub fn to_blockchain_timestamp(&self) -> BlockchainTimestamp {
        BlockchainTimestamp((self.0.duration_since(
            UNIX_EPOCH).unwrap().as_nanos() / MILLISECONDS as u128) as i64
            - TIMESTAMP_NEMESIS_BLOCK_MILLISECONDS)
    }
}

impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let datetime = DateTime::<Local>::from(self.0);
        write!(
            f, "{}", datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deadline(Timestamp);

impl Deadline {
    pub fn new(hour: u8, minute: u8, second: u64) -> Self {
        let time_now = SystemTime::now();
        let _hour: u64 = hour as u64 * 3600;
        let _minute: u64 = minute as u64 * 60;

        Deadline(Timestamp(time_now.add(Duration::from_secs(_hour + _minute + second))))
    }

    pub fn to_blockchain_timestamp(&self) -> BlockchainTimestamp {
        self.0.to_blockchain_timestamp()
    }
}

impl core::fmt::Display for Deadline {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}", self.0
        )
    }
}

impl From<BlockchainTimestamp> for Deadline {
    fn from(e: BlockchainTimestamp) -> Self {
        return Deadline(e.to_timestamp());
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

/// Creates `Deadline` with the default parameters.
impl Default for Deadline {
    fn default() -> Self {
        Deadline::new(1, 0, 0)
    }
}
