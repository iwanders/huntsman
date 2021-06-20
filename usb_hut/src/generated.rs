#[allow(dead_code)]
// This file is generated with dev/generate.rs
use crate::defs::Key;

// keys:
pub mod hid_keys {
    #[allow(dead_code)]
    use crate::defs::{Key, Usage};

    pub const KEY_ERRORROLLOVER: Key = Key {
        hid: 0x01,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard ErrorRollOver",
    };

    pub const KEY_POSTFAIL: Key = Key {
        hid: 0x02,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard POSTFail",
    };

    pub const KEY_ERRORUNDEFINED: Key = Key {
        hid: 0x03,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard ErrorUndefined",
    };

    pub const KEY_A: Key = Key {
        hid: 0x04,
        at101: Some(31),
        usage: Usage::Selector,
        desc: "Keyboard a and A",
    };

    pub const KEY_B: Key = Key {
        hid: 0x05,
        at101: Some(50),
        usage: Usage::Selector,
        desc: "Keyboard b and B",
    };

    pub const KEY_C: Key = Key {
        hid: 0x06,
        at101: Some(48),
        usage: Usage::Selector,
        desc: "Keyboard c and C",
    };

    pub const KEY_D: Key = Key {
        hid: 0x07,
        at101: Some(33),
        usage: Usage::Selector,
        desc: "Keyboard d and D",
    };

    pub const KEY_E: Key = Key {
        hid: 0x08,
        at101: Some(19),
        usage: Usage::Selector,
        desc: "Keyboard e and E",
    };

    pub const KEY_F: Key = Key {
        hid: 0x09,
        at101: Some(34),
        usage: Usage::Selector,
        desc: "Keyboard f and F",
    };

    pub const KEY_G: Key = Key {
        hid: 0x0a,
        at101: Some(35),
        usage: Usage::Selector,
        desc: "Keyboard g and G",
    };

    pub const KEY_H: Key = Key {
        hid: 0x0b,
        at101: Some(36),
        usage: Usage::Selector,
        desc: "Keyboard h and H",
    };

    pub const KEY_I: Key = Key {
        hid: 0x0c,
        at101: Some(24),
        usage: Usage::Selector,
        desc: "Keyboard i and I",
    };

    pub const KEY_J: Key = Key {
        hid: 0x0d,
        at101: Some(37),
        usage: Usage::Selector,
        desc: "Keyboard j and J",
    };

    pub const KEY_K: Key = Key {
        hid: 0x0e,
        at101: Some(38),
        usage: Usage::Selector,
        desc: "Keyboard k and K",
    };

    pub const KEY_L: Key = Key {
        hid: 0x0f,
        at101: Some(39),
        usage: Usage::Selector,
        desc: "Keyboard l and L",
    };

    pub const KEY_M: Key = Key {
        hid: 0x10,
        at101: Some(52),
        usage: Usage::Selector,
        desc: "Keyboard m and M",
    };

    pub const KEY_N: Key = Key {
        hid: 0x11,
        at101: Some(51),
        usage: Usage::Selector,
        desc: "Keyboard n and N",
    };

    pub const KEY_O: Key = Key {
        hid: 0x12,
        at101: Some(25),
        usage: Usage::Selector,
        desc: "Keyboard o and O",
    };

    pub const KEY_P: Key = Key {
        hid: 0x13,
        at101: Some(26),
        usage: Usage::Selector,
        desc: "Keyboard p and P",
    };

    pub const KEY_Q: Key = Key {
        hid: 0x14,
        at101: Some(17),
        usage: Usage::Selector,
        desc: "Keyboard q and Q",
    };

    pub const KEY_R: Key = Key {
        hid: 0x15,
        at101: Some(20),
        usage: Usage::Selector,
        desc: "Keyboard r and R",
    };

    pub const KEY_S: Key = Key {
        hid: 0x16,
        at101: Some(32),
        usage: Usage::Selector,
        desc: "Keyboard s and S",
    };

    pub const KEY_T: Key = Key {
        hid: 0x17,
        at101: Some(21),
        usage: Usage::Selector,
        desc: "Keyboard t and T",
    };

    pub const KEY_U: Key = Key {
        hid: 0x18,
        at101: Some(23),
        usage: Usage::Selector,
        desc: "Keyboard u and U",
    };

    pub const KEY_V: Key = Key {
        hid: 0x19,
        at101: Some(49),
        usage: Usage::Selector,
        desc: "Keyboard v and V",
    };

    pub const KEY_W: Key = Key {
        hid: 0x1a,
        at101: Some(18),
        usage: Usage::Selector,
        desc: "Keyboard w and W",
    };

    pub const KEY_X: Key = Key {
        hid: 0x1b,
        at101: Some(47),
        usage: Usage::Selector,
        desc: "Keyboard x and X",
    };

    pub const KEY_Y: Key = Key {
        hid: 0x1c,
        at101: Some(22),
        usage: Usage::Selector,
        desc: "Keyboard y and Y",
    };

    pub const KEY_Z: Key = Key {
        hid: 0x1d,
        at101: Some(46),
        usage: Usage::Selector,
        desc: "Keyboard z and Z",
    };

    pub const KEY_1: Key = Key {
        hid: 0x1e,
        at101: Some(2),
        usage: Usage::Selector,
        desc: "Keyboard 1 and !",
    };

    pub const KEY_2: Key = Key {
        hid: 0x1f,
        at101: Some(3),
        usage: Usage::Selector,
        desc: "Keyboard 2 and @",
    };

    pub const KEY_3: Key = Key {
        hid: 0x20,
        at101: Some(4),
        usage: Usage::Selector,
        desc: "Keyboard 3 and #",
    };

    pub const KEY_4: Key = Key {
        hid: 0x21,
        at101: Some(5),
        usage: Usage::Selector,
        desc: "Keyboard 4 and $",
    };

    pub const KEY_5: Key = Key {
        hid: 0x22,
        at101: Some(6),
        usage: Usage::Selector,
        desc: "Keyboard 5 and %",
    };

    pub const KEY_6: Key = Key {
        hid: 0x23,
        at101: Some(7),
        usage: Usage::Selector,
        desc: "Keyboard 6 and",
    };

    pub const KEY_7: Key = Key {
        hid: 0x24,
        at101: Some(8),
        usage: Usage::Selector,
        desc: "Keyboard 7 and &",
    };

    pub const KEY_8: Key = Key {
        hid: 0x25,
        at101: Some(9),
        usage: Usage::Selector,
        desc: "Keyboard 8 and *",
    };

    pub const KEY_9: Key = Key {
        hid: 0x26,
        at101: Some(10),
        usage: Usage::Selector,
        desc: "Keyboard 9 and (",
    };

    pub const KEY_0: Key = Key {
        hid: 0x27,
        at101: Some(11),
        usage: Usage::Selector,
        desc: "Keyboard 0 and )",
    };

    pub const KEY_ENTER: Key = Key {
        hid: 0x28,
        at101: Some(43),
        usage: Usage::Selector,
        desc: "Keyboard Return (ENTER)",
    };

    pub const KEY_ESC: Key = Key {
        hid: 0x29,
        at101: Some(110),
        usage: Usage::Selector,
        desc: "Keyboard ESCAPE",
    };

    pub const KEY_BACKSPACE: Key = Key {
        hid: 0x2a,
        at101: Some(15),
        usage: Usage::Selector,
        desc: "Keyboard DELETE (Backspace)",
    };

    pub const KEY_TAB: Key = Key {
        hid: 0x2b,
        at101: Some(16),
        usage: Usage::Selector,
        desc: "Keyboard Tab",
    };

    pub const KEY_SPACE: Key = Key {
        hid: 0x2c,
        at101: Some(61),
        usage: Usage::Selector,
        desc: "Keyboard Spacebar",
    };

    pub const KEY_DASH: Key = Key {
        hid: 0x2d,
        at101: Some(12),
        usage: Usage::Selector,
        desc: "Keyboard - and (underscore)",
    };

    pub const KEY_EQUAL: Key = Key {
        hid: 0x2e,
        at101: Some(13),
        usage: Usage::Selector,
        desc: "Keyboard = and +",
    };

    pub const KEY_LEFT_BRACKET: Key = Key {
        hid: 0x2f,
        at101: Some(27),
        usage: Usage::Selector,
        desc: "Keyboard [ and",
    };

    pub const KEY_RIGHT_BRACKET: Key = Key {
        hid: 0x30,
        at101: Some(28),
        usage: Usage::Selector,
        desc: "Keyboard ] and }",
    };

    pub const KEY_BACKSLASH_AND_PIPE: Key = Key {
        hid: 0x31,
        at101: Some(29),
        usage: Usage::Selector,
        desc: "Keyboard \\and |",
    };

    pub const KEY_NON_US_HASH_AND_: Key = Key {
        hid: 0x32,
        at101: Some(42),
        usage: Usage::Selector,
        desc: "Keyboard Non-US # and ",
    };

    pub const KEY_SEMICOLON: Key = Key {
        hid: 0x33,
        at101: Some(40),
        usage: Usage::Selector,
        desc: "Keyboard ; and :",
    };

    pub const KEY_QUOTE: Key = Key {
        hid: 0x34,
        at101: Some(41),
        usage: Usage::Selector,
        desc: "Keyboard ‘ and “",
    };

    pub const KEY_GRAVE_ACCENT_AND_TILDE: Key = Key {
        hid: 0x35,
        at101: Some(1),
        usage: Usage::Selector,
        desc: "Keyboard Grave Accent and Tilde",
    };

    pub const KEY_COMMA: Key = Key {
        hid: 0x36,
        at101: Some(53),
        usage: Usage::Selector,
        desc: "Keyboard , and <",
    };

    pub const KEY_DOT: Key = Key {
        hid: 0x37,
        at101: Some(54),
        usage: Usage::Selector,
        desc: "Keyboard . and >",
    };

    pub const KEY_SLASH: Key = Key {
        hid: 0x38,
        at101: Some(55),
        usage: Usage::Selector,
        desc: "Keyboard / and ?",
    };

    pub const KEY_CAPS_LOCK: Key = Key {
        hid: 0x39,
        at101: Some(30),
        usage: Usage::Selector,
        desc: "Keyboard Caps Lock",
    };

    pub const KEY_F1: Key = Key {
        hid: 0x3a,
        at101: Some(112),
        usage: Usage::Selector,
        desc: "Keyboard F1",
    };

    pub const KEY_F2: Key = Key {
        hid: 0x3b,
        at101: Some(113),
        usage: Usage::Selector,
        desc: "Keyboard F2",
    };

    pub const KEY_F3: Key = Key {
        hid: 0x3c,
        at101: Some(114),
        usage: Usage::Selector,
        desc: "Keyboard F3",
    };

    pub const KEY_F4: Key = Key {
        hid: 0x3d,
        at101: Some(115),
        usage: Usage::Selector,
        desc: "Keyboard F4",
    };

    pub const KEY_F5: Key = Key {
        hid: 0x3e,
        at101: Some(116),
        usage: Usage::Selector,
        desc: "Keyboard F5",
    };

    pub const KEY_F6: Key = Key {
        hid: 0x3f,
        at101: Some(117),
        usage: Usage::Selector,
        desc: "Keyboard F6",
    };

    pub const KEY_F7: Key = Key {
        hid: 0x40,
        at101: Some(118),
        usage: Usage::Selector,
        desc: "Keyboard F7",
    };

    pub const KEY_F8: Key = Key {
        hid: 0x41,
        at101: Some(119),
        usage: Usage::Selector,
        desc: "Keyboard F8",
    };

    pub const KEY_F9: Key = Key {
        hid: 0x42,
        at101: Some(120),
        usage: Usage::Selector,
        desc: "Keyboard F9",
    };

    pub const KEY_F10: Key = Key {
        hid: 0x43,
        at101: Some(121),
        usage: Usage::Selector,
        desc: "Keyboard F10",
    };

    pub const KEY_F11: Key = Key {
        hid: 0x44,
        at101: Some(122),
        usage: Usage::Selector,
        desc: "Keyboard F11",
    };

    pub const KEY_F12: Key = Key {
        hid: 0x45,
        at101: Some(123),
        usage: Usage::Selector,
        desc: "Keyboard F12",
    };

    pub const KEY_PRINTSCREEN: Key = Key {
        hid: 0x46,
        at101: Some(124),
        usage: Usage::Selector,
        desc: "Keyboard PrintScreen",
    };

    pub const KEY_SCROLL_LOCK: Key = Key {
        hid: 0x47,
        at101: Some(125),
        usage: Usage::Selector,
        desc: "Keyboard Scroll Lock",
    };

    pub const KEY_PAUSE: Key = Key {
        hid: 0x48,
        at101: Some(126),
        usage: Usage::Selector,
        desc: "Keyboard Pause",
    };

    pub const KEY_INSERT: Key = Key {
        hid: 0x49,
        at101: Some(75),
        usage: Usage::Selector,
        desc: "Keyboard Insert",
    };

    pub const KEY_HOME: Key = Key {
        hid: 0x4a,
        at101: Some(80),
        usage: Usage::Selector,
        desc: "Keyboard Home",
    };

    pub const KEY_PAGEUP: Key = Key {
        hid: 0x4b,
        at101: Some(85),
        usage: Usage::Selector,
        desc: "Keyboard PageUp",
    };

    pub const KEY_DELETE_FORWARD: Key = Key {
        hid: 0x4c,
        at101: Some(76),
        usage: Usage::Selector,
        desc: "Keyboard Delete Forward",
    };

    pub const KEY_END: Key = Key {
        hid: 0x4d,
        at101: Some(81),
        usage: Usage::Selector,
        desc: "Keyboard End",
    };

    pub const KEY_PAGEDOWN: Key = Key {
        hid: 0x4e,
        at101: Some(86),
        usage: Usage::Selector,
        desc: "Keyboard PageDown",
    };

    pub const KEY_RIGHT_ARROW: Key = Key {
        hid: 0x4f,
        at101: Some(89),
        usage: Usage::Selector,
        desc: "Keyboard Right Arrow",
    };

    pub const KEY_LEFT_ARROW: Key = Key {
        hid: 0x50,
        at101: Some(79),
        usage: Usage::Selector,
        desc: "Keyboard Left Arrow",
    };

    pub const KEY_DOWN_ARROW: Key = Key {
        hid: 0x51,
        at101: Some(84),
        usage: Usage::Selector,
        desc: "Keyboard Down Arrow",
    };

    pub const KEY_UP_ARROW: Key = Key {
        hid: 0x52,
        at101: Some(83),
        usage: Usage::Selector,
        desc: "Keyboard Up Arrow",
    };

    pub const KEY_KPD_NUM_LOCK_AND_CLEAR: Key = Key {
        hid: 0x53,
        at101: Some(90),
        usage: Usage::Selector,
        desc: "Keypad Num Lock and Clear",
    };

    pub const KEY_KPD_SLASH: Key = Key {
        hid: 0x54,
        at101: Some(95),
        usage: Usage::Selector,
        desc: "Keypad /",
    };

    pub const KEY_KPD_ASTERISK: Key = Key {
        hid: 0x55,
        at101: Some(100),
        usage: Usage::Selector,
        desc: "Keypad *",
    };

    pub const KEY_KPD_DASH: Key = Key {
        hid: 0x56,
        at101: Some(105),
        usage: Usage::Selector,
        desc: "Keypad -",
    };

    pub const KEY_KPD_PLUS: Key = Key {
        hid: 0x57,
        at101: Some(106),
        usage: Usage::Selector,
        desc: "Keypad +",
    };

    pub const KEY_KPD_ENTER: Key = Key {
        hid: 0x58,
        at101: Some(108),
        usage: Usage::Selector,
        desc: "Keypad ENTER",
    };

    pub const KEY_KPD_1: Key = Key {
        hid: 0x59,
        at101: Some(93),
        usage: Usage::Selector,
        desc: "Keypad 1 and End",
    };

    pub const KEY_KPD_2: Key = Key {
        hid: 0x5a,
        at101: Some(98),
        usage: Usage::Selector,
        desc: "Keypad 2 and Down Arrow",
    };

    pub const KEY_KPD_3: Key = Key {
        hid: 0x5b,
        at101: Some(103),
        usage: Usage::Selector,
        desc: "Keypad 3 and PageDn",
    };

    pub const KEY_KPD_4: Key = Key {
        hid: 0x5c,
        at101: Some(92),
        usage: Usage::Selector,
        desc: "Keypad 4 and Left Arrow",
    };

    pub const KEY_KPD_5: Key = Key {
        hid: 0x5d,
        at101: Some(97),
        usage: Usage::Selector,
        desc: "Keypad 5",
    };

    pub const KEY_KPD_6: Key = Key {
        hid: 0x5e,
        at101: Some(102),
        usage: Usage::Selector,
        desc: "Keypad 6 and Right Arrow",
    };

    pub const KEY_KPD_7: Key = Key {
        hid: 0x5f,
        at101: Some(91),
        usage: Usage::Selector,
        desc: "Keypad 7 and Home",
    };

    pub const KEY_KPD_8: Key = Key {
        hid: 0x60,
        at101: Some(96),
        usage: Usage::Selector,
        desc: "Keypad 8 and Up Arrow",
    };

    pub const KEY_KPD_9: Key = Key {
        hid: 0x61,
        at101: Some(101),
        usage: Usage::Selector,
        desc: "Keypad 9 and PageUp",
    };

    pub const KEY_KPD_0: Key = Key {
        hid: 0x62,
        at101: Some(99),
        usage: Usage::Selector,
        desc: "Keypad 0 and Insert",
    };

    pub const KEY_KPD_DOT: Key = Key {
        hid: 0x63,
        at101: Some(104),
        usage: Usage::Selector,
        desc: "Keypad . and Delete",
    };

    pub const KEY_NON_US_BACKSLASH_AND_PIPE: Key = Key {
        hid: 0x64,
        at101: Some(45),
        usage: Usage::Selector,
        desc: "Keyboard Non-US \\and |",
    };

    pub const KEY_APPLICATION: Key = Key {
        hid: 0x65,
        at101: Some(129),
        usage: Usage::Selector,
        desc: "Keyboard Application",
    };

    pub const KEY_POWER: Key = Key {
        hid: 0x66,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Power",
    };

    pub const KEY_KPD_EQUAL: Key = Key {
        hid: 0x67,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad =",
    };

    pub const KEY_F13: Key = Key {
        hid: 0x68,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F13",
    };

    pub const KEY_F14: Key = Key {
        hid: 0x69,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F14",
    };

    pub const KEY_F15: Key = Key {
        hid: 0x6a,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F15",
    };

    pub const KEY_F16: Key = Key {
        hid: 0x6b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F16",
    };

    pub const KEY_F17: Key = Key {
        hid: 0x6c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F17",
    };

    pub const KEY_F18: Key = Key {
        hid: 0x6d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F18",
    };

    pub const KEY_F19: Key = Key {
        hid: 0x6e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F19",
    };

    pub const KEY_F20: Key = Key {
        hid: 0x6f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F20",
    };

    pub const KEY_F21: Key = Key {
        hid: 0x70,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F21",
    };

    pub const KEY_F22: Key = Key {
        hid: 0x71,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F22",
    };

    pub const KEY_F23: Key = Key {
        hid: 0x72,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F23",
    };

    pub const KEY_F24: Key = Key {
        hid: 0x73,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F24",
    };

    pub const KEY_EXECUTE: Key = Key {
        hid: 0x74,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Execute",
    };

    pub const KEY_HELP: Key = Key {
        hid: 0x75,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Help",
    };

    pub const KEY_MENU: Key = Key {
        hid: 0x76,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Menu",
    };

    pub const KEY_SELECT: Key = Key {
        hid: 0x77,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Select",
    };

    pub const KEY_STOP: Key = Key {
        hid: 0x78,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Stop",
    };

    pub const KEY_AGAIN: Key = Key {
        hid: 0x79,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Again",
    };

    pub const KEY_UNDO: Key = Key {
        hid: 0x7a,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Undo",
    };

    pub const KEY_CUT: Key = Key {
        hid: 0x7b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Cut",
    };

    pub const KEY_COPY: Key = Key {
        hid: 0x7c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Copy",
    };

    pub const KEY_PASTE: Key = Key {
        hid: 0x7d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Paste",
    };

    pub const KEY_FIND: Key = Key {
        hid: 0x7e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Find",
    };

    pub const KEY_MUTE: Key = Key {
        hid: 0x7f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Mute",
    };

    pub const KEY_VOLUME_UP: Key = Key {
        hid: 0x80,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Volume Up",
    };

    pub const KEY_VOLUME_DOWN: Key = Key {
        hid: 0x81,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Volume Down",
    };

    pub const KEY_LOCKING_CAPS_LOCK: Key = Key {
        hid: 0x82,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Locking Caps Lock",
    };

    pub const KEY_LOCKING_NUM_LOCK: Key = Key {
        hid: 0x83,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Locking Num Lock",
    };

    pub const KEY_LOCKING_SCROLL: Key = Key {
        hid: 0x84,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Locking Scroll",
    };

    pub const KEY_KPD_COMMA: Key = Key {
        hid: 0x85,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Comma",
    };

    pub const KEY_INTERNATIONAL1: Key = Key {
        hid: 0x87,
        at101: Some(56),
        usage: Usage::Selector,
        desc: "Keyboard International1",
    };

    pub const KEY_INTERNATIONAL2: Key = Key {
        hid: 0x88,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International2",
    };

    pub const KEY_INTERNATIONAL3: Key = Key {
        hid: 0x89,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International3",
    };

    pub const KEY_INTERNATIONAL4: Key = Key {
        hid: 0x8a,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International4",
    };

    pub const KEY_INTERNATIONAL5: Key = Key {
        hid: 0x8b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International5",
    };

    pub const KEY_INTERNATIONAL6: Key = Key {
        hid: 0x8c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International6",
    };

    pub const KEY_INTERNATIONAL7: Key = Key {
        hid: 0x8d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International7",
    };

    pub const KEY_INTERNATIONAL8: Key = Key {
        hid: 0x8e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International8",
    };

    pub const KEY_INTERNATIONAL9: Key = Key {
        hid: 0x8f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International9",
    };

    pub const KEY_LANG1: Key = Key {
        hid: 0x90,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG1",
    };

    pub const KEY_LANG2: Key = Key {
        hid: 0x91,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG2",
    };

    pub const KEY_LANG3: Key = Key {
        hid: 0x92,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG3",
    };

    pub const KEY_LANG4: Key = Key {
        hid: 0x93,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG4",
    };

    pub const KEY_LANG5: Key = Key {
        hid: 0x94,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG5",
    };

    pub const KEY_LANG6: Key = Key {
        hid: 0x95,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG6",
    };

    pub const KEY_LANG7: Key = Key {
        hid: 0x96,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG7",
    };

    pub const KEY_LANG8: Key = Key {
        hid: 0x97,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG8",
    };

    pub const KEY_LANG9: Key = Key {
        hid: 0x98,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG9",
    };

    pub const KEY_CANCEL: Key = Key {
        hid: 0x9b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Cancel",
    };

    pub const KEY_CLEAR: Key = Key {
        hid: 0x9c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Clear",
    };

    pub const KEY_PRIOR: Key = Key {
        hid: 0x9d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Prior",
    };

    pub const KEY_RETURN: Key = Key {
        hid: 0x9e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Return",
    };

    pub const KEY_SEPARATOR: Key = Key {
        hid: 0x9f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Separator",
    };

    pub const KEY_OUT: Key = Key {
        hid: 0xa0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Out",
    };

    pub const KEY_OPER: Key = Key {
        hid: 0xa1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Oper",
    };

    pub const KEY_CLEAR_AGAIN: Key = Key {
        hid: 0xa2,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Clear/Again",
    };

    pub const KEY_CRSEL_PROPS: Key = Key {
        hid: 0xa3,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard CrSel/Props",
    };

    pub const KEY_EXSEL: Key = Key {
        hid: 0xa4,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard ExSel",
    };

    pub const KEY_KPD_00: Key = Key {
        hid: 0xb0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad 00",
    };

    pub const KEY_KPD_000: Key = Key {
        hid: 0xb1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad 000",
    };

    pub const KEY_THOUSANDS_SEPARATOR: Key = Key {
        hid: 0xb2,
        at101: None,
        usage: Usage::Selector,
        desc: "Thousands Separator",
    };

    pub const KEY_DECIMAL_SEPARATOR: Key = Key {
        hid: 0xb3,
        at101: None,
        usage: Usage::Selector,
        desc: "Decimal Separator",
    };

    pub const KEY_CURRENCY_UNIT: Key = Key {
        hid: 0xb4,
        at101: None,
        usage: Usage::Selector,
        desc: "Currency Unit",
    };

    pub const KEY_CURRENCY_SUBUNIT: Key = Key {
        hid: 0xb5,
        at101: None,
        usage: Usage::Selector,
        desc: "Currency Sub-unit",
    };

    pub const KEY_KPD_LEFT_PARENTHESIS: Key = Key {
        hid: 0xb6,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad (",
    };

    pub const KEY_KPD_RIGHT_PARENTHESIS: Key = Key {
        hid: 0xb7,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad )",
    };

    pub const KEY_KPD_LEFT_BRACE: Key = Key {
        hid: 0xb8,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad {",
    };

    pub const KEY_KPD_RIGHT_BRACE: Key = Key {
        hid: 0xb9,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad }",
    };

    pub const KEY_KPD_TAB: Key = Key {
        hid: 0xba,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Tab",
    };

    pub const KEY_KPD_BACKSPACE: Key = Key {
        hid: 0xbb,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Backspace",
    };

    pub const KEY_KPD_A: Key = Key {
        hid: 0xbc,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad A",
    };

    pub const KEY_KPD_B: Key = Key {
        hid: 0xbd,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad B",
    };

    pub const KEY_KPD_C: Key = Key {
        hid: 0xbe,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad C",
    };

    pub const KEY_KPD_D: Key = Key {
        hid: 0xbf,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad D",
    };

    pub const KEY_KPD_E: Key = Key {
        hid: 0xc0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad E",
    };

    pub const KEY_KPD_F: Key = Key {
        hid: 0xc1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad F",
    };

    pub const KEY_KPD_XOR: Key = Key {
        hid: 0xc2,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad XOR",
    };

    pub const KEY_KPD_CARET: Key = Key {
        hid: 0xc3,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad ^",
    };

    pub const KEY_KPD_PCT: Key = Key {
        hid: 0xc4,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad %",
    };

    pub const KEY_KPD_LT: Key = Key {
        hid: 0xc5,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad <",
    };

    pub const KEY_KPD_GT: Key = Key {
        hid: 0xc6,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad >",
    };

    pub const KEY_KPD_AMPERSAND: Key = Key {
        hid: 0xc7,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad &",
    };

    pub const KEY_KPD_DOUBLEAMPERSAND: Key = Key {
        hid: 0xc8,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad &&",
    };

    pub const KEY_KPD_PIPE: Key = Key {
        hid: 0xc9,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad |",
    };

    pub const KEY_KPD_DOUBLEPIPE: Key = Key {
        hid: 0xca,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad ||",
    };

    pub const KEY_KPD_COLON: Key = Key {
        hid: 0xcb,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad :",
    };

    pub const KEY_KPD_HASH: Key = Key {
        hid: 0xcc,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad #",
    };

    pub const KEY_KPD_SPACE: Key = Key {
        hid: 0xcd,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Space",
    };

    pub const KEY_KPD_AT: Key = Key {
        hid: 0xce,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad @",
    };

    pub const KEY_KPD_EXCLAMATION: Key = Key {
        hid: 0xcf,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad !",
    };

    pub const KEY_KPD_MEMORY_STORE: Key = Key {
        hid: 0xd0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Store",
    };

    pub const KEY_KPD_MEMORY_RECALL: Key = Key {
        hid: 0xd1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Recall",
    };

    pub const KEY_KPD_MEMORY_CLEAR: Key = Key {
        hid: 0xd2,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Clear",
    };

    pub const KEY_KPD_MEMORY_ADD: Key = Key {
        hid: 0xd3,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Add",
    };

    pub const KEY_KPD_MEMORY_SUBTRACT: Key = Key {
        hid: 0xd4,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Subtract",
    };

    pub const KEY_KPD_MEMORY_MULTIPLY: Key = Key {
        hid: 0xd5,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Multiply",
    };

    pub const KEY_KPD_MEMORY_DIVIDE: Key = Key {
        hid: 0xd6,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Divide",
    };

    pub const KEY_KPD_PLUSMINUS: Key = Key {
        hid: 0xd7,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad +/-",
    };

    pub const KEY_KPD_CLEAR: Key = Key {
        hid: 0xd8,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Clear",
    };

    pub const KEY_KPD_CLEAR_ENTRY: Key = Key {
        hid: 0xd9,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Clear Entry",
    };

    pub const KEY_KPD_BINARY: Key = Key {
        hid: 0xda,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Binary",
    };

    pub const KEY_KPD_OCTAL: Key = Key {
        hid: 0xdb,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Octal",
    };

    pub const KEY_KPD_DECIMAL: Key = Key {
        hid: 0xdc,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Decimal",
    };

    pub const KEY_KPD_HEXADECIMAL: Key = Key {
        hid: 0xdd,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Hexadecimal",
    };

    pub const KEY_LEFT_CONTROL: Key = Key {
        hid: 0xe0,
        at101: Some(58),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left Control",
    };

    pub const KEY_LEFT_SHIFT: Key = Key {
        hid: 0xe1,
        at101: Some(44),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left Shift",
    };

    pub const KEY_LEFT_ALT: Key = Key {
        hid: 0xe2,
        at101: Some(60),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left Alt",
    };

    pub const KEY_LEFT_META: Key = Key {
        hid: 0xe3,
        at101: Some(127),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left GUI",
    };

    pub const KEY_RIGHT_CONTROL: Key = Key {
        hid: 0xe4,
        at101: Some(64),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right Control",
    };

    pub const KEY_RIGHT_SHIFT: Key = Key {
        hid: 0xe5,
        at101: Some(57),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right Shift",
    };

    pub const KEY_RIGHT_ALT: Key = Key {
        hid: 0xe6,
        at101: Some(62),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right Alt",
    };

    pub const KEY_RIGHT_META: Key = Key {
        hid: 0xe7,
        at101: Some(128),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right GUI",
    };
}

pub const fn keys() -> [Key; 215] {
    [
        hid_keys::KEY_ERRORROLLOVER,
        hid_keys::KEY_POSTFAIL,
        hid_keys::KEY_ERRORUNDEFINED,
        hid_keys::KEY_A,
        hid_keys::KEY_B,
        hid_keys::KEY_C,
        hid_keys::KEY_D,
        hid_keys::KEY_E,
        hid_keys::KEY_F,
        hid_keys::KEY_G,
        hid_keys::KEY_H,
        hid_keys::KEY_I,
        hid_keys::KEY_J,
        hid_keys::KEY_K,
        hid_keys::KEY_L,
        hid_keys::KEY_M,
        hid_keys::KEY_N,
        hid_keys::KEY_O,
        hid_keys::KEY_P,
        hid_keys::KEY_Q,
        hid_keys::KEY_R,
        hid_keys::KEY_S,
        hid_keys::KEY_T,
        hid_keys::KEY_U,
        hid_keys::KEY_V,
        hid_keys::KEY_W,
        hid_keys::KEY_X,
        hid_keys::KEY_Y,
        hid_keys::KEY_Z,
        hid_keys::KEY_1,
        hid_keys::KEY_2,
        hid_keys::KEY_3,
        hid_keys::KEY_4,
        hid_keys::KEY_5,
        hid_keys::KEY_6,
        hid_keys::KEY_7,
        hid_keys::KEY_8,
        hid_keys::KEY_9,
        hid_keys::KEY_0,
        hid_keys::KEY_ENTER,
        hid_keys::KEY_ESC,
        hid_keys::KEY_BACKSPACE,
        hid_keys::KEY_TAB,
        hid_keys::KEY_SPACE,
        hid_keys::KEY_DASH,
        hid_keys::KEY_EQUAL,
        hid_keys::KEY_LEFT_BRACKET,
        hid_keys::KEY_RIGHT_BRACKET,
        hid_keys::KEY_BACKSLASH_AND_PIPE,
        hid_keys::KEY_NON_US_HASH_AND_,
        hid_keys::KEY_SEMICOLON,
        hid_keys::KEY_QUOTE,
        hid_keys::KEY_GRAVE_ACCENT_AND_TILDE,
        hid_keys::KEY_COMMA,
        hid_keys::KEY_DOT,
        hid_keys::KEY_SLASH,
        hid_keys::KEY_CAPS_LOCK,
        hid_keys::KEY_F1,
        hid_keys::KEY_F2,
        hid_keys::KEY_F3,
        hid_keys::KEY_F4,
        hid_keys::KEY_F5,
        hid_keys::KEY_F6,
        hid_keys::KEY_F7,
        hid_keys::KEY_F8,
        hid_keys::KEY_F9,
        hid_keys::KEY_F10,
        hid_keys::KEY_F11,
        hid_keys::KEY_F12,
        hid_keys::KEY_PRINTSCREEN,
        hid_keys::KEY_SCROLL_LOCK,
        hid_keys::KEY_PAUSE,
        hid_keys::KEY_INSERT,
        hid_keys::KEY_HOME,
        hid_keys::KEY_PAGEUP,
        hid_keys::KEY_DELETE_FORWARD,
        hid_keys::KEY_END,
        hid_keys::KEY_PAGEDOWN,
        hid_keys::KEY_RIGHT_ARROW,
        hid_keys::KEY_LEFT_ARROW,
        hid_keys::KEY_DOWN_ARROW,
        hid_keys::KEY_UP_ARROW,
        hid_keys::KEY_KPD_NUM_LOCK_AND_CLEAR,
        hid_keys::KEY_KPD_SLASH,
        hid_keys::KEY_KPD_ASTERISK,
        hid_keys::KEY_KPD_DASH,
        hid_keys::KEY_KPD_PLUS,
        hid_keys::KEY_KPD_ENTER,
        hid_keys::KEY_KPD_1,
        hid_keys::KEY_KPD_2,
        hid_keys::KEY_KPD_3,
        hid_keys::KEY_KPD_4,
        hid_keys::KEY_KPD_5,
        hid_keys::KEY_KPD_6,
        hid_keys::KEY_KPD_7,
        hid_keys::KEY_KPD_8,
        hid_keys::KEY_KPD_9,
        hid_keys::KEY_KPD_0,
        hid_keys::KEY_KPD_DOT,
        hid_keys::KEY_NON_US_BACKSLASH_AND_PIPE,
        hid_keys::KEY_APPLICATION,
        hid_keys::KEY_POWER,
        hid_keys::KEY_KPD_EQUAL,
        hid_keys::KEY_F13,
        hid_keys::KEY_F14,
        hid_keys::KEY_F15,
        hid_keys::KEY_F16,
        hid_keys::KEY_F17,
        hid_keys::KEY_F18,
        hid_keys::KEY_F19,
        hid_keys::KEY_F20,
        hid_keys::KEY_F21,
        hid_keys::KEY_F22,
        hid_keys::KEY_F23,
        hid_keys::KEY_F24,
        hid_keys::KEY_EXECUTE,
        hid_keys::KEY_HELP,
        hid_keys::KEY_MENU,
        hid_keys::KEY_SELECT,
        hid_keys::KEY_STOP,
        hid_keys::KEY_AGAIN,
        hid_keys::KEY_UNDO,
        hid_keys::KEY_CUT,
        hid_keys::KEY_COPY,
        hid_keys::KEY_PASTE,
        hid_keys::KEY_FIND,
        hid_keys::KEY_MUTE,
        hid_keys::KEY_VOLUME_UP,
        hid_keys::KEY_VOLUME_DOWN,
        hid_keys::KEY_LOCKING_CAPS_LOCK,
        hid_keys::KEY_LOCKING_NUM_LOCK,
        hid_keys::KEY_LOCKING_SCROLL,
        hid_keys::KEY_KPD_COMMA,
        hid_keys::KEY_INTERNATIONAL1,
        hid_keys::KEY_INTERNATIONAL2,
        hid_keys::KEY_INTERNATIONAL3,
        hid_keys::KEY_INTERNATIONAL4,
        hid_keys::KEY_INTERNATIONAL5,
        hid_keys::KEY_INTERNATIONAL6,
        hid_keys::KEY_INTERNATIONAL7,
        hid_keys::KEY_INTERNATIONAL8,
        hid_keys::KEY_INTERNATIONAL9,
        hid_keys::KEY_LANG1,
        hid_keys::KEY_LANG2,
        hid_keys::KEY_LANG3,
        hid_keys::KEY_LANG4,
        hid_keys::KEY_LANG5,
        hid_keys::KEY_LANG6,
        hid_keys::KEY_LANG7,
        hid_keys::KEY_LANG8,
        hid_keys::KEY_LANG9,
        hid_keys::KEY_CANCEL,
        hid_keys::KEY_CLEAR,
        hid_keys::KEY_PRIOR,
        hid_keys::KEY_RETURN,
        hid_keys::KEY_SEPARATOR,
        hid_keys::KEY_OUT,
        hid_keys::KEY_OPER,
        hid_keys::KEY_CLEAR_AGAIN,
        hid_keys::KEY_CRSEL_PROPS,
        hid_keys::KEY_EXSEL,
        hid_keys::KEY_KPD_00,
        hid_keys::KEY_KPD_000,
        hid_keys::KEY_THOUSANDS_SEPARATOR,
        hid_keys::KEY_DECIMAL_SEPARATOR,
        hid_keys::KEY_CURRENCY_UNIT,
        hid_keys::KEY_CURRENCY_SUBUNIT,
        hid_keys::KEY_KPD_LEFT_PARENTHESIS,
        hid_keys::KEY_KPD_RIGHT_PARENTHESIS,
        hid_keys::KEY_KPD_LEFT_BRACE,
        hid_keys::KEY_KPD_RIGHT_BRACE,
        hid_keys::KEY_KPD_TAB,
        hid_keys::KEY_KPD_BACKSPACE,
        hid_keys::KEY_KPD_A,
        hid_keys::KEY_KPD_B,
        hid_keys::KEY_KPD_C,
        hid_keys::KEY_KPD_D,
        hid_keys::KEY_KPD_E,
        hid_keys::KEY_KPD_F,
        hid_keys::KEY_KPD_XOR,
        hid_keys::KEY_KPD_CARET,
        hid_keys::KEY_KPD_PCT,
        hid_keys::KEY_KPD_LT,
        hid_keys::KEY_KPD_GT,
        hid_keys::KEY_KPD_AMPERSAND,
        hid_keys::KEY_KPD_DOUBLEAMPERSAND,
        hid_keys::KEY_KPD_PIPE,
        hid_keys::KEY_KPD_DOUBLEPIPE,
        hid_keys::KEY_KPD_COLON,
        hid_keys::KEY_KPD_HASH,
        hid_keys::KEY_KPD_SPACE,
        hid_keys::KEY_KPD_AT,
        hid_keys::KEY_KPD_EXCLAMATION,
        hid_keys::KEY_KPD_MEMORY_STORE,
        hid_keys::KEY_KPD_MEMORY_RECALL,
        hid_keys::KEY_KPD_MEMORY_CLEAR,
        hid_keys::KEY_KPD_MEMORY_ADD,
        hid_keys::KEY_KPD_MEMORY_SUBTRACT,
        hid_keys::KEY_KPD_MEMORY_MULTIPLY,
        hid_keys::KEY_KPD_MEMORY_DIVIDE,
        hid_keys::KEY_KPD_PLUSMINUS,
        hid_keys::KEY_KPD_CLEAR,
        hid_keys::KEY_KPD_CLEAR_ENTRY,
        hid_keys::KEY_KPD_BINARY,
        hid_keys::KEY_KPD_OCTAL,
        hid_keys::KEY_KPD_DECIMAL,
        hid_keys::KEY_KPD_HEXADECIMAL,
        hid_keys::KEY_LEFT_CONTROL,
        hid_keys::KEY_LEFT_SHIFT,
        hid_keys::KEY_LEFT_ALT,
        hid_keys::KEY_LEFT_META,
        hid_keys::KEY_RIGHT_CONTROL,
        hid_keys::KEY_RIGHT_SHIFT,
        hid_keys::KEY_RIGHT_ALT,
        hid_keys::KEY_RIGHT_META,
    ]
}
