// Why, oh why don't they just provide a machine readable flavour?
#![allow(dead_code)]
mod defs;

pub use defs::*;
mod generated;
pub use generated::{hid_keys, keys};
