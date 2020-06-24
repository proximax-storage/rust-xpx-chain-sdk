/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

#[macro_use]
extern crate serde;

use serde::de::Error as SerdeError;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use xpx_chain_api::{TransactionDto, Uint64Dto};

const ALLOW_ADDRESS: u16 = 0x01;
const ALLOW_MOSAIC: u16 = 0x02;
const ALLOW_TRANSACTION: u16 = 0x04;
const SENTINEL: u16 = 0x05;
const BLOCK_ADDRESS: u16 = 0x80 + 0x01;
const BLOCK_MOSAIC: u16 = 0x80 + 0x02;
const BLOCK_TRANSACTION: u16 = 0x80 + 0x04;

#[derive(Debug)]
struct AccountPropertiesModificationDto {
    pub property_type: u16,
    pub mosaic_ids: Option<Vec<Uint64Dto>>,
    pub addresses: Option<Vec<String>>,
    pub entity_types: Option<Vec<u8>>,
}

impl Serialize for AccountPropertiesModificationDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("AccountPropertiesModificationDto", 4)?;

        state.serialize_field("propertyType", &self.property_type)?;
        // if &self._type & ALLOW_ADDRESS != 0 {
        //     state.serialize_field("addresses", &self.addresses)?;
        // } else if &self._type & ALLOW_MOSAIC != 0 {
        //     state.serialize_field("mosaicIds", &self.mosaic_ids)?;
        // } else if &self._type & ALLOW_TRANSACTION != 0 {
        //     state.serialize_field("entityTypes", &self.entity_types)?;
        // } else {
        //     panic!("not supported PropertyType");
        // }
        state.end()
    }
}

impl<'de> Deserialize<'de> for AccountPropertiesModificationDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct PropertyValue {
            #[serde(rename = "propertyType")]
            _type: u16,
            modifications: Value,
        }

        let property = PropertyValue::deserialize(deserializer)?;

        let mut addresses = None;
        let mut mosaic_ids = None;
        let mut entity_types = None;

        if property._type & ALLOW_ADDRESS != 0 {
            addresses = Some(serde_json::from_value(property.modifications).unwrap());
        } else if property._type & ALLOW_MOSAIC != 0 {
            mosaic_ids = Some(vec![serde_json::from_value(property.modifications).unwrap()])
        } else if property._type & ALLOW_TRANSACTION != 0 {
            entity_types = Some(serde_json::from_value(property.modifications).unwrap_or_default());
        };

        Ok(AccountPropertiesModificationDto {
            property_type: property._type,
            mosaic_ids,
            addresses,
            entity_types,
        })
    }
}

fn main() {
    println!("{}", 0x84);
    let data = r#"{
"meta": {
"height": [
1552315,
0
],
"hash": "9E9266073941265F5F445C6DE49EA10B57537B6527BA0F255E1D2281AE194B18",
"merkleComponentHash": "9E9266073941265F5F445C6DE49EA10B57537B6527BA0F255E1D2281AE194B18",
"index": 0,
"id": "5EF20E481F081200014B41F6"
},
"transaction": {
"signature": "EC69F64451FF8BB02620DE0A02B6BF720792C768B379E190893A1EA7A5E8387C659ACD2D6A57A3024C8FDBF8C18FEDFC1A4531B7C837DDD46AF3F22EB9651204",
"signer": "527636076D4ABCB01A6B2E6B8BA88CF041535526102D3D9F4D85AF20E00E3537",
"version": 2818572289,
"type": 16720,
"maxFee": [
0,
0
],
"deadline": [
312448769,
31
],
"propertyType": 129,
"modifications": [
{
"type": 0,
"value": "A88046701A76CA3034E940A3DA56143A03B03A04A3BEBF420D"
}
]
}
}"#;

    let de: xpx_chain_api::AccountPropertiesTransactionInfoDto =
        serde_json::from_str(data).unwrap();
    println!("{}", de.compact().unwrap());
}
