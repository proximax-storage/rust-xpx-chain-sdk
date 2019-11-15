/// MosaicPropertyIdEnum : The mosaic propery id means: * 0 - MosaicFlags * 1 - Divisibility * 2 - Duration

/// The mosaic propery id means: * 0 - MosaicFlags * 1 - Divisibility * 2 - Duration 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MosaicPropertyIdEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,

}




