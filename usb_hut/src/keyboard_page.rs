#[allow(dead_code)]
// This file is generated with dev/generate.rs

pub mod hid_keyboard_page {
    #[allow(dead_code)]
    use crate::defs::{Key, Usage};

    pub const KEY_ERRORROLLOVER: Key = Key {
        name: "KEY_ERRORROLLOVER",
        hid: 0x01,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard ErrorRollOver",
    };

    pub const KEY_POSTFAIL: Key = Key {
        name: "KEY_POSTFAIL",
        hid: 0x02,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard POSTFail",
    };

    pub const KEY_ERRORUNDEFINED: Key = Key {
        name: "KEY_ERRORUNDEFINED",
        hid: 0x03,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard ErrorUndefined",
    };

    pub const KEY_A: Key = Key {
        name: "KEY_A",
        hid: 0x04,
        at101: Some(31),
        usage: Usage::Selector,
        desc: "Keyboard a and A",
    };

    pub const KEY_B: Key = Key {
        name: "KEY_B",
        hid: 0x05,
        at101: Some(50),
        usage: Usage::Selector,
        desc: "Keyboard b and B",
    };

    pub const KEY_C: Key = Key {
        name: "KEY_C",
        hid: 0x06,
        at101: Some(48),
        usage: Usage::Selector,
        desc: "Keyboard c and C",
    };

    pub const KEY_D: Key = Key {
        name: "KEY_D",
        hid: 0x07,
        at101: Some(33),
        usage: Usage::Selector,
        desc: "Keyboard d and D",
    };

    pub const KEY_E: Key = Key {
        name: "KEY_E",
        hid: 0x08,
        at101: Some(19),
        usage: Usage::Selector,
        desc: "Keyboard e and E",
    };

    pub const KEY_F: Key = Key {
        name: "KEY_F",
        hid: 0x09,
        at101: Some(34),
        usage: Usage::Selector,
        desc: "Keyboard f and F",
    };

    pub const KEY_G: Key = Key {
        name: "KEY_G",
        hid: 0x0a,
        at101: Some(35),
        usage: Usage::Selector,
        desc: "Keyboard g and G",
    };

    pub const KEY_H: Key = Key {
        name: "KEY_H",
        hid: 0x0b,
        at101: Some(36),
        usage: Usage::Selector,
        desc: "Keyboard h and H",
    };

    pub const KEY_I: Key = Key {
        name: "KEY_I",
        hid: 0x0c,
        at101: Some(24),
        usage: Usage::Selector,
        desc: "Keyboard i and I",
    };

    pub const KEY_J: Key = Key {
        name: "KEY_J",
        hid: 0x0d,
        at101: Some(37),
        usage: Usage::Selector,
        desc: "Keyboard j and J",
    };

    pub const KEY_K: Key = Key {
        name: "KEY_K",
        hid: 0x0e,
        at101: Some(38),
        usage: Usage::Selector,
        desc: "Keyboard k and K",
    };

    pub const KEY_L: Key = Key {
        name: "KEY_L",
        hid: 0x0f,
        at101: Some(39),
        usage: Usage::Selector,
        desc: "Keyboard l and L",
    };

    pub const KEY_M: Key = Key {
        name: "KEY_M",
        hid: 0x10,
        at101: Some(52),
        usage: Usage::Selector,
        desc: "Keyboard m and M",
    };

    pub const KEY_N: Key = Key {
        name: "KEY_N",
        hid: 0x11,
        at101: Some(51),
        usage: Usage::Selector,
        desc: "Keyboard n and N",
    };

    pub const KEY_O: Key = Key {
        name: "KEY_O",
        hid: 0x12,
        at101: Some(25),
        usage: Usage::Selector,
        desc: "Keyboard o and O",
    };

    pub const KEY_P: Key = Key {
        name: "KEY_P",
        hid: 0x13,
        at101: Some(26),
        usage: Usage::Selector,
        desc: "Keyboard p and P",
    };

    pub const KEY_Q: Key = Key {
        name: "KEY_Q",
        hid: 0x14,
        at101: Some(17),
        usage: Usage::Selector,
        desc: "Keyboard q and Q",
    };

    pub const KEY_R: Key = Key {
        name: "KEY_R",
        hid: 0x15,
        at101: Some(20),
        usage: Usage::Selector,
        desc: "Keyboard r and R",
    };

    pub const KEY_S: Key = Key {
        name: "KEY_S",
        hid: 0x16,
        at101: Some(32),
        usage: Usage::Selector,
        desc: "Keyboard s and S",
    };

    pub const KEY_T: Key = Key {
        name: "KEY_T",
        hid: 0x17,
        at101: Some(21),
        usage: Usage::Selector,
        desc: "Keyboard t and T",
    };

    pub const KEY_U: Key = Key {
        name: "KEY_U",
        hid: 0x18,
        at101: Some(23),
        usage: Usage::Selector,
        desc: "Keyboard u and U",
    };

    pub const KEY_V: Key = Key {
        name: "KEY_V",
        hid: 0x19,
        at101: Some(49),
        usage: Usage::Selector,
        desc: "Keyboard v and V",
    };

    pub const KEY_W: Key = Key {
        name: "KEY_W",
        hid: 0x1a,
        at101: Some(18),
        usage: Usage::Selector,
        desc: "Keyboard w and W",
    };

    pub const KEY_X: Key = Key {
        name: "KEY_X",
        hid: 0x1b,
        at101: Some(47),
        usage: Usage::Selector,
        desc: "Keyboard x and X",
    };

    pub const KEY_Y: Key = Key {
        name: "KEY_Y",
        hid: 0x1c,
        at101: Some(22),
        usage: Usage::Selector,
        desc: "Keyboard y and Y",
    };

    pub const KEY_Z: Key = Key {
        name: "KEY_Z",
        hid: 0x1d,
        at101: Some(46),
        usage: Usage::Selector,
        desc: "Keyboard z and Z",
    };

    pub const KEY_1: Key = Key {
        name: "KEY_1",
        hid: 0x1e,
        at101: Some(2),
        usage: Usage::Selector,
        desc: "Keyboard 1 and !",
    };

    pub const KEY_2: Key = Key {
        name: "KEY_2",
        hid: 0x1f,
        at101: Some(3),
        usage: Usage::Selector,
        desc: "Keyboard 2 and @",
    };

    pub const KEY_3: Key = Key {
        name: "KEY_3",
        hid: 0x20,
        at101: Some(4),
        usage: Usage::Selector,
        desc: "Keyboard 3 and #",
    };

    pub const KEY_4: Key = Key {
        name: "KEY_4",
        hid: 0x21,
        at101: Some(5),
        usage: Usage::Selector,
        desc: "Keyboard 4 and $",
    };

    pub const KEY_5: Key = Key {
        name: "KEY_5",
        hid: 0x22,
        at101: Some(6),
        usage: Usage::Selector,
        desc: "Keyboard 5 and %",
    };

    pub const KEY_6: Key = Key {
        name: "KEY_6",
        hid: 0x23,
        at101: Some(7),
        usage: Usage::Selector,
        desc: "Keyboard 6 and",
    };

    pub const KEY_7: Key = Key {
        name: "KEY_7",
        hid: 0x24,
        at101: Some(8),
        usage: Usage::Selector,
        desc: "Keyboard 7 and &",
    };

    pub const KEY_8: Key = Key {
        name: "KEY_8",
        hid: 0x25,
        at101: Some(9),
        usage: Usage::Selector,
        desc: "Keyboard 8 and *",
    };

    pub const KEY_9: Key = Key {
        name: "KEY_9",
        hid: 0x26,
        at101: Some(10),
        usage: Usage::Selector,
        desc: "Keyboard 9 and (",
    };

    pub const KEY_0: Key = Key {
        name: "KEY_0",
        hid: 0x27,
        at101: Some(11),
        usage: Usage::Selector,
        desc: "Keyboard 0 and )",
    };

    pub const KEY_ENTER: Key = Key {
        name: "KEY_ENTER",
        hid: 0x28,
        at101: Some(43),
        usage: Usage::Selector,
        desc: "Keyboard Return (ENTER)",
    };

    pub const KEY_ESC: Key = Key {
        name: "KEY_ESC",
        hid: 0x29,
        at101: Some(110),
        usage: Usage::Selector,
        desc: "Keyboard ESCAPE",
    };

    pub const KEY_BACKSPACE: Key = Key {
        name: "KEY_BACKSPACE",
        hid: 0x2a,
        at101: Some(15),
        usage: Usage::Selector,
        desc: "Keyboard DELETE (Backspace)",
    };

    pub const KEY_TAB: Key = Key {
        name: "KEY_TAB",
        hid: 0x2b,
        at101: Some(16),
        usage: Usage::Selector,
        desc: "Keyboard Tab",
    };

    pub const KEY_SPACE: Key = Key {
        name: "KEY_SPACE",
        hid: 0x2c,
        at101: Some(61),
        usage: Usage::Selector,
        desc: "Keyboard Spacebar",
    };

    pub const KEY_DASH: Key = Key {
        name: "KEY_DASH",
        hid: 0x2d,
        at101: Some(12),
        usage: Usage::Selector,
        desc: "Keyboard - and (underscore)",
    };

    pub const KEY_EQUAL: Key = Key {
        name: "KEY_EQUAL",
        hid: 0x2e,
        at101: Some(13),
        usage: Usage::Selector,
        desc: "Keyboard = and +",
    };

    pub const KEY_LEFT_BRACKET: Key = Key {
        name: "KEY_LEFT_BRACKET",
        hid: 0x2f,
        at101: Some(27),
        usage: Usage::Selector,
        desc: "Keyboard [ and",
    };

    pub const KEY_RIGHT_BRACKET: Key = Key {
        name: "KEY_RIGHT_BRACKET",
        hid: 0x30,
        at101: Some(28),
        usage: Usage::Selector,
        desc: "Keyboard ] and }",
    };

    pub const KEY_BACKSLASH_AND_PIPE: Key = Key {
        name: "KEY_BACKSLASH_AND_PIPE",
        hid: 0x31,
        at101: Some(29),
        usage: Usage::Selector,
        desc: "Keyboard \\and |",
    };

    pub const KEY_NON_US_HASH_AND_: Key = Key {
        name: "KEY_NON_US_HASH_AND_",
        hid: 0x32,
        at101: Some(42),
        usage: Usage::Selector,
        desc: "Keyboard Non-US # and ",
    };

    pub const KEY_SEMICOLON: Key = Key {
        name: "KEY_SEMICOLON",
        hid: 0x33,
        at101: Some(40),
        usage: Usage::Selector,
        desc: "Keyboard ; and :",
    };

    pub const KEY_QUOTE: Key = Key {
        name: "KEY_QUOTE",
        hid: 0x34,
        at101: Some(41),
        usage: Usage::Selector,
        desc: "Keyboard ‘ and “",
    };

    pub const KEY_GRAVE_ACCENT_AND_TILDE: Key = Key {
        name: "KEY_GRAVE_ACCENT_AND_TILDE",
        hid: 0x35,
        at101: Some(1),
        usage: Usage::Selector,
        desc: "Keyboard Grave Accent and Tilde",
    };

    pub const KEY_COMMA: Key = Key {
        name: "KEY_COMMA",
        hid: 0x36,
        at101: Some(53),
        usage: Usage::Selector,
        desc: "Keyboard , and <",
    };

    pub const KEY_DOT: Key = Key {
        name: "KEY_DOT",
        hid: 0x37,
        at101: Some(54),
        usage: Usage::Selector,
        desc: "Keyboard . and >",
    };

    pub const KEY_SLASH: Key = Key {
        name: "KEY_SLASH",
        hid: 0x38,
        at101: Some(55),
        usage: Usage::Selector,
        desc: "Keyboard / and ?",
    };

    pub const KEY_CAPS_LOCK: Key = Key {
        name: "KEY_CAPS_LOCK",
        hid: 0x39,
        at101: Some(30),
        usage: Usage::Selector,
        desc: "Keyboard Caps Lock",
    };

    pub const KEY_F1: Key = Key {
        name: "KEY_F1",
        hid: 0x3a,
        at101: Some(112),
        usage: Usage::Selector,
        desc: "Keyboard F1",
    };

    pub const KEY_F2: Key = Key {
        name: "KEY_F2",
        hid: 0x3b,
        at101: Some(113),
        usage: Usage::Selector,
        desc: "Keyboard F2",
    };

    pub const KEY_F3: Key = Key {
        name: "KEY_F3",
        hid: 0x3c,
        at101: Some(114),
        usage: Usage::Selector,
        desc: "Keyboard F3",
    };

    pub const KEY_F4: Key = Key {
        name: "KEY_F4",
        hid: 0x3d,
        at101: Some(115),
        usage: Usage::Selector,
        desc: "Keyboard F4",
    };

    pub const KEY_F5: Key = Key {
        name: "KEY_F5",
        hid: 0x3e,
        at101: Some(116),
        usage: Usage::Selector,
        desc: "Keyboard F5",
    };

    pub const KEY_F6: Key = Key {
        name: "KEY_F6",
        hid: 0x3f,
        at101: Some(117),
        usage: Usage::Selector,
        desc: "Keyboard F6",
    };

    pub const KEY_F7: Key = Key {
        name: "KEY_F7",
        hid: 0x40,
        at101: Some(118),
        usage: Usage::Selector,
        desc: "Keyboard F7",
    };

    pub const KEY_F8: Key = Key {
        name: "KEY_F8",
        hid: 0x41,
        at101: Some(119),
        usage: Usage::Selector,
        desc: "Keyboard F8",
    };

    pub const KEY_F9: Key = Key {
        name: "KEY_F9",
        hid: 0x42,
        at101: Some(120),
        usage: Usage::Selector,
        desc: "Keyboard F9",
    };

    pub const KEY_F10: Key = Key {
        name: "KEY_F10",
        hid: 0x43,
        at101: Some(121),
        usage: Usage::Selector,
        desc: "Keyboard F10",
    };

    pub const KEY_F11: Key = Key {
        name: "KEY_F11",
        hid: 0x44,
        at101: Some(122),
        usage: Usage::Selector,
        desc: "Keyboard F11",
    };

    pub const KEY_F12: Key = Key {
        name: "KEY_F12",
        hid: 0x45,
        at101: Some(123),
        usage: Usage::Selector,
        desc: "Keyboard F12",
    };

    pub const KEY_PRINTSCREEN: Key = Key {
        name: "KEY_PRINTSCREEN",
        hid: 0x46,
        at101: Some(124),
        usage: Usage::Selector,
        desc: "Keyboard PrintScreen",
    };

    pub const KEY_SCROLL_LOCK: Key = Key {
        name: "KEY_SCROLL_LOCK",
        hid: 0x47,
        at101: Some(125),
        usage: Usage::Selector,
        desc: "Keyboard Scroll Lock",
    };

    pub const KEY_PAUSE: Key = Key {
        name: "KEY_PAUSE",
        hid: 0x48,
        at101: Some(126),
        usage: Usage::Selector,
        desc: "Keyboard Pause",
    };

    pub const KEY_INSERT: Key = Key {
        name: "KEY_INSERT",
        hid: 0x49,
        at101: Some(75),
        usage: Usage::Selector,
        desc: "Keyboard Insert",
    };

    pub const KEY_HOME: Key = Key {
        name: "KEY_HOME",
        hid: 0x4a,
        at101: Some(80),
        usage: Usage::Selector,
        desc: "Keyboard Home",
    };

    pub const KEY_PAGEUP: Key = Key {
        name: "KEY_PAGEUP",
        hid: 0x4b,
        at101: Some(85),
        usage: Usage::Selector,
        desc: "Keyboard PageUp",
    };

    pub const KEY_DELETE_FORWARD: Key = Key {
        name: "KEY_DELETE_FORWARD",
        hid: 0x4c,
        at101: Some(76),
        usage: Usage::Selector,
        desc: "Keyboard Delete Forward",
    };

    pub const KEY_END: Key = Key {
        name: "KEY_END",
        hid: 0x4d,
        at101: Some(81),
        usage: Usage::Selector,
        desc: "Keyboard End",
    };

    pub const KEY_PAGEDOWN: Key = Key {
        name: "KEY_PAGEDOWN",
        hid: 0x4e,
        at101: Some(86),
        usage: Usage::Selector,
        desc: "Keyboard PageDown",
    };

    pub const KEY_RIGHT_ARROW: Key = Key {
        name: "KEY_RIGHT_ARROW",
        hid: 0x4f,
        at101: Some(89),
        usage: Usage::Selector,
        desc: "Keyboard Right Arrow",
    };

    pub const KEY_LEFT_ARROW: Key = Key {
        name: "KEY_LEFT_ARROW",
        hid: 0x50,
        at101: Some(79),
        usage: Usage::Selector,
        desc: "Keyboard Left Arrow",
    };

    pub const KEY_DOWN_ARROW: Key = Key {
        name: "KEY_DOWN_ARROW",
        hid: 0x51,
        at101: Some(84),
        usage: Usage::Selector,
        desc: "Keyboard Down Arrow",
    };

    pub const KEY_UP_ARROW: Key = Key {
        name: "KEY_UP_ARROW",
        hid: 0x52,
        at101: Some(83),
        usage: Usage::Selector,
        desc: "Keyboard Up Arrow",
    };

    pub const KEY_KPD_NUM_LOCK_AND_CLEAR: Key = Key {
        name: "KEY_KPD_NUM_LOCK_AND_CLEAR",
        hid: 0x53,
        at101: Some(90),
        usage: Usage::Selector,
        desc: "Keypad Num Lock and Clear",
    };

    pub const KEY_KPD_SLASH: Key = Key {
        name: "KEY_KPD_SLASH",
        hid: 0x54,
        at101: Some(95),
        usage: Usage::Selector,
        desc: "Keypad /",
    };

    pub const KEY_KPD_ASTERISK: Key = Key {
        name: "KEY_KPD_ASTERISK",
        hid: 0x55,
        at101: Some(100),
        usage: Usage::Selector,
        desc: "Keypad *",
    };

    pub const KEY_KPD_DASH: Key = Key {
        name: "KEY_KPD_DASH",
        hid: 0x56,
        at101: Some(105),
        usage: Usage::Selector,
        desc: "Keypad -",
    };

    pub const KEY_KPD_PLUS: Key = Key {
        name: "KEY_KPD_PLUS",
        hid: 0x57,
        at101: Some(106),
        usage: Usage::Selector,
        desc: "Keypad +",
    };

    pub const KEY_KPD_ENTER: Key = Key {
        name: "KEY_KPD_ENTER",
        hid: 0x58,
        at101: Some(108),
        usage: Usage::Selector,
        desc: "Keypad ENTER",
    };

    pub const KEY_KPD_1: Key = Key {
        name: "KEY_KPD_1",
        hid: 0x59,
        at101: Some(93),
        usage: Usage::Selector,
        desc: "Keypad 1 and End",
    };

    pub const KEY_KPD_2: Key = Key {
        name: "KEY_KPD_2",
        hid: 0x5a,
        at101: Some(98),
        usage: Usage::Selector,
        desc: "Keypad 2 and Down Arrow",
    };

    pub const KEY_KPD_3: Key = Key {
        name: "KEY_KPD_3",
        hid: 0x5b,
        at101: Some(103),
        usage: Usage::Selector,
        desc: "Keypad 3 and PageDn",
    };

    pub const KEY_KPD_4: Key = Key {
        name: "KEY_KPD_4",
        hid: 0x5c,
        at101: Some(92),
        usage: Usage::Selector,
        desc: "Keypad 4 and Left Arrow",
    };

    pub const KEY_KPD_5: Key = Key {
        name: "KEY_KPD_5",
        hid: 0x5d,
        at101: Some(97),
        usage: Usage::Selector,
        desc: "Keypad 5",
    };

    pub const KEY_KPD_6: Key = Key {
        name: "KEY_KPD_6",
        hid: 0x5e,
        at101: Some(102),
        usage: Usage::Selector,
        desc: "Keypad 6 and Right Arrow",
    };

    pub const KEY_KPD_7: Key = Key {
        name: "KEY_KPD_7",
        hid: 0x5f,
        at101: Some(91),
        usage: Usage::Selector,
        desc: "Keypad 7 and Home",
    };

    pub const KEY_KPD_8: Key = Key {
        name: "KEY_KPD_8",
        hid: 0x60,
        at101: Some(96),
        usage: Usage::Selector,
        desc: "Keypad 8 and Up Arrow",
    };

    pub const KEY_KPD_9: Key = Key {
        name: "KEY_KPD_9",
        hid: 0x61,
        at101: Some(101),
        usage: Usage::Selector,
        desc: "Keypad 9 and PageUp",
    };

    pub const KEY_KPD_0: Key = Key {
        name: "KEY_KPD_0",
        hid: 0x62,
        at101: Some(99),
        usage: Usage::Selector,
        desc: "Keypad 0 and Insert",
    };

    pub const KEY_KPD_DOT: Key = Key {
        name: "KEY_KPD_DOT",
        hid: 0x63,
        at101: Some(104),
        usage: Usage::Selector,
        desc: "Keypad . and Delete",
    };

    pub const KEY_NON_US_BACKSLASH_AND_PIPE: Key = Key {
        name: "KEY_NON_US_BACKSLASH_AND_PIPE",
        hid: 0x64,
        at101: Some(45),
        usage: Usage::Selector,
        desc: "Keyboard Non-US \\and |",
    };

    pub const KEY_APPLICATION: Key = Key {
        name: "KEY_APPLICATION",
        hid: 0x65,
        at101: Some(129),
        usage: Usage::Selector,
        desc: "Keyboard Application",
    };

    pub const KEY_POWER: Key = Key {
        name: "KEY_POWER",
        hid: 0x66,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Power",
    };

    pub const KEY_KPD_EQUAL: Key = Key {
        name: "KEY_KPD_EQUAL",
        hid: 0x67,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad =",
    };

    pub const KEY_F13: Key = Key {
        name: "KEY_F13",
        hid: 0x68,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F13",
    };

    pub const KEY_F14: Key = Key {
        name: "KEY_F14",
        hid: 0x69,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F14",
    };

    pub const KEY_F15: Key = Key {
        name: "KEY_F15",
        hid: 0x6a,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F15",
    };

    pub const KEY_F16: Key = Key {
        name: "KEY_F16",
        hid: 0x6b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F16",
    };

    pub const KEY_F17: Key = Key {
        name: "KEY_F17",
        hid: 0x6c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F17",
    };

    pub const KEY_F18: Key = Key {
        name: "KEY_F18",
        hid: 0x6d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F18",
    };

    pub const KEY_F19: Key = Key {
        name: "KEY_F19",
        hid: 0x6e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F19",
    };

    pub const KEY_F20: Key = Key {
        name: "KEY_F20",
        hid: 0x6f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F20",
    };

    pub const KEY_F21: Key = Key {
        name: "KEY_F21",
        hid: 0x70,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F21",
    };

    pub const KEY_F22: Key = Key {
        name: "KEY_F22",
        hid: 0x71,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F22",
    };

    pub const KEY_F23: Key = Key {
        name: "KEY_F23",
        hid: 0x72,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F23",
    };

    pub const KEY_F24: Key = Key {
        name: "KEY_F24",
        hid: 0x73,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard F24",
    };

    pub const KEY_EXECUTE: Key = Key {
        name: "KEY_EXECUTE",
        hid: 0x74,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Execute",
    };

    pub const KEY_HELP: Key = Key {
        name: "KEY_HELP",
        hid: 0x75,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Help",
    };

    pub const KEY_MENU: Key = Key {
        name: "KEY_MENU",
        hid: 0x76,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Menu",
    };

    pub const KEY_SELECT: Key = Key {
        name: "KEY_SELECT",
        hid: 0x77,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Select",
    };

    pub const KEY_STOP: Key = Key {
        name: "KEY_STOP",
        hid: 0x78,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Stop",
    };

    pub const KEY_AGAIN: Key = Key {
        name: "KEY_AGAIN",
        hid: 0x79,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Again",
    };

    pub const KEY_UNDO: Key = Key {
        name: "KEY_UNDO",
        hid: 0x7a,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Undo",
    };

    pub const KEY_CUT: Key = Key {
        name: "KEY_CUT",
        hid: 0x7b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Cut",
    };

    pub const KEY_COPY: Key = Key {
        name: "KEY_COPY",
        hid: 0x7c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Copy",
    };

    pub const KEY_PASTE: Key = Key {
        name: "KEY_PASTE",
        hid: 0x7d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Paste",
    };

    pub const KEY_FIND: Key = Key {
        name: "KEY_FIND",
        hid: 0x7e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Find",
    };

    pub const KEY_MUTE: Key = Key {
        name: "KEY_MUTE",
        hid: 0x7f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Mute",
    };

    pub const KEY_VOLUME_UP: Key = Key {
        name: "KEY_VOLUME_UP",
        hid: 0x80,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Volume Up",
    };

    pub const KEY_VOLUME_DOWN: Key = Key {
        name: "KEY_VOLUME_DOWN",
        hid: 0x81,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Volume Down",
    };

    pub const KEY_LOCKING_CAPS_LOCK: Key = Key {
        name: "KEY_LOCKING_CAPS_LOCK",
        hid: 0x82,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Locking Caps Lock",
    };

    pub const KEY_LOCKING_NUM_LOCK: Key = Key {
        name: "KEY_LOCKING_NUM_LOCK",
        hid: 0x83,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Locking Num Lock",
    };

    pub const KEY_LOCKING_SCROLL: Key = Key {
        name: "KEY_LOCKING_SCROLL",
        hid: 0x84,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Locking Scroll",
    };

    pub const KEY_KPD_COMMA: Key = Key {
        name: "KEY_KPD_COMMA",
        hid: 0x85,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Comma",
    };

    pub const KEY_INTERNATIONAL1: Key = Key {
        name: "KEY_INTERNATIONAL1",
        hid: 0x87,
        at101: Some(56),
        usage: Usage::Selector,
        desc: "Keyboard International1",
    };

    pub const KEY_INTERNATIONAL2: Key = Key {
        name: "KEY_INTERNATIONAL2",
        hid: 0x88,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International2",
    };

    pub const KEY_INTERNATIONAL3: Key = Key {
        name: "KEY_INTERNATIONAL3",
        hid: 0x89,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International3",
    };

    pub const KEY_INTERNATIONAL4: Key = Key {
        name: "KEY_INTERNATIONAL4",
        hid: 0x8a,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International4",
    };

    pub const KEY_INTERNATIONAL5: Key = Key {
        name: "KEY_INTERNATIONAL5",
        hid: 0x8b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International5",
    };

    pub const KEY_INTERNATIONAL6: Key = Key {
        name: "KEY_INTERNATIONAL6",
        hid: 0x8c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International6",
    };

    pub const KEY_INTERNATIONAL7: Key = Key {
        name: "KEY_INTERNATIONAL7",
        hid: 0x8d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International7",
    };

    pub const KEY_INTERNATIONAL8: Key = Key {
        name: "KEY_INTERNATIONAL8",
        hid: 0x8e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International8",
    };

    pub const KEY_INTERNATIONAL9: Key = Key {
        name: "KEY_INTERNATIONAL9",
        hid: 0x8f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard International9",
    };

    pub const KEY_LANG1: Key = Key {
        name: "KEY_LANG1",
        hid: 0x90,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG1",
    };

    pub const KEY_LANG2: Key = Key {
        name: "KEY_LANG2",
        hid: 0x91,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG2",
    };

    pub const KEY_LANG3: Key = Key {
        name: "KEY_LANG3",
        hid: 0x92,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG3",
    };

    pub const KEY_LANG4: Key = Key {
        name: "KEY_LANG4",
        hid: 0x93,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG4",
    };

    pub const KEY_LANG5: Key = Key {
        name: "KEY_LANG5",
        hid: 0x94,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG5",
    };

    pub const KEY_LANG6: Key = Key {
        name: "KEY_LANG6",
        hid: 0x95,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG6",
    };

    pub const KEY_LANG7: Key = Key {
        name: "KEY_LANG7",
        hid: 0x96,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG7",
    };

    pub const KEY_LANG8: Key = Key {
        name: "KEY_LANG8",
        hid: 0x97,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG8",
    };

    pub const KEY_LANG9: Key = Key {
        name: "KEY_LANG9",
        hid: 0x98,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard LANG9",
    };

    pub const KEY_CANCEL: Key = Key {
        name: "KEY_CANCEL",
        hid: 0x9b,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Cancel",
    };

    pub const KEY_CLEAR: Key = Key {
        name: "KEY_CLEAR",
        hid: 0x9c,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Clear",
    };

    pub const KEY_PRIOR: Key = Key {
        name: "KEY_PRIOR",
        hid: 0x9d,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Prior",
    };

    pub const KEY_RETURN: Key = Key {
        name: "KEY_RETURN",
        hid: 0x9e,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Return",
    };

    pub const KEY_SEPARATOR: Key = Key {
        name: "KEY_SEPARATOR",
        hid: 0x9f,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Separator",
    };

    pub const KEY_OUT: Key = Key {
        name: "KEY_OUT",
        hid: 0xa0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Out",
    };

    pub const KEY_OPER: Key = Key {
        name: "KEY_OPER",
        hid: 0xa1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Oper",
    };

    pub const KEY_CLEAR_AGAIN: Key = Key {
        name: "KEY_CLEAR_AGAIN",
        hid: 0xa2,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard Clear/Again",
    };

    pub const KEY_CRSEL_PROPS: Key = Key {
        name: "KEY_CRSEL_PROPS",
        hid: 0xa3,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard CrSel/Props",
    };

    pub const KEY_EXSEL: Key = Key {
        name: "KEY_EXSEL",
        hid: 0xa4,
        at101: None,
        usage: Usage::Selector,
        desc: "Keyboard ExSel",
    };

    pub const KEY_KPD_00: Key = Key {
        name: "KEY_KPD_00",
        hid: 0xb0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad 00",
    };

    pub const KEY_KPD_000: Key = Key {
        name: "KEY_KPD_000",
        hid: 0xb1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad 000",
    };

    pub const KEY_THOUSANDS_SEPARATOR: Key = Key {
        name: "KEY_THOUSANDS_SEPARATOR",
        hid: 0xb2,
        at101: None,
        usage: Usage::Selector,
        desc: "Thousands Separator",
    };

    pub const KEY_DECIMAL_SEPARATOR: Key = Key {
        name: "KEY_DECIMAL_SEPARATOR",
        hid: 0xb3,
        at101: None,
        usage: Usage::Selector,
        desc: "Decimal Separator",
    };

    pub const KEY_CURRENCY_UNIT: Key = Key {
        name: "KEY_CURRENCY_UNIT",
        hid: 0xb4,
        at101: None,
        usage: Usage::Selector,
        desc: "Currency Unit",
    };

    pub const KEY_CURRENCY_SUBUNIT: Key = Key {
        name: "KEY_CURRENCY_SUBUNIT",
        hid: 0xb5,
        at101: None,
        usage: Usage::Selector,
        desc: "Currency Sub-unit",
    };

    pub const KEY_KPD_LEFT_PARENTHESIS: Key = Key {
        name: "KEY_KPD_LEFT_PARENTHESIS",
        hid: 0xb6,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad (",
    };

    pub const KEY_KPD_RIGHT_PARENTHESIS: Key = Key {
        name: "KEY_KPD_RIGHT_PARENTHESIS",
        hid: 0xb7,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad )",
    };

    pub const KEY_KPD_LEFT_BRACE: Key = Key {
        name: "KEY_KPD_LEFT_BRACE",
        hid: 0xb8,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad {",
    };

    pub const KEY_KPD_RIGHT_BRACE: Key = Key {
        name: "KEY_KPD_RIGHT_BRACE",
        hid: 0xb9,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad }",
    };

    pub const KEY_KPD_TAB: Key = Key {
        name: "KEY_KPD_TAB",
        hid: 0xba,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Tab",
    };

    pub const KEY_KPD_BACKSPACE: Key = Key {
        name: "KEY_KPD_BACKSPACE",
        hid: 0xbb,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Backspace",
    };

    pub const KEY_KPD_A: Key = Key {
        name: "KEY_KPD_A",
        hid: 0xbc,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad A",
    };

    pub const KEY_KPD_B: Key = Key {
        name: "KEY_KPD_B",
        hid: 0xbd,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad B",
    };

    pub const KEY_KPD_C: Key = Key {
        name: "KEY_KPD_C",
        hid: 0xbe,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad C",
    };

    pub const KEY_KPD_D: Key = Key {
        name: "KEY_KPD_D",
        hid: 0xbf,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad D",
    };

    pub const KEY_KPD_E: Key = Key {
        name: "KEY_KPD_E",
        hid: 0xc0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad E",
    };

    pub const KEY_KPD_F: Key = Key {
        name: "KEY_KPD_F",
        hid: 0xc1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad F",
    };

    pub const KEY_KPD_XOR: Key = Key {
        name: "KEY_KPD_XOR",
        hid: 0xc2,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad XOR",
    };

    pub const KEY_KPD_CARET: Key = Key {
        name: "KEY_KPD_CARET",
        hid: 0xc3,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad ^",
    };

    pub const KEY_KPD_PCT: Key = Key {
        name: "KEY_KPD_PCT",
        hid: 0xc4,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad %",
    };

    pub const KEY_KPD_LT: Key = Key {
        name: "KEY_KPD_LT",
        hid: 0xc5,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad <",
    };

    pub const KEY_KPD_GT: Key = Key {
        name: "KEY_KPD_GT",
        hid: 0xc6,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad >",
    };

    pub const KEY_KPD_AMPERSAND: Key = Key {
        name: "KEY_KPD_AMPERSAND",
        hid: 0xc7,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad &",
    };

    pub const KEY_KPD_DOUBLEAMPERSAND: Key = Key {
        name: "KEY_KPD_DOUBLEAMPERSAND",
        hid: 0xc8,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad &&",
    };

    pub const KEY_KPD_PIPE: Key = Key {
        name: "KEY_KPD_PIPE",
        hid: 0xc9,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad |",
    };

    pub const KEY_KPD_DOUBLEPIPE: Key = Key {
        name: "KEY_KPD_DOUBLEPIPE",
        hid: 0xca,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad ||",
    };

    pub const KEY_KPD_COLON: Key = Key {
        name: "KEY_KPD_COLON",
        hid: 0xcb,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad :",
    };

    pub const KEY_KPD_HASH: Key = Key {
        name: "KEY_KPD_HASH",
        hid: 0xcc,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad #",
    };

    pub const KEY_KPD_SPACE: Key = Key {
        name: "KEY_KPD_SPACE",
        hid: 0xcd,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Space",
    };

    pub const KEY_KPD_AT: Key = Key {
        name: "KEY_KPD_AT",
        hid: 0xce,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad @",
    };

    pub const KEY_KPD_EXCLAMATION: Key = Key {
        name: "KEY_KPD_EXCLAMATION",
        hid: 0xcf,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad !",
    };

    pub const KEY_KPD_MEMORY_STORE: Key = Key {
        name: "KEY_KPD_MEMORY_STORE",
        hid: 0xd0,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Store",
    };

    pub const KEY_KPD_MEMORY_RECALL: Key = Key {
        name: "KEY_KPD_MEMORY_RECALL",
        hid: 0xd1,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Recall",
    };

    pub const KEY_KPD_MEMORY_CLEAR: Key = Key {
        name: "KEY_KPD_MEMORY_CLEAR",
        hid: 0xd2,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Clear",
    };

    pub const KEY_KPD_MEMORY_ADD: Key = Key {
        name: "KEY_KPD_MEMORY_ADD",
        hid: 0xd3,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Add",
    };

    pub const KEY_KPD_MEMORY_SUBTRACT: Key = Key {
        name: "KEY_KPD_MEMORY_SUBTRACT",
        hid: 0xd4,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Subtract",
    };

    pub const KEY_KPD_MEMORY_MULTIPLY: Key = Key {
        name: "KEY_KPD_MEMORY_MULTIPLY",
        hid: 0xd5,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Multiply",
    };

    pub const KEY_KPD_MEMORY_DIVIDE: Key = Key {
        name: "KEY_KPD_MEMORY_DIVIDE",
        hid: 0xd6,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Memory Divide",
    };

    pub const KEY_KPD_PLUSMINUS: Key = Key {
        name: "KEY_KPD_PLUSMINUS",
        hid: 0xd7,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad +/-",
    };

    pub const KEY_KPD_CLEAR: Key = Key {
        name: "KEY_KPD_CLEAR",
        hid: 0xd8,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Clear",
    };

    pub const KEY_KPD_CLEAR_ENTRY: Key = Key {
        name: "KEY_KPD_CLEAR_ENTRY",
        hid: 0xd9,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Clear Entry",
    };

    pub const KEY_KPD_BINARY: Key = Key {
        name: "KEY_KPD_BINARY",
        hid: 0xda,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Binary",
    };

    pub const KEY_KPD_OCTAL: Key = Key {
        name: "KEY_KPD_OCTAL",
        hid: 0xdb,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Octal",
    };

    pub const KEY_KPD_DECIMAL: Key = Key {
        name: "KEY_KPD_DECIMAL",
        hid: 0xdc,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Decimal",
    };

    pub const KEY_KPD_HEXADECIMAL: Key = Key {
        name: "KEY_KPD_HEXADECIMAL",
        hid: 0xdd,
        at101: None,
        usage: Usage::Selector,
        desc: "Keypad Hexadecimal",
    };

    pub const KEY_LEFT_CONTROL: Key = Key {
        name: "KEY_LEFT_CONTROL",
        hid: 0xe0,
        at101: Some(58),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left Control",
    };

    pub const KEY_LEFT_SHIFT: Key = Key {
        name: "KEY_LEFT_SHIFT",
        hid: 0xe1,
        at101: Some(44),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left Shift",
    };

    pub const KEY_LEFT_ALT: Key = Key {
        name: "KEY_LEFT_ALT",
        hid: 0xe2,
        at101: Some(60),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left Alt",
    };

    pub const KEY_LEFT_META: Key = Key {
        name: "KEY_LEFT_META",
        hid: 0xe3,
        at101: Some(127),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Left GUI",
    };

    pub const KEY_RIGHT_CONTROL: Key = Key {
        name: "KEY_RIGHT_CONTROL",
        hid: 0xe4,
        at101: Some(64),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right Control",
    };

    pub const KEY_RIGHT_SHIFT: Key = Key {
        name: "KEY_RIGHT_SHIFT",
        hid: 0xe5,
        at101: Some(57),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right Shift",
    };

    pub const KEY_RIGHT_ALT: Key = Key {
        name: "KEY_RIGHT_ALT",
        hid: 0xe6,
        at101: Some(62),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right Alt",
    };

    pub const KEY_RIGHT_META: Key = Key {
        name: "KEY_RIGHT_META",
        hid: 0xe7,
        at101: Some(128),
        usage: Usage::DynamicFlag,
        desc: "Keyboard Right GUI",
    };

    pub const fn keys() -> &'static [Key] {
        &[
            KEY_ERRORROLLOVER,
            KEY_POSTFAIL,
            KEY_ERRORUNDEFINED,
            KEY_A,
            KEY_B,
            KEY_C,
            KEY_D,
            KEY_E,
            KEY_F,
            KEY_G,
            KEY_H,
            KEY_I,
            KEY_J,
            KEY_K,
            KEY_L,
            KEY_M,
            KEY_N,
            KEY_O,
            KEY_P,
            KEY_Q,
            KEY_R,
            KEY_S,
            KEY_T,
            KEY_U,
            KEY_V,
            KEY_W,
            KEY_X,
            KEY_Y,
            KEY_Z,
            KEY_1,
            KEY_2,
            KEY_3,
            KEY_4,
            KEY_5,
            KEY_6,
            KEY_7,
            KEY_8,
            KEY_9,
            KEY_0,
            KEY_ENTER,
            KEY_ESC,
            KEY_BACKSPACE,
            KEY_TAB,
            KEY_SPACE,
            KEY_DASH,
            KEY_EQUAL,
            KEY_LEFT_BRACKET,
            KEY_RIGHT_BRACKET,
            KEY_BACKSLASH_AND_PIPE,
            KEY_NON_US_HASH_AND_,
            KEY_SEMICOLON,
            KEY_QUOTE,
            KEY_GRAVE_ACCENT_AND_TILDE,
            KEY_COMMA,
            KEY_DOT,
            KEY_SLASH,
            KEY_CAPS_LOCK,
            KEY_F1,
            KEY_F2,
            KEY_F3,
            KEY_F4,
            KEY_F5,
            KEY_F6,
            KEY_F7,
            KEY_F8,
            KEY_F9,
            KEY_F10,
            KEY_F11,
            KEY_F12,
            KEY_PRINTSCREEN,
            KEY_SCROLL_LOCK,
            KEY_PAUSE,
            KEY_INSERT,
            KEY_HOME,
            KEY_PAGEUP,
            KEY_DELETE_FORWARD,
            KEY_END,
            KEY_PAGEDOWN,
            KEY_RIGHT_ARROW,
            KEY_LEFT_ARROW,
            KEY_DOWN_ARROW,
            KEY_UP_ARROW,
            KEY_KPD_NUM_LOCK_AND_CLEAR,
            KEY_KPD_SLASH,
            KEY_KPD_ASTERISK,
            KEY_KPD_DASH,
            KEY_KPD_PLUS,
            KEY_KPD_ENTER,
            KEY_KPD_1,
            KEY_KPD_2,
            KEY_KPD_3,
            KEY_KPD_4,
            KEY_KPD_5,
            KEY_KPD_6,
            KEY_KPD_7,
            KEY_KPD_8,
            KEY_KPD_9,
            KEY_KPD_0,
            KEY_KPD_DOT,
            KEY_NON_US_BACKSLASH_AND_PIPE,
            KEY_APPLICATION,
            KEY_POWER,
            KEY_KPD_EQUAL,
            KEY_F13,
            KEY_F14,
            KEY_F15,
            KEY_F16,
            KEY_F17,
            KEY_F18,
            KEY_F19,
            KEY_F20,
            KEY_F21,
            KEY_F22,
            KEY_F23,
            KEY_F24,
            KEY_EXECUTE,
            KEY_HELP,
            KEY_MENU,
            KEY_SELECT,
            KEY_STOP,
            KEY_AGAIN,
            KEY_UNDO,
            KEY_CUT,
            KEY_COPY,
            KEY_PASTE,
            KEY_FIND,
            KEY_MUTE,
            KEY_VOLUME_UP,
            KEY_VOLUME_DOWN,
            KEY_LOCKING_CAPS_LOCK,
            KEY_LOCKING_NUM_LOCK,
            KEY_LOCKING_SCROLL,
            KEY_KPD_COMMA,
            KEY_INTERNATIONAL1,
            KEY_INTERNATIONAL2,
            KEY_INTERNATIONAL3,
            KEY_INTERNATIONAL4,
            KEY_INTERNATIONAL5,
            KEY_INTERNATIONAL6,
            KEY_INTERNATIONAL7,
            KEY_INTERNATIONAL8,
            KEY_INTERNATIONAL9,
            KEY_LANG1,
            KEY_LANG2,
            KEY_LANG3,
            KEY_LANG4,
            KEY_LANG5,
            KEY_LANG6,
            KEY_LANG7,
            KEY_LANG8,
            KEY_LANG9,
            KEY_CANCEL,
            KEY_CLEAR,
            KEY_PRIOR,
            KEY_RETURN,
            KEY_SEPARATOR,
            KEY_OUT,
            KEY_OPER,
            KEY_CLEAR_AGAIN,
            KEY_CRSEL_PROPS,
            KEY_EXSEL,
            KEY_KPD_00,
            KEY_KPD_000,
            KEY_THOUSANDS_SEPARATOR,
            KEY_DECIMAL_SEPARATOR,
            KEY_CURRENCY_UNIT,
            KEY_CURRENCY_SUBUNIT,
            KEY_KPD_LEFT_PARENTHESIS,
            KEY_KPD_RIGHT_PARENTHESIS,
            KEY_KPD_LEFT_BRACE,
            KEY_KPD_RIGHT_BRACE,
            KEY_KPD_TAB,
            KEY_KPD_BACKSPACE,
            KEY_KPD_A,
            KEY_KPD_B,
            KEY_KPD_C,
            KEY_KPD_D,
            KEY_KPD_E,
            KEY_KPD_F,
            KEY_KPD_XOR,
            KEY_KPD_CARET,
            KEY_KPD_PCT,
            KEY_KPD_LT,
            KEY_KPD_GT,
            KEY_KPD_AMPERSAND,
            KEY_KPD_DOUBLEAMPERSAND,
            KEY_KPD_PIPE,
            KEY_KPD_DOUBLEPIPE,
            KEY_KPD_COLON,
            KEY_KPD_HASH,
            KEY_KPD_SPACE,
            KEY_KPD_AT,
            KEY_KPD_EXCLAMATION,
            KEY_KPD_MEMORY_STORE,
            KEY_KPD_MEMORY_RECALL,
            KEY_KPD_MEMORY_CLEAR,
            KEY_KPD_MEMORY_ADD,
            KEY_KPD_MEMORY_SUBTRACT,
            KEY_KPD_MEMORY_MULTIPLY,
            KEY_KPD_MEMORY_DIVIDE,
            KEY_KPD_PLUSMINUS,
            KEY_KPD_CLEAR,
            KEY_KPD_CLEAR_ENTRY,
            KEY_KPD_BINARY,
            KEY_KPD_OCTAL,
            KEY_KPD_DECIMAL,
            KEY_KPD_HEXADECIMAL,
            KEY_LEFT_CONTROL,
            KEY_LEFT_SHIFT,
            KEY_LEFT_ALT,
            KEY_LEFT_META,
            KEY_RIGHT_CONTROL,
            KEY_RIGHT_SHIFT,
            KEY_RIGHT_ALT,
            KEY_RIGHT_META,
        ]
    }
}
