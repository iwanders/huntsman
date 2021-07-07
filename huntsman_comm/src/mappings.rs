use struct_helper::*;

/// Struct to denote a physical key on the keyboard.
#[derive(Debug, Clone, Copy, Default, FromBytes, ToBytes, PartialEq, Eq)]
pub struct Key {
    /// The key's at101 code, or whatever the keyboard uses to denote it.
    pub scan_code: u8,
    /// Whether or not this is the hypershift binding of that key.
    pub hypershift: bool,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// Represents a particular HID Keyboard page key with modifiers.
pub struct KeyboardKey {
    pub id: u8,
    pub modifiers: Modifiers,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                if res != 0x01
                {
                    panic!("Hypershift class detected, but value is not 0x01, got 0x{:x}, full: {:?}", res, src);
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

#[derive(Clone, Debug, Default)]

struct KeyMap {
    pub profile: u8,
    pub key: Key,
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
        Ok(buff)
    }
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
        // 2021_06_05_23_32_set_right_ctrl_alpha_numeric_include_mod_right_alt_and_20_turbo
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

        // 2021_06_05_23_25_set_right_ctrl_scroll_click.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:03:00:00:00:00:00
        // 2021_06_05_23_25_set_right_ctrl_button_5.pcapng  <- prob 4...
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:04:00:00:00:00:00
        // 2021_06_05_23_26_set_right_ctrl_button_5.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:05:00:00:00:00:00

        // Right shift to k, top two entries seem to be THE thing.
        // TSetMapping MOD: 0x00000000	MAP: MAPPING_SINGLEKEY Key: MC=0x25, EX=0x00, Mods=0x00000000
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:39:00:02:02:00:0e:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:39:00:02:02:00:0e:00...
        //                                                                        ^^  hid id for 'k'

        // Right shift to default:
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:39:00:02:02:00:e5:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:39:00:02:02:00:e5:00...
        //                                                                        ^^  hid id for Right shift.

        // Unbind hypershift minus (on numpad)
        //                              00:1f:00:00:00:0a:02:0d:01:69:01:02:02:00:56:00...

        // Right control to mouse rightclick.
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:40:00:01:01:02:00:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:40:00:01:01:02:00:00...
        // Hypershift 3 as rightclick:
        //                              00:1f:00:00:00:0a:02:0d:01:04:01:01:01:02:00:00...
        //                                                            ^^   is hypershift / modifier?

        // Right control to mouse leftclick.
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:40:00:01:01:01:00:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:40:00:01:01:01:00:00...
        //                                profile or activation?^^
        //                                                    key  ^^                      :check:
        //                                                                        ^^  hid id key if keyboard
        //                                                                     ^^  Mouse buttion / action?
        //                                              kbd or mouse?    ^^  01 is mouse, 02 is kbd?
        //                                                                  ^^ ^^    ???

        // This looks more like a disable...
        // Bind hypershift plus to brightness up:
        // 00:1f:00:00:00:0a:02:0d:01:6a:01:00:00:00:00:00....
        // Bind hypershift minus to brightness down:
        // 00:1f:00:00:00:0a:02:0d:01:69:01:00:00:00:00:00...

        // Switch profile with right alt... that just updates the mappings that are currently active
        // with the '01' in first?
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:01:00:00:00:00
        // 00:1f:00:00:00:0a:02:0d:01:3e:00:00:00:00:00:00:00:00
        // 00:1f:00:00:00:0a:02:0d:01:02:01:00:00:00:00:00:00:00
        // 00:1f:00:00:00:0a:02:0d:01:03:01:01:01:01:00:00:00:00

        // Keyboard instructions;

        // Bind hypershift plus to brightness up:
        // 00:1f:00:00:00:0a:02:0d:01:6a:01:00:00:00:00:00....

        // 2021_06_05_23_30_set_right_ctrl_alpha_numeric_include_mod_left_shift.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:02:02:02:04:00:00:00:00
        // 2021_06_05_23_31_set_right_ctrl_alpha_numeric_include_mod_left_alt.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:02:02:04:04:00:00:00:00

        // 2021_06_05_23_30_set_right_ctrl_alpha_numeric_include_mod_no_mod.pcapng.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:02:02:00:04:00:00:00:00

        // 2021_06_05_23_31_set_right_ctrl_alpha_numeric_include_mod_right_ctrl.pcapng
        // 02:1f:00:00:00:0a:02:0d:01:40:00:02:02:10:04:00:00:00:00
        // 2021_06_05_23_31_set_right_ctrl_alpha_numeric_include_mod_right_shift.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:02:02:20:04:00:00:00:00

        // 2021_06_05_23_32_set_right_ctrl_alpha_numeric_include_mod_right_alt_and_20_turbo
        // 00:1f:00:00:00:0a:02:0d:01:40:00:0d:04:40:04:00:32:00:00
        //                                        ^ Right modifier bitmask, 0x1=ctrl, 0x2=shift, 0x4 = alt
        //                                         ^ Left modifier bitmask, 0x1=ctrl, 0x2=shift, 0x4 = alt

        // 2021_06_05_23_32_set_right_ctrl_alpha_numeric_and_20_turbo.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:0d:04:00:04:00:32:00:00
        // 2021_06_05_23_32_set_right_ctrl_alpha_numeric_include_mod_right_alt_and_20_tur
        // 00:1f:00:00:00:0a:02:0d:01:40:00:0d:04:40:04:00:32:00:00

        // 2021_06_05_23_32_set_right_ctrl_alpha_numeric_include_mod_right_alt.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:02:02:40:04:00:00:00:00

        // 2021_06_05_23_27_set_right_ctrl_scroll_up.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:09:00:00:00:00:00

        // Mouse instructions

        // 2021_06_05_23_22_set_right_ctrl_left_click_turbo_7_per_s.pcapng
        // 00:1f:00:00:00:0a:02:0d:03:40:00:0e:03:01:00:8e:00:00:00

        // 2021_06_05_23_26_set_right_ctrl_dbl_click.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:0b:01:01:00:00:00:00:00
        // 2021_06_05_23_27_set_right_ctrl_scroll_down.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:0a:00:00:00:00:00
        // 2021_06_05_23_27_set_right_ctrl_scroll_up.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:09:00:00:00:00:00
        // 2021_06_05_23_27_set_right_ctrl_scroll_left_synapse.pcapng  <- disable
        // 00:1f:00:00:00:0a:02:0d:01:40:00:00:00:00:00:00:00:00:00
        //

        // index[2] input type[KEYBOARD] flag[2] makecode[29] modifier[0x0] mapping type[BUTTON] button[5]
        // 00:1f:00:00:00:0a:02:0d:01:40:00:01:01:05:00:00:00:00:00:00:00:00:00:41:00
        // Not too sure what that makecode means...

        // Doubleclick;
        //  [Done] index[2] input type[KEYBOARD] flag[2] makecode[29] modifier[0x0] mapping type[BUTTON] button[13]
        // 02:1f:00:00:00:0a:02:0d:01:40:00:0b:01:01:00:00:00:00:00:4f:00
        //                                  ^^ 0b instead of 01!? Maybe...
        //                                  0b for Button Page (0x09)? 01 is Button 1, Primary Button. Used for object selecting, dragging, and double click activation.

        // 2021_06_05_23_32_set_right_ctrl_alpha_numeric_include_mod_right_alt_and_20_turbo
        // [Done] index[0] input type[KEYBOARD] flag[2] makecode[29] modifier[0x0] mapping type[SINGLEKEYTURBO]
        // 00:1f:00:00:00:0a:02:0d:01:40:00:0d:04:40:04:00:32:00:00:3b:00
        //                                  ^^ 0x0d now...

        // [profile data event] [Done] index[2] input type[KEYBOARD] flag[2] makecode[29] modifier[0x0] mapping type[SINGLEKEYTURBO]
        // 00:1f:00:00:00:0a:02:0d:01:40:00:0d:04:00:04:00:32:00:00:7b:00

        // Macro instructions
        // 2021_06_05_23_36_set_right_ctrl_macro_shift_key.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:03:03:3b:68:01:00:00:00
        // 2021_06_05_23_36_set_right_ctrl_macro_shift_key_play_multiple_times_twice.pcap
        // 00:1f:00:00:00:0a:02:0d:01:40:00:03:03:3b:68:02:00:00:00
        // 2021_06_05_23_37_set_right_ctrl_macro_shift_key_play_multiple_times_5.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:03:03:3b:68:05:00:00:00
        //                                              ^^ amount of times?

        //                                        ^^ ^^ macro address??

        // 2021_06_05_23_37_set_right_ctrl_macro_shift_key_play_assigned_pressed.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:04:02:3b:68:00:00:00:00
        //                                  ^^ while pressed?

        // 2021_06_05_23_37_set_right_ctrl_macro_shift_key_play_toggle.pcapng
        // 00:1f:00:00:00:0a:02:0d:01:40:00:05:02:3b:68:00:00:00:00
        //                                  ^^ Toggle?

        // 2021_06_05_23_36_set_right_ctrl_macro_shift_key holds macro addition.
        // 00:1f:00:00:00:0a:02:0d:01:40:00:03:03:3b:68:01:00:16:00
        // [profile data event] [Done] index[0] input type[KEYBOARD] flag[2] makecode[29] modifier[0x0] mapping type[MACRO] guid[B13069EB-E654-49E2-8B6C-204137CC4786]

        // 2021_06_05_23_50_set_right_ctrl_volume_mute
        // [Done] index[0] input type[KEYBOARD] flag[2] makecode[29] modifier[0x0] mapping type[MULTIMEDIA] action type[5]
        // 02:1f:00:00:00:0a:02:0d:01:40:00:0a:02:00:e2:00:00:ae:00
        //                                           ^^ e2 is Consumer Page (0x0C) Mute instruction.

        // 2021_06_05_23_50_set_right_ctrl_volume_down
        // index[1] input type[KEYBOARD] flag[2] makecode[29] modifier[0x0] mapping type[MULTIMEDIA] action type[4]
        // 00:1f:00:00:00:0a:02:0d:01:40:00:0a:02:00:ea:00:a6:00
        //                                           ^^ ea is Consumer Page (0x0C) Volume Decrement

        // Lets structure this a bit;
        // 00:1f:00:00:00:0a:02:0d:01:40:HH:CC:NI
        //                         ^^  01 seems to denote currently active, otherwise 02 or 03 for profile
        //                            ^^ AT-101 Position
        // HH: Hypershift
        // CC; Class, we have seen:
        //                                  00 disabled, always followed by zeros
        //                                  01 Mouse, simple?
        //                                  02 Keyboard, simple?
        //                                  03 Macro, 'n' fire
        //                                  04 Macro, while pressed
        //                                  05 Macro, toggle
        //                                  0a Multimedia (Consumer Page? 0x0c?)
        //                                  0b Mouse, doubleclick (button page 0x09?)
        //                                  0e Mouse, left click with turbo
        //                                  0d Single Key Turbo.
        //                                  11 (Obtained through READ; magical keys?)
        //
        //                                  xx Expected; generic desktop page 0x01 for system sleep 0x82
        // NI; No idea; often same as CC, but not always
        // NI is the number of bytes that follows to describe the instruction by the looks of it?

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
}
