use models::account::PublicAccount;
use models::Uint64;

use super::{MosaicFlags, MosaicId};

pub struct MosaicInfo {
    /// The mosaic ID.
    pub  mosaic_id: MosaicId,

    /// The total supply of the mosaic.
    pub  supply: Uint64,

    /// The block height the mosaic was created.
    pub   height: Uint64,

    /// The account of the owner of this mosaic.
    pub owner: PublicAccount,

    /// The mosaic revision.
    pub revision: usize,

    /// The mosaic flags.
    pub flags: MosaicFlags,

    /// The divisibility determines the decimal place the mosaic can be divided into.
    pub divisibility: usize,

    /// The duration in blocks a mosaic will become available.
    pub duration: Uint64,
}
