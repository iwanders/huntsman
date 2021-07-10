use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::Deserialize;
use usb_hut::hid_keyboard_page;

#[derive(Debug)]
struct KeyError {
    details: String,
}

impl KeyError {
    fn new(msg: &str) -> KeyError {
        KeyError {
            details: msg.to_string(),
        }
    }
    fn boxed(msg: String) -> Box<KeyError> {
        Box::new(KeyError::new(msg.as_str()))
    }
}
impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "KeyError {}", self.details)
    }
}
impl std::error::Error for KeyError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn key_name_to_key(key: &str) -> Result<usb_hut::Key, Box<dyn std::error::Error>> {
    // try to find a key that matches our self.key.
    let key_uppercase = key.to_uppercase();
    let with_key = "KEY_".to_string() + &key_uppercase;
    for k in hid_keyboard_page::keys() {
        if k.name == &key_uppercase || k.name == &with_key {
            // how delightful, we found the key.
            // No guarantee for success though, we also need to check whether we have an AT101
            // code, if so we are in business, otherwise we still fail :(
            return Ok(*k);
        }
    }
    Err(KeyError::boxed(format!("Key not found, got {}.", key)))
}

/// Function to look up a key by name and return the scan code.
pub fn key_name_to_at101(key: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let k = key_name_to_key(key)?;
    if let Some(code) = k.at101 {
        return Ok(code as u8);
    } else {
        return Err(KeyError::boxed(format!(
            "Key {}, found, but this key has no at101 scan code.",
            key
        )));
    }
}

/// Function to go from at101 code to a key name.
pub fn at101_to_key_name(scan_code: u8) -> Result<&'static str, Box<dyn std::error::Error>> {
    for k in hid_keyboard_page::keys() {
        if let Some(key_code) = k.at101 {
            if key_code == scan_code as usize {
                return Ok(&k.name);
            }
        }
    }
    Err(KeyError::boxed(format!(
        "Could not find key for at101/scan_code: {}.",
        scan_code
    )))
}

/// Function to go from a keyboard hid id to a key name.
pub fn keyboard_hid_to_key_name(hid_id: u8) -> Result<&'static str, Box<dyn std::error::Error>> {
    for k in hid_keyboard_page::keys() {
        if k.hid == hid_id as usize {
            return Ok(&k.name);
        }
    }
    Err(KeyError::boxed(format!(
        "Could not find key for usb hid id: {}.",
        hid_id
    )))
}

/// Function to look up a key by name and return the scan code.
pub fn key_name_to_keyboard_hid(key: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let k = key_name_to_key(key)?;
    Ok(k.hid as u8)
}

// https://serde.rs/impl-serialize.html
pub fn at101_serialize<S>(scan_code: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::Error;
    serializer.serialize_str(at101_to_key_name(*scan_code).map_err(Error::custom)?)
}

pub fn at101_deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    use serde::de::Error;
    let r = key_name_to_at101(&s).map_err(Error::custom)?;
    Ok(r)
}

pub fn keyboard_page_serialize<S>(scan_code: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::Error;
    serializer.serialize_str(keyboard_hid_to_key_name(*scan_code).map_err(Error::custom)?)
}

pub fn keyboard_page_deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    use serde::de::Error;
    let r = key_name_to_keyboard_hid(&s).map_err(Error::custom)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_key_lookup() {
        assert_eq!(
            key_name_to_at101("KEY_RIGHT_META").expect("Should be found"),
            128
        );
        assert_eq!(
            key_name_to_at101("RIGHT_META").expect("Should be found"),
            128
        );
        assert_eq!(
            key_name_to_at101("right_meta").expect("Should be found"),
            128
        );
        assert_eq!(key_name_to_at101("a").expect("Should be found"), 31);
        assert_eq!(key_name_to_at101("1").expect("Should be found"), 2);
        assert_eq!(key_name_to_at101("kpd_plus").expect("Should be found"), 106);

        assert!(key_name_to_at101("kpd_c").is_err()); // no at101 code
        assert!(key_name_to_at101("this is not a key").is_err()); // no key found
    }
}
