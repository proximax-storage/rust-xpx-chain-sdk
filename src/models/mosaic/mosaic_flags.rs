/// `MosaicFlags` structure which includes several properties for defining mosaic
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicFlags {
    /// `supply_mutable` - is supply of defined mosaic can be changed in future
    ///
    #[serde(rename = "supplyMutable")]
    pub supply_mutable: bool,

    /// `transferable` - if this property is set to "false",
    /// only transfer transactions having the creator as sender or as recipient can transfer mosaics of
    /// that type. If set to "true" the mosaics can be transferred to and from arbitrary accounts
    ///
    #[serde(rename = "transferable")]
    pub transferable: bool,

    /// `divisibility` - divisibility determines up to what decimal place the mosaic can be divided into
    ///
    #[serde(rename = "divisibility")]
    pub divisibility: bool,
}

impl MosaicFlags {
    pub fn new(supply_mutable: bool, transferable: bool, divisibility: bool) -> MosaicFlags {
        MosaicFlags {
            supply_mutable,
            transferable,
            divisibility,
        }
    }
}

/// Creates `MosaicFlags` with the default parameters.
impl Default for MosaicFlags {
    fn default() -> Self {
        MosaicFlags { supply_mutable: true, transferable: true, divisibility: false }
    }
}
