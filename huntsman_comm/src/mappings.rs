use struct_helper::*;
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};
use serde::de::{Deserializer};
use usb_hut::hid_keyboard_page;

/// Struct to denote a physical key on the keyboard.
#[derive(Debug, Clone, Copy, Default, FromBytes, ToBytes, PartialEq, Eq, Deserialize, Serialize)]
pub struct Key {
    /// The key's at101 code, or whatever the keyboard uses to denote it.
    #[serde(serialize_with = "at101_serialize", deserialize_with = "at101_deserialize")]
    pub scan_code: u8,
    /// Whether or not this is the hypershift binding of that key.
    pub hypershift: bool,
}

// https://serde.rs/impl-serialize.html
fn at101_serialize<S>(scan_code: &u8, serializer: S) -> Result<S::Ok, S::Error> where    S: Serializer
{
    use serde::ser::Error;
    serializer.serialize_str(at101_to_key_name(*scan_code).map_err(Error::custom)?)
}

fn at101_deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error> where D: Deserializer<'de>
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    use serde::de::Error;
    let r = key_name_to_at101(s).map_err(Error::custom)?;
    Ok(r)
}



#[derive(Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Modifiers {
    left_ctrl: bool,
    left_shift: bool,
    left_alt: bool,
    // Maybe left meta?
    right_ctrl: bool,
    right_shift: bool,
    right_alt: bool,
    // Maybe right meta?
}
impl Modifiers {
    pub fn any(&self) -> bool {
        u8::from(*self) != 0
    }
}

// ^ Right modifier bitmask, 0x1=ctrl, 0x2=shift, 0x4 = alt
//  ^ Left modifier bitmask, 0x1=ctrl, 0x2=shift, 0x4 = alt
impl From<u8> for Modifiers {
    fn from(encoded: u8) -> Self {
        Modifiers {
            left_ctrl: (encoded & 0x01) != 0,
            left_shift: (encoded & 0x02) != 0,
            left_alt: (encoded & 0x04) != 0,
            right_ctrl: (encoded & 0x10) != 0,
            right_shift: (encoded & 0x20) != 0,
            right_alt: (encoded & 0x40) != 0,
        }
    }
}
impl From<Modifiers> for u8 {
    fn from(item: Modifiers) -> Self {
        ((item.left_ctrl as u8) << 0)
            | ((item.left_shift as u8) << 1)
            | ((item.left_alt as u8) << 2)
            | ((item.right_ctrl as u8) << (0 + 4))
            | ((item.right_shift as u8) << (1 + 4))
            | ((item.right_alt as u8) << (2 + 4))
    }
}

impl std::fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let enc: u8 = (*self).into();
        write!(f, "[")?;
        for (side_i, side) in ["left", "right"].iter().enumerate() {
            for (p, which) in ["ctrl", "shift", "alt"].iter().enumerate() {
                if (enc & ((1 << p) << (4 * side_i))) != 0 {
                    write!(f, "{}_{}", side, which)?
                }
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
/// Represents a particular HID Keyboard page key with modifiers.
pub struct KeyboardKey {
    #[serde(serialize_with = "at101_serialize", deserialize_with = "at101_deserialize")]
    pub id: u8,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
/// Represents a particular mouse button for mouse clicks.
pub enum MouseButton {
    Left = 1,
    Right = 2,
    Scroll = 3,
    M4 = 4,
    M5 = 5,
}
impl From<u8> for MouseButton {
    fn from(encoded: u8) -> Self {
        match encoded {
            _z if (encoded == MouseButton::Left as u8) => MouseButton::Left,
            _z if (encoded == MouseButton::Right as u8) => MouseButton::Right,
            _z if (encoded == MouseButton::Scroll as u8) => MouseButton::Scroll,
            _z if (encoded == MouseButton::M4 as u8) => MouseButton::M4,
            _z if (encoded == MouseButton::M5 as u8) => MouseButton::M5,
            _ => panic!("couldn't convert mouse button"),
        }
    }
}

pub type MacroId = u16;

/// Represent particular mapping for a physical key on the keyboard to produce any of the outputs
/// from this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum KeyMapping {
    /// Key is inactive
    Disabled,
    /// Mouse click
    Mouse(MouseButton), // This is a single button, not a bitmask like the macro flavour.
    /// This is a standard keyboard key, emitting a HID Keyboard Page (0x07) event.
    Key(KeyboardKey),
    /// Macro 'n' repeat.
    Macro(MacroId /* macro id */, u8 /* repeat count */),
    /// Macro repeat while pressed.
    MacroRepeat(MacroId),
    /// Macro toggle.
    MacroToggle(MacroId),
    /// Emits a multimedia hid code (Consumer Page 0x0c?)
    MultiMedia(u16),
    /// Emits from button page 0x09? Only seen for doubleclick.
    ButtonPage(u8),
    /// Repeats mouse clicks using the provided interval.
    TurboMouse(MouseButton, u16 /* interval delay */),
    /// Repeats keys using the provided interval.
    TurboKey(KeyboardKey, u16 /* interval delay */),
    /// Magical special keys, led brightness, game mode etc.
    Special(u8),
    /// Generic desktop page 0x01 (System Sleep)
    GenericDesktop(u8),
    /// Profile instruction? Seen once in hypershift + application profile cycle (0x04)
    ProfileInstruction(u8),
    /// Maps to enable hypershift when pressed, seems to take 1 byte payload, only seen as 0x01.
    Hypershift,
}

impl KeyMapping {
    const MAP_DISABLED: u8 = 0x00;
    const MAP_MOUSE: u8 = 0x01;
    const MAP_KEY: u8 = 0x02; // HID page 0x07
    const MAP_MACRO: u8 = 0x03;
    const MAP_MACRO_REPEAT: u8 = 0x04;
    const MAP_MACRO_TOGGLE: u8 = 0x05;

    const MAP_PROFILE_INSTRUCTION: u8 = 0x07;

    const MAP_GENERIC_DESKTOP: u8 = 0x09; // HID page 0x01
    const MAP_MULTI_MEDIA: u8 = 0x0a; // HID page 0x0c
    const MAP_BUTTON_PAGE: u8 = 0x0b; // HID page 0x09
    const MAP_HYPERSHIFT: u8 = 0x0c;
    const MAP_TURBO_KEY: u8 = 0x0d;
    const MAP_TURBO_MOUSE: u8 = 0x0e;

    const MAP_SPECIAL: u8 = 0x11;
}
/*
These would also seem... potentially useful when making a gaming keyboard?
    Game Controls Page (0x05)
    Unicode Page (0x10)
*/

impl FromBytes for KeyMapping {
    fn from_bytes(&mut self, src: &[u8], _endianness: Endianness) -> Result<usize, String>
    where
        Self: Sized,
    {
        let specification = src[0];
        let len_byte = src[1];
        match specification {
            KeyMapping::MAP_DISABLED => {
                *self = KeyMapping::Disabled;
                return Ok(2);
            }
            KeyMapping::MAP_KEY => {
                if len_byte != 2 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        2, len_byte
                    ));
                }
                *self = KeyMapping::Key(KeyboardKey {
                    id: src[3],
                    modifiers: src[2].into(),
                });

                return Ok(4);
            }
            KeyMapping::MAP_MOUSE => {
                if len_byte != 1 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        1, len_byte
                    ));
                }
                *self = KeyMapping::Mouse(src[2].into());

                return Ok(3);
            }
            KeyMapping::MAP_MACRO => {
                if len_byte != 3 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        3, len_byte
                    ));
                }
                let x: [u8; 2] = [src[3], src[2]];
                let macro_id = u16::from_le_bytes(x);
                let count = src[4];
                *self = KeyMapping::Macro(macro_id.into(), count);

                return Ok(5);
            }
            KeyMapping::MAP_MACRO_REPEAT => {
                if len_byte != 2 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        2, len_byte
                    ));
                }
                let x: [u8; 2] = [src[3], src[2]];
                let macro_id = u16::from_le_bytes(x);
                *self = KeyMapping::MacroRepeat(macro_id.into());

                return Ok(5);
            }
            KeyMapping::MAP_MACRO_TOGGLE => {
                if len_byte != 2 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        2, len_byte
                    ));
                }
                let x: [u8; 2] = [src[3], src[2]];
                let macro_id = u16::from_le_bytes(x);
                *self = KeyMapping::MacroToggle(macro_id.into());

                return Ok(5);
            }
            KeyMapping::MAP_MULTI_MEDIA => {
                if len_byte != 2 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        1, len_byte
                    ));
                }
                let x: [u8; 2] = [src[3], src[2]];
                let hid_id = u16::from_le_bytes(x);
                *self = KeyMapping::MultiMedia(hid_id);

                return Ok(4);
            }
            KeyMapping::MAP_BUTTON_PAGE => {
                if len_byte != 1 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        1, len_byte
                    ));
                }
                *self = KeyMapping::ButtonPage(src[2].into());

                return Ok(3);
            }
            KeyMapping::MAP_TURBO_MOUSE => {
                if len_byte != 3 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        3, len_byte
                    ));
                }
                let x: [u8; 2] = [src[4], src[3]];
                let repeat_interval = u16::from_le_bytes(x);
                *self = KeyMapping::TurboMouse(src[2].into(), repeat_interval);

                return Ok(5);
            }
            KeyMapping::MAP_TURBO_KEY => {
                if len_byte != 4 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        4, len_byte
                    ));
                }

                let x: [u8; 2] = [src[5], src[4]];
                let repeat_interval = u16::from_le_bytes(x);
                *self = KeyMapping::TurboKey(
                    KeyboardKey {
                        id: src[3],
                        modifiers: src[2].into(),
                    },
                    repeat_interval,
                );

                return Ok(6);
            }
            KeyMapping::MAP_SPECIAL => {
                if len_byte != 1 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        1, len_byte
                    ));
                }
                *self = KeyMapping::Special(src[2].into());

                return Ok(3);
            }
            KeyMapping::MAP_GENERIC_DESKTOP => {
                if len_byte != 1 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        1, len_byte
                    ));
                }
                *self = KeyMapping::GenericDesktop(src[2].into());

                return Ok(3);
            }
            KeyMapping::MAP_PROFILE_INSTRUCTION => {
                if len_byte != 1 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        1, len_byte
                    ));
                }
                *self = KeyMapping::ProfileInstruction(src[2].into());

                return Ok(3);
            }
            KeyMapping::MAP_HYPERSHIFT => {
                if len_byte != 1 {
                    return Err(format!(
                        "Length didn't match, expected {}, got {}",
                        1, len_byte
                    ));
                }
                let res = src[2];
                if res != 0x01 {
                    panic!(
                        "Hypershift class detected, but value is not 0x01, got 0x{:x}, full: {:?}",
                        res, src
                    );
                }
                *self = KeyMapping::Hypershift;

                return Ok(3);
            }
            z => panic!("Unhandled keymap code {:?}, total src: {:?}", z, src),
        }
    }
}

impl ToBytes for KeyMapping {
    fn to_bytes(&self, _endianness: Endianness) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = Vec::new();
        match self {
            KeyMapping::Disabled => {
                buff.push(KeyMapping::MAP_DISABLED);
                buff.push(0); // is followed by the length.
            }
            KeyMapping::Key(v) => {
                buff.push(KeyMapping::MAP_KEY);
                buff.push(2); // 2 bytes follow
                buff.push(v.modifiers.into());
                buff.push(v.id);
            }
            KeyMapping::Mouse(button) => {
                buff.push(KeyMapping::MAP_MOUSE);
                buff.push(1);
                buff.push(*button as u8);
            }
            KeyMapping::Macro(macro_id, count) => {
                buff.push(KeyMapping::MAP_MACRO);
                buff.push(3);
                let id = macro_id.to_le_bytes()?;
                buff.push(id[1]);
                buff.push(id[0]);
                buff.push(*count);
            }
            KeyMapping::MacroRepeat(macro_id) => {
                buff.push(KeyMapping::MAP_MACRO_REPEAT);
                buff.push(2);
                let id = macro_id.to_le_bytes()?;
                buff.push(id[1]);
                buff.push(id[0]);
            }
            KeyMapping::MacroToggle(macro_id) => {
                buff.push(KeyMapping::MAP_MACRO_TOGGLE);
                buff.push(2);
                let id = macro_id.to_le_bytes()?;
                buff.push(id[1]);
                buff.push(id[0]);
            }
            KeyMapping::MultiMedia(hid_id) => {
                buff.push(KeyMapping::MAP_MULTI_MEDIA);
                buff.push(2);
                let id = hid_id.to_le_bytes()?;
                buff.push(id[1]);
                buff.push(id[0]);
            }
            KeyMapping::ButtonPage(v) => {
                buff.push(KeyMapping::MAP_BUTTON_PAGE);
                buff.push(1);
                buff.push(*v);
            }
            KeyMapping::TurboMouse(button, interval) => {
                buff.push(KeyMapping::MAP_TURBO_MOUSE);
                buff.push(3);
                buff.push(*button as u8);
                let id = interval.to_le_bytes()?;
                buff.push(id[1]);
                buff.push(id[0]);
            }
            KeyMapping::TurboKey(button, interval) => {
                buff.push(KeyMapping::MAP_TURBO_KEY);
                buff.push(4);
                buff.push(button.modifiers.into());
                buff.push(button.id);
                let id = interval.to_le_bytes()?;
                buff.push(id[1]);
                buff.push(id[0]);
            }
            KeyMapping::Special(id) => {
                buff.push(KeyMapping::MAP_SPECIAL);
                buff.push(1);
                buff.push(*id as u8);
            }
            KeyMapping::GenericDesktop(id) => {
                buff.push(KeyMapping::MAP_GENERIC_DESKTOP);
                buff.push(1);
                buff.push(*id as u8);
            }
            KeyMapping::ProfileInstruction(id) => {
                buff.push(KeyMapping::MAP_PROFILE_INSTRUCTION);
                buff.push(1);
                buff.push(*id as u8);
            }
            KeyMapping::Hypershift => {
                buff.push(KeyMapping::MAP_HYPERSHIFT);
                buff.push(1);
                buff.push(1);
            }
        }
        Ok(buff)
    }
}
impl Default for KeyMapping {
    fn default() -> KeyMapping {
        KeyMapping::Disabled
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KeyMap {
    /// The profile to apply this mapping to.
    pub profile: u8,
    /// The physical key on the keyboard to map.
    pub key: Key,
    /// The effect that pressing this key will have.
    pub mapping: KeyMapping,
}

impl FromBytes for KeyMap {
    fn from_bytes(&mut self, src: &[u8], endianness: Endianness) -> Result<usize, String> {
        let mut offset: usize = 0;
        offset += self.profile.from_bytes(&src[0..1], endianness)?;
        offset += self.key.from_bytes(&src[1..3], endianness)?;
        offset += self.mapping.from_bytes(&src[3..], endianness)?;
        Ok(offset)
    }
}

impl ToBytes for KeyMap {
    fn to_bytes(&self, endianness: Endianness) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];
        buff.extend(self.profile.to_bytes(endianness)?);
        buff.extend(self.key.to_bytes(endianness)?);
        buff.extend(self.mapping.to_bytes(endianness)?);
        while buff.len() < 0x0a
        {
            buff.push(0)  // zero pad to the appropriate size.
        }
        Ok(buff)
    }
}



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


/// Function to look up a key by name and return the scan code.
pub fn key_name_to_at101(key: &str) ->  Result<u8, Box<dyn std::error::Error>>
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
                return Err(KeyError::boxed(format!("Key {}, found, but this key has no at101 scan code.", key)));
            }
        }
    }
    Err(KeyError::boxed(format!("Key not found, got {}.", key)))
}

pub fn at101_to_key_name(scan_code: u8) ->  Result<&'static str, Box<dyn std::error::Error>>
{
    for k in hid_keyboard_page::keys()
    {
        if let Some(key_code) = k.at101
        {
            if key_code == scan_code as usize
            {
                return Ok(&k.name);
            }
        }
    }
    Err(KeyError::boxed(format!("Could not find key for at101/scan_code: {}.", scan_code)))
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::helpers::{parse_wireshark_truncated, PAYLOAD_START};

    #[test]
    fn overrides_for_keys() {
        // Better; hut1_12v2.pdf -  HID Usage Tables
        // 'Typical AT-101' column holds the scancode in decimal
        // Usage ID is the.

        fn test_keymap_roundtrip(buf: &Vec<u8>) -> KeyMap {
            let cmd = KeyMap::from_be_bytes(&buf[PAYLOAD_START..]).expect("Should pass");
            println!("{:?}", cmd);
            let and_back = cmd.to_be_bytes().expect("Success");
            println!("{:?}", and_back);
            assert_eq!(
                &buf[PAYLOAD_START..PAYLOAD_START + and_back.len()],
                and_back
            );
            cmd
        }

        fn expect_key(k: KeyMap) -> KeyboardKey {
            if let KeyMapping::Key(v) = k.mapping {
                return v;
            }
            assert_eq!(true, false);
            panic!("cant reach this");
        }

        fn expect_mouse(k: KeyMap) -> MouseButton {
            if let KeyMapping::Mouse(v) = k.mapping {
                return v;
            }
            assert_eq!(true, false);
            panic!("cant reach this");
        }

        // disable
        let disable_key_62 =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:3e:00:00:00:00", 0x3a);
        let x = test_keymap_roundtrip(&disable_key_62);
        assert_eq!(x.key.scan_code, 62);

        // Test the key modifiers.
        let right_shift_to_right_shift =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:39:00:02:02:00:e5:00", 0xd8);
        let x = test_keymap_roundtrip(&right_shift_to_right_shift);
        assert_eq!(x.key.scan_code, 57); // at101 for right shift.
        let v = expect_key(x);
        assert_eq!(v.modifiers.any(), false);
        assert_eq!(v.id, 0xe5);

        let right_ctrl_alphanumeric_left_shift =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:02:02:02:04:00", 0x42);
        let v = expect_key(test_keymap_roundtrip(&right_ctrl_alphanumeric_left_shift));
        assert_eq!(v.modifiers.left_shift, true);

        let right_ctrl_alphanumeric_left_alt =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:02:02:04:04", 0x44);
        let v = expect_key(test_keymap_roundtrip(&right_ctrl_alphanumeric_left_alt));
        assert_eq!(v.modifiers.left_alt, true);

        let right_ctrl_alphanumeric_right_control =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:02:02:10:04:00", 0x50);
        let v = expect_key(test_keymap_roundtrip(
            &right_ctrl_alphanumeric_right_control,
        ));
        assert_eq!(v.modifiers.right_ctrl, true);

        let right_ctrl_alphanumeric_right_shift =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:02:02:20:04", 0x60);
        let v = expect_key(test_keymap_roundtrip(&right_ctrl_alphanumeric_right_shift));
        assert_eq!(v.modifiers.right_shift, true);

        // mouse button;
        let right_ctrl_right_click =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:01:01:02:00", 0x46);
        let v = expect_mouse(test_keymap_roundtrip(&right_ctrl_right_click));
        assert_eq!(v, MouseButton::Right);

        let right_ctrl_scroll_click =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:01:01:03:00", 0x47);
        let v = expect_mouse(test_keymap_roundtrip(&right_ctrl_scroll_click));
        assert_eq!(v, MouseButton::Scroll);

        // macro 'n' fire; n = 1
        let right_control_macro_single =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:03:03:3b:68:01:00", 0x16);
        let res = test_keymap_roundtrip(&right_control_macro_single);
        if let KeyMapping::Macro(macro_id, count) = res.mapping {
            assert_eq!(macro_id, 0x3b68);
            assert_eq!(count, 0x01);
        } else {
            assert_eq!(true, false);
        }

        // macro 'n' fire; n = 2
        let right_control_macro_double =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:03:03:3b:68:02:00", 0x15);
        let res = test_keymap_roundtrip(&right_control_macro_double);
        if let KeyMapping::Macro(macro_id, count) = res.mapping {
            assert_eq!(macro_id, 0x3b68);
            assert_eq!(count, 0x02);
        } else {
            assert_eq!(true, false);
        }

        // macro while pressed;
        let right_control_macro_repeat =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:04:02:3b:68:00", 0x11);
        let res = test_keymap_roundtrip(&right_control_macro_repeat);
        if let KeyMapping::MacroRepeat(macro_id) = res.mapping {
            assert_eq!(macro_id, 0x3b68);
        } else {
            assert_eq!(true, false);
        }

        // macro toggle
        let right_control_macro_toggle =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:05:02:3b:68:00", 0x10);
        let res = test_keymap_roundtrip(&right_control_macro_toggle);
        if let KeyMapping::MacroToggle(macro_id) = res.mapping {
            assert_eq!(macro_id, 0x3b68);
        } else {
            assert_eq!(true, false);
        }

        // multimedia page
        let right_control_volume_mute =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:0a:02:00:e2:00", 0xae);
        let res = test_keymap_roundtrip(&right_control_volume_mute);
        if let KeyMapping::MultiMedia(hid_id) = res.mapping {
            assert_eq!(hid_id, 0x00e2);
        } else {
            assert_eq!(true, false);
        }

        // button page
        let right_control_double_click =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:0b:01:01:00", 0x4f);
        let res = test_keymap_roundtrip(&right_control_double_click);
        if let KeyMapping::ButtonPage(hid_id) = res.mapping {
            assert_eq!(hid_id, 0x01);
        } else {
            assert_eq!(true, false);
        }

        // turbo mouse...
        let right_control_turbo_left_click_7_per_s =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:03:40:00:0e:03:01:00:8e:00", 0xc4);
        let res = test_keymap_roundtrip(&right_control_turbo_left_click_7_per_s);
        if let KeyMapping::TurboMouse(button, repeat_interval) = res.mapping {
            assert_eq!(button, MouseButton::Left);
            assert_eq!(repeat_interval, (1000.0f64 / 7.0).floor() as u16);
        } else {
            assert_eq!(true, false);
        }

        // turbo keyboard.
        // 2021_06_05_23_32_set_right_ctrl_alpha_numeric_include_mod_right_alt_and_20_turbo
        let right_ctrl_alphanumeric_mod_right_alt_and_20_per_s = parse_wireshark_truncated(
            "00:1f:00:00:00:0a:02:0d:01:40:00:0d:04:40:04:00:32:00",
            0x3b,
        );
        let res = test_keymap_roundtrip(&right_ctrl_alphanumeric_mod_right_alt_and_20_per_s);
        if let KeyMapping::TurboKey(button, repeat_interval) = res.mapping {
            assert_eq!(button.id, 0x04);
            assert_eq!(button.modifiers.right_alt, true);
            assert_eq!(repeat_interval, (1000.0f64 / 20.0).floor() as u16);
        } else {
            assert_eq!(true, false);
        }

        // special
        let f9_otf_macro =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:04:78:01:11:01:04:00", 0xe0);
        let res = test_keymap_roundtrip(&f9_otf_macro);
        if let KeyMapping::Special(id) = res.mapping {
            assert_eq!(id, 0x04);
        } else {
            assert_eq!(true, false);
        }

        // Generic Desktop page, 0x82 corresponds to System Sleep
        let pause_sleep =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:04:7e:01:09:01:82:00", 0x78);
        let res = test_keymap_roundtrip(&pause_sleep);
        if let KeyMapping::GenericDesktop(id) = res.mapping {
            assert_eq!(id, 0x82);
        } else {
            assert_eq!(true, false);
        }

        // More special sauce... profile cycle;
        let hypershift_application_cycle_profile =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:01:81:01:07:01:04:00", 0x0a);
        let res = test_keymap_roundtrip(&hypershift_application_cycle_profile);
        if let KeyMapping::ProfileInstruction(id) = res.mapping {
            assert_eq!(id, 0x04);
        } else {
            assert_eq!(true, false);
        }

        // And hypershift map, yet another class identifier.
        let right_control_hypershift =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:0c:01:01:00", 0x48);
        let res = test_keymap_roundtrip(&right_control_hypershift);
        assert_eq!(KeyMapping::Hypershift, res.mapping);

        /*
            The media buttons at the top right of the keyboard, as well as the fn button itself do
            not seem to be expressed as proper key maps. When querying all keys, it appears none of
            them are bound to the multimedia page, neither do we find the fn key to map to the
            hypershift class.
        */

        // These keystrokes... they may be specified in the same way as the macros? Doesn't look like it...
        // Mapping type 11 is indeed magical keys.
        // Looks like I lost game mode toggling on 2 profiles somehow.
        // Captured from working profile:
        let _f9_otf_macro =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:04:78:01:11:01:04:00", 0xe0);
        let _f10_game_mode =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:04:79:01:11:01:03:00", 0xe6);
        let _f11_brightness_down =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:04:7a:01:11:01:09:00", 0xef);
        let _f12_brightness_up =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:04:7b:01:11:01:08:00", 0xef);

        // Other special keys, seen once class identifiers.
        // This is the Generic Desktop Page
        let _pause_sleep =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:04:7e:01:09:01:82:00", 0x78);
        // This is the profile cycle shortcut, the hardware-enabled profile cycle....
        let _hypershift_application =
            parse_wireshark_truncated("02:1f:00:00:00:06:02:8d:01:81:01:07:01:04:00", 0x0a);

        // Set right control as hypershift;
        let _right_control_hypershift =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:0c:01:01:00", 0x48);
    }


    fn print_serialize<T: Serialize + std::fmt::Debug>(v: T) -> String
    {
        let serialized = serde_json::to_string(&v).unwrap();
        println!("serialize {:?} -> {}", v, serialized);
        serialized
    }
    fn print_deserialize<'a, T: Deserialize<'a> + Sized + std::fmt::Debug>(v: &'a str) -> T
    {
        let deserialized: T = serde_json::from_str(&v).unwrap();
        println!("deserialize {} -> {:?}", v, deserialized);
        deserialized
    }

    #[test]
    fn test_key_lookup() {
        assert_eq!(key_name_to_at101("KEY_RIGHT_META").expect("Should be found"), 128);
        assert_eq!(key_name_to_at101("RIGHT_META").expect("Should be found"), 128);
        assert_eq!(key_name_to_at101("right_meta").expect("Should be found"), 128);
        assert_eq!(key_name_to_at101("a").expect("Should be found"), 31);
        assert_eq!(key_name_to_at101("1").expect("Should be found"), 2);
        assert_eq!(key_name_to_at101("kpd_plus").expect("Should be found"), 106);

        assert!(key_name_to_at101("kpd_c").is_err()); // no at101 code
        assert!(key_name_to_at101("this is not a key").is_err()); // no key found
    }


    #[test]
    pub fn test_key_serialize()
    {

        print_serialize(KeyMapping::Disabled);
        print_serialize(KeyMapping::Mouse(MouseButton::Left));
        print_serialize(KeyMapping::Key(KeyboardKey{id: 0x04, ..Default::default()}));

        print_serialize(Key{scan_code: 0x04, hypershift: false});
        print_serialize(Key{scan_code: 0x31, hypershift: false});

        print_deserialize::<Key>("{\"scan_code\":\"KEY_V\",\"hypershift\":false}");
    }

}
