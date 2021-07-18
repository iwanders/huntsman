pub use crate::commands::mappings::{Key, KeyMapping, KeyboardKey, Modifier, Modifiers};
use crate::configuration::KeyConfig;

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

pub fn get_default_keymap(key: Key) -> KeyMapping
{
    let k = if key.hypershift { hypershift_keymaps() } else { default_keymaps() };
    for z in k.iter()
    {
        if key.id == z.key.id
        {
            return z.mapping;
        }
    }
    panic!("Could not find default key for {:?}", key);

}

pub fn default_keymaps() -> &'static [KeyConfig] {
    const DEFAULT_KEYMAPS: [KeyConfig; 102] = [
        KeyConfig {
            profile: None,
            key: Key {
                id: 1,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 53,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 2,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 30,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 3,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 31,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 4,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 32,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 5,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 33,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 6,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 34,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 7,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 35,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 8,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 36,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 9,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 37,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 10,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 38,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 11,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 39,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 12,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 45,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 13,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 46,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 15,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 42,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 16,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 43,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 17,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 20,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 18,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 26,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 19,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 8,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 20,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 21,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 21,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 23,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 22,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 28,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 23,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 24,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 24,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 12,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 25,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 18,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 26,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 19,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 27,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 47,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 28,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 48,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 29,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 49,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 30,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 57,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 31,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 4,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 32,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 22,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 33,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 7,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 34,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 9,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 35,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 10,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 36,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 11,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 37,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 13,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 38,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 14,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 39,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 15,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 40,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 51,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 41,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 52,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 43,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 40,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 44,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::LeftShift),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 46,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 29,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 47,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 27,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 48,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 6,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 49,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 25,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 50,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 5,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 51,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 17,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 52,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 16,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 53,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 54,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 54,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 55,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 55,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 56,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 57,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::RightShift),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 58,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::LeftControl),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 60,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::LeftAlt),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 61,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 44,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 62,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::RightAlt),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 64,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::RightControl),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 75,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 73,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 76,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 76,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 79,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 80,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 80,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 74,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 81,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 77,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 83,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 82,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 84,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 81,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 85,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 75,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 86,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 78,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 89,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 79,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 90,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 83,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 91,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 95,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 92,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 92,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 93,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 89,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 95,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 84,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 96,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 96,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 97,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 93,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 98,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 90,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 99,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 98,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 100,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 85,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 101,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 97,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 102,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 94,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 103,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 91,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 104,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 99,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 105,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 86,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 106,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 87,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 108,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 88,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 110,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 41,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 112,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 58,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 113,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 59,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 114,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 60,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 115,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 61,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 116,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 62,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 117,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 63,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 118,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 64,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 119,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 65,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 120,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 66,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 121,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 67,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 122,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 68,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 123,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 69,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 124,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 70,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 125,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 71,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 126,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 72,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 129,
                hypershift: false,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 101,
                modifiers: Modifiers::none(),
            }),
        },
    ];

    &DEFAULT_KEYMAPS
}

pub fn hypershift_keymaps() -> &'static [KeyConfig] {
    const HYPERSHIFT_KEYMAPS: [KeyConfig; 102] = [
        KeyConfig {
            profile: None,
            key: Key {
                id: 1,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 53,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 2,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 30,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 3,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 31,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 4,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 32,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 5,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 33,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 6,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 34,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 7,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 35,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 8,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 36,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 9,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 37,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 10,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 38,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 11,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 39,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 12,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 45,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 13,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 46,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 15,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 42,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 16,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 43,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 17,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 20,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 18,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 26,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 19,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 8,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 20,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 21,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 21,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 23,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 22,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 28,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 23,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 24,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 24,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 12,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 25,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 18,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 26,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 19,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 27,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 47,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 28,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 48,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 29,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 49,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 30,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 57,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 31,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 4,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 32,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 22,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 33,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 7,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 34,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 9,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 35,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 10,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 36,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 11,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 37,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 13,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 38,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 14,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 39,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 15,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 40,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 51,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 41,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 52,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 43,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 40,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 44,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::LeftShift),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 46,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 29,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 47,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 27,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 48,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 6,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 49,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 25,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 50,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 5,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 51,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 17,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 52,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 16,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 53,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 54,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 54,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 55,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 55,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 56,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 57,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::RightShift),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 58,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::LeftControl),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 60,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::LeftAlt),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 61,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 44,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 62,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::RightAlt),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 64,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 0,
                modifiers: Modifiers::with(Modifier::RightControl),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 75,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 73,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 76,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 76,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 79,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 80,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 80,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 74,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 81,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 77,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 83,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 82,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 84,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 81,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 85,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 75,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 86,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 78,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 89,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 79,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 90,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 83,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 91,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 95,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 92,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 92,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 93,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 89,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 95,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 84,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 96,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 96,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 97,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 93,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 98,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 90,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 99,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 98,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 100,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 85,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 101,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 97,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 102,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 94,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 103,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 91,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 104,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 99,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 105,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 86,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 106,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 87,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 108,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 88,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 110,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 41,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 112,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 58,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 113,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 59,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 114,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 60,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 115,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 61,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 116,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 62,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 117,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 63,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 118,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 64,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 119,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 65,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 120,
                hypershift: true,
            },
            mapping: KeyMapping::Special(4),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 121,
                hypershift: true,
            },
            mapping: KeyMapping::Special(3),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 122,
                hypershift: true,
            },
            mapping: KeyMapping::Special(9),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 123,
                hypershift: true,
            },
            mapping: KeyMapping::Special(8),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 124,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 70,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 125,
                hypershift: true,
            },
            mapping: KeyMapping::Key(KeyboardKey {
                id: 71,
                modifiers: Modifiers::none(),
            }),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 126,
                hypershift: true,
            },
            mapping: KeyMapping::GenericDesktop(130),
        },
        KeyConfig {
            profile: None,
            key: Key {
                id: 129,
                hypershift: true,
            },
            mapping: KeyMapping::ProfileInstruction(4),
        },
    ];
    &HYPERSHIFT_KEYMAPS
}
