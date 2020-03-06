#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::mosaic::{Mosaic, MosaicId};
use xpx_chain_sdk::Uint64;

fn main() {
    let mosaic_xpx = Mosaic::xpx(100);
    println!("Mosaic Xpx: {}\n", mosaic_xpx);

    let mosaic_xpx = Mosaic::xpx_relative(100);
    println!("Mosaic XpxRelative: {}\n", mosaic_xpx);

    let mosaic_id = MosaicId::from_hex("DC67FBD1CAD29E2").unwrap();

    let amount = Uint64::new(100);

    let mosaic_from_id = Mosaic::new(Box::new(mosaic_id), amount);
    println!("Mosaic FromId: {}\n", mosaic_from_id);
}
