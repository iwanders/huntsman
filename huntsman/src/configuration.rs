
use serde::{Deserialize, Serialize};

use usb_hut::hid_keyboard_page;
use huntsman_comm::mappings;

pub use huntsman_comm::mappings::KeyMapping;



#[derive(Debug)]
struct ConfigurationError {
    details: String,
}

impl ConfigurationError {
    fn new(msg: &str) -> ConfigurationError {
        ConfigurationError {
            details: msg.to_string(),
        }
    }
    fn boxed(msg: String) -> Box<ConfigurationError> {
        Box::new(ConfigurationError::new(msg.as_str()))
    }
}
impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ConfigurationError {}", self.details)
    }
}
impl std::error::Error for ConfigurationError {
    fn description(&self) -> &str {
        &self.details
    }
}



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

/// Function to look up a key by name and return the scan code.
pub fn key_name_to_scan_code(key: &str) -> Result<u8, Box<dyn std::error::Error>>
{
    // try to find a key that matches our self.key.
    let key_uppercase = key.to_uppercase();
    let with_key = "KEY_".to_string() + &key_uppercase;
    for k in hid_keyboard_page::keys()
    {
        if k.name == &key_uppercase || k.name == &with_key
        {
            // how delightful, we found the key.
            // No guarantee for success though, we also need to check whether we have an AT101
            // code, if so we are in business, otherwise we still fail :(
            if let Some(code) = k.at101
            {
                return Ok(code as u8);
            }
            else
            {
                return Err(ConfigurationError::boxed(format!("Key {}, found, but this key has no at101 scan code.", key)));
            }
        }
    }
    Err(ConfigurationError::boxed(format!("Key not found, got {}.", key)))
}

impl KeyConfig
{
    pub fn to_key(&self) -> Result<mappings::Key, Box<dyn std::error::Error>>
    {
        let scan_code = key_name_to_scan_code(&self.key)?;
        return Ok(mappings::Key {
                    scan_code: scan_code,
                    hypershift: self.hypershift,
                });
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    pub use huntsman_comm::mappings::{MouseButton, KeyboardKey};

    #[test]
    fn test_key_lookup() {
        assert_eq!(key_name_to_scan_code("KEY_RIGHT_META").expect("Should be found"), 128);
        assert_eq!(key_name_to_scan_code("RIGHT_META").expect("Should be found"), 128);
        assert_eq!(key_name_to_scan_code("right_meta").expect("Should be found"), 128);
        assert_eq!(key_name_to_scan_code("a").expect("Should be found"), 31);
        assert_eq!(key_name_to_scan_code("1").expect("Should be found"), 2);
        assert_eq!(key_name_to_scan_code("kpd_plus").expect("Should be found"), 106);

        assert!(key_name_to_scan_code("kpd_c").is_err()); // no at101 code
        assert!(key_name_to_scan_code("this is not a key").is_err()); // no key found
    }


    #[test]
    pub fn z()
    {
        fn print_serialize(v: KeyConfig) -> String
        {
            let serialized = serde_json::to_string(&v).unwrap();
            println!("serialize {:?} -> {}", v, serialized);
            serialized
        }
        fn print_deserialize(v: String) -> KeyConfig
        {
            let deserialized: KeyConfig = serde_json::from_str(&v).unwrap();
            println!("deserialize {} -> {:?}", v, deserialized);
            deserialized
        }

        print_serialize(KeyConfig{key: "A".to_string(), hypershift: false, mapping: KeyMapping::Disabled});
        print_serialize(KeyConfig{key: "A".to_string(), hypershift: false, mapping: KeyMapping::Mouse(MouseButton::Left)});
        print_serialize(KeyConfig{key: "A".to_string(), hypershift: false, mapping: KeyMapping::Key(KeyboardKey{id: 0x04, ..Default::default()})});

    }

}