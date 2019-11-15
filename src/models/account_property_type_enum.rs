/// AccountPropertyTypeEnum : The account properties type: * 0x01 (1 decimal) - The property type only allows receiving transactions from an address. * 0x02 (2 decimal) - The property type only allows receiving transactions containing a mosaic id. * 0x04 (4 decimal) - The property type only allows sending transactions with a given transaction type. * 0x05 (5 decimal) - Property type sentinel. * 0x81 (129 decimal) - The property type blocks receiving transactions from an address. * 0x82 (130 decimal) - The property type blocks receiving transactions containing a mosaic id. * 0x84 (132 decimal) -  The property type blocks sending transactions with a given transaction type.

/// The account properties type: * 0x01 (1 decimal) - The property type only allows receiving transactions from an address. * 0x02 (2 decimal) - The property type only allows receiving transactions containing a mosaic id. * 0x04 (4 decimal) - The property type only allows sending transactions with a given transaction type. * 0x05 (5 decimal) - Property type sentinel. * 0x81 (129 decimal) - The property type blocks receiving transactions from an address. * 0x82 (130 decimal) - The property type blocks receiving transactions containing a mosaic id. * 0x84 (132 decimal) -  The property type blocks sending transactions with a given transaction type. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountPropertyTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "4")]
    _4,
    #[serde(rename = "5")]
    _5,
    #[serde(rename = "129")]
    _129,
    #[serde(rename = "130")]
    _130,
    #[serde(rename = "132")]
    _132,

}




