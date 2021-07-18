pub use crate::commands::mappings::{Key, KeyMapping, KeyboardKey, Modifiers};
use serde::{Deserialize, Serialize};

pub use crate::commands::macros::MacroAction;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeyConfig {
    /// Profile to set for this key.
    #[serde(default)]
    pub profile: Option<u8>,
    /// Human readable key name, should match against name from usb_hut.
    #[serde(flatten)]
    pub key: Key,
    /// Action this key maps to.
    pub mapping: KeyMapping,
}

pub fn load_mappings(filename: &str) -> Result<Vec<KeyConfig>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename).expect("file should be opened");
    if filename.ends_with("yaml") {
        let yaml: serde_yaml::Value =
            serde_yaml::from_reader(file).expect("file should be proper yaml");
        let effects: Vec<KeyConfig> = serde_yaml::from_value(
            yaml.get("mappings")
                .expect("file should have mappings key")
                .clone(),
        )?;
        return Ok(effects);
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "File type not supported. Use .yaml.",
    )))
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct MacroConfig {
    #[serde(default)]
    pub macro_id: u16,
    pub events: Vec<MacroAction>,
}

pub fn load_macro(filename: &str) -> Result<MacroConfig, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename).expect("file should be opened");
    if filename.ends_with("yaml") {
        let yaml: serde_yaml::Value =
            serde_yaml::from_reader(file).expect("file should be proper yaml");
        let macro_config: MacroConfig = serde_yaml::from_value(yaml.clone())?;
        return Ok(macro_config);
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "File type not supported. Use .yaml.",
    )))
}

pub fn at101_keys() -> &'static [u8] {
    const KEYS_PRESENT: [u8; 102] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0f, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2b, 0x2c, 0x2e, 0x2f, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x39, 0x3a, 0x3c, 0x3d, 0x3e, 0x40, 0x4b, 0x4c,
        0x4f, 0x50, 0x51, 0x53, 0x54, 0x55, 0x56, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5f, 0x60, 0x61,
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6c, 0x6e, 0x70, 0x71, 0x72, 0x73,
        0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x81,
    ];
    &KEYS_PRESENT
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    fn print_serialize<T: Serialize + std::fmt::Debug>(v: T) -> String {
        let serialized = serde_yaml::to_string(&v).unwrap();
        println!("serialize {:?} -> {}", v, serialized);
        serialized
    }

    #[test]
    fn test_key_lookup() {
        print_serialize(KeyConfig {
            profile: None,
            key: Key {
                id: 0x04,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0x04,
                modifiers: Modifiers::shift(),
            }),
        });
    }
}
