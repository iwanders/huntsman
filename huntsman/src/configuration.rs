
use serde::{Deserialize, Serialize};

use usb_hut::hid_keyboard_page;
use huntsman_comm::mappings;

pub use huntsman_comm::mappings::KeyMapping;



#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeyConfig
{
    /// Human readable key name, should match against name from usb_hut.
    pub key: String,
    #[serde(default)]
    pub hypershift: bool,
    /// Action this key maps to.
    pub mapping: KeyMapping,
}

impl KeyConfig
{
    pub fn to_key(&self) -> Result<mappings::Key, Box<dyn std::error::Error>>
    {
        let scan_code = huntsman_comm::mappings::key_name_to_scan_code(&self.key)?;
        return Ok(mappings::Key {
                    scan_code: scan_code,
                    hypershift: self.hypershift,
                });
    }
}


