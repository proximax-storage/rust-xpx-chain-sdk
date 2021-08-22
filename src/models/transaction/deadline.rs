/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::{
        ops::Add,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    chrono::{
        {Local, NaiveTime, Timelike},
        prelude::DateTime,
    },
    serde::{Serialize, Serializer},
};

use crate::models::Uint64;

/// It is Friday, 01 April 2016 00:00:00 +0000 UTC (1459468800000 milliseconds since the epoch time).
const TIMESTAMP_NEMESIS_BLOCK_MILLISECONDS: i64 = 1_459_468_800 * 1_000;
const MILLISECONDS: i64 = 1_000_000;

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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Timestamp(SystemTime);

impl Timestamp {
    /// returns new timestamp from passed milliseconds value
    pub fn new(milliseconds: i64) -> Self {
        // Creates a new SystemTime from the specified number of whole seconds
        Timestamp(UNIX_EPOCH + Duration::from_nanos((milliseconds * MILLISECONDS) as u64))
    }

    pub fn to_blockchain_timestamp(&self) -> BlockchainTimestamp {
        BlockchainTimestamp(
            (self.0.duration_since(UNIX_EPOCH).unwrap().as_nanos() / MILLISECONDS as u128) as i64
                - TIMESTAMP_NEMESIS_BLOCK_MILLISECONDS,
        )
    }

    pub fn to_date_time(&self) -> DateTime<Local> {
        DateTime::<Local>::from(self.0)
    }

    pub fn to_hour(&self) -> u32 {
        self.to_date_time().hour()
    }

    pub fn to_minute(&self) -> u32 {
        self.to_date_time().minute()
    }

    pub fn to_time(&self) -> NaiveTime {
        self.to_date_time().time()
    }
}

impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let datetime = DateTime::<Local>::from(self.0);
        write!(f, "{}", datetime.format("%Y-%m-%d %H:%M:%S %Z").to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)] // we derive Default in order to use the clear() method in Drop
pub struct Deadline(Timestamp);

impl Deadline {
    pub fn new(hour: u8, minute: u8, second: u64) -> Self {
        let time_now = SystemTime::now();
        let _hour: u64 = hour as u64 * 3600;
        let _minute: u64 = minute as u64 * 60;

        Deadline(Timestamp(
            time_now.add(Duration::from_secs(_hour + _minute + second)),
        ))
    }

    pub fn to_blockchain_timestamp(&self) -> BlockchainTimestamp {
        self.0.to_blockchain_timestamp()
    }

    pub fn to_uint64(&self) -> Uint64 {
        self.to_blockchain_timestamp().to_uint64()
    }

    pub fn to_u32_array(&self) -> [u32; 2] {
        self.to_uint64().to_u32_array()
    }
}

impl core::fmt::Display for Deadline {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BlockchainTimestamp> for Deadline {
    fn from(e: BlockchainTimestamp) -> Self {
        Deadline(e.to_timestamp())
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
