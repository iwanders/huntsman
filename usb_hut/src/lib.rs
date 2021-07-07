// Why, oh why don't they just provide a machine readable flavour?
#![allow(dead_code)]
mod defs;

pub use defs::*;
mod keyboard_page;
pub use keyboard_page::{hid_keyboard_page};
