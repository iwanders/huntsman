pub use crate::commands::mappings::{Key, KeyMapping, KeyboardKey, Modifier, Modifiers};
use serde::{Deserialize, Serialize};

pub use crate::commands::macros::MacroAction;
pub use crate::keymap_util::{at101_keys, default_keymaps, get_default_keymap, hypershift_keymaps};

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
