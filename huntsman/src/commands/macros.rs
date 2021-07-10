use serde::{Deserialize, Serialize};
use struct_helper::*;

use crate::hut_util::{keyboard_page_deserialize, keyboard_page_serialize};

pub type MacroId = u16;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MouseState {
    None = 0,
    Left = 1,
    Right = 2,
    Scroll = 4,
    M4 = 8,
    M5 = 16,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
/// Enum to represent an action in a macro.
#[serde(rename_all = "snake_case")]
pub enum MacroAction {
    /// HID key id.
    #[serde(
        serialize_with = "keyboard_page_serialize",
        deserialize_with = "keyboard_page_deserialize"
    )]
    KeyboardMake { hid: u8 },
    /// HID key id.
    #[serde(
        serialize_with = "keyboard_page_serialize",
        deserialize_with = "keyboard_page_deserialize"
    )]
    KeyboardBreak { hid: u8 },
    /// Delay in milliseconds.
    Delay(u32),
    /// Sets the mouse click state (bitmask), use again with 0 to release
    MouseClick(MouseState),
    /// scroll with the mouse
    MouseScroll(i8),
    /// move mouse relative, with x and y values.
    MouseMove { x: i16, y: i16 },
    /// No action.
    None,
}

impl MacroAction {
    const KEYBOARD_MAKE: u8 = 0x01;
    const KEYBOARD_BREAK: u8 = 0x02;

    const KEYBOARD_DELAY_U8: u8 = 0x11;
    const KEYBOARD_DELAY_U16: u8 = 0x12;
    const KEYBOARD_DELAY_U24: u8 = 0x13; // for real... :(
    const KEYBOARD_DELAY_U32: u8 = 0x14;
    const MOUSE_MOVE: u8 = 0x15;

    const MOUSE_CLICK: u8 = 0x08;

    const MOUSE_SCROLL: u8 = 0x0a;
}

impl FromBytes for MacroAction {
    fn from_bytes(&mut self, src: &[u8], _endianness: Endianness) -> Result<usize, String>
    where
        Self: Sized,
    {
        if src.len() < 2 {
            return Err(format!(
                "Not enough bytes to make a macro from, got {:?}",
                src
            ));
        }
        let specification = src[0];
        match specification {
            MacroAction::KEYBOARD_MAKE => {
                *self = MacroAction::KeyboardMake { hid: src[1] };
                return Ok(2);
            }
            MacroAction::KEYBOARD_BREAK => {
                *self = MacroAction::KeyboardBreak { hid: src[1] };
                return Ok(2);
            }
            MacroAction::KEYBOARD_DELAY_U8
            | MacroAction::KEYBOARD_DELAY_U16
            | MacroAction::KEYBOARD_DELAY_U24
            | MacroAction::KEYBOARD_DELAY_U32 => {
                let get_delay_byte_length = (specification & 0x0F) as usize;
                // lets always use a 32 bit integer.
                let mut arr: [u8; 4] = [0; 4];
                // Now copy the correct number of bytes to the correct location
                for i in 0..get_delay_byte_length {
                    arr[i] = src[get_delay_byte_length - i];
                }
                // Now, we interpret this as little endian, since least significant is left.
                *self = MacroAction::Delay(u32::from_le_bytes(arr));
                return Ok(1 + get_delay_byte_length);
            }
            MacroAction::MOUSE_CLICK => {
                let key_code = src[1];
                let button = match key_code {
                    i if i == MouseState::None as u8 => MouseState::None,
                    i if i == MouseState::Left as u8 => MouseState::Left,
                    i if i == MouseState::Right as u8 => MouseState::Right,
                    i if i == MouseState::Scroll as u8 => MouseState::Scroll,
                    i if i == MouseState::M4 as u8 => MouseState::M4,
                    i if i == MouseState::M5 as u8 => MouseState::M5,
                    _ => panic!("Unhandled mouse code: {:?}, total: {:?}", key_code, src),
                };

                *self = MacroAction::MouseClick(button);
                return Ok(2);
            }
            MacroAction::MOUSE_SCROLL => {
                let arr: [u8; 1] = [src[1]];
                *self = MacroAction::MouseScroll(i8::from_le_bytes(arr));
                return Ok(2);
            }
            MacroAction::MOUSE_MOVE => {
                let x: [u8; 2] = [src[1], src[2]];
                let y: [u8; 2] = [src[3], src[4]];
                *self = MacroAction::MouseMove {
                    x: i16::from_be_bytes(x),
                    y: i16::from_be_bytes(y),
                };
                return Ok(5);
            }
            z => panic!("Unhandled macro code {:?}, total src: {:?}", z, src),
        }
    }
}

impl ToBytes for MacroAction {
    fn to_bytes(&self, _endianness: Endianness) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = Vec::new();
        match self {
            MacroAction::KeyboardMake { hid } => {
                buff.push(MacroAction::KEYBOARD_MAKE);
                buff.push(*hid);
            }
            MacroAction::KeyboardBreak { hid } => {
                buff.push(MacroAction::KEYBOARD_BREAK);
                buff.push(*hid);
            }
            MacroAction::Delay(delay) => {
                let b = delay.to_be_bytes()?;
                // now... we do things based on the amount of zeros :(
                if b[0] != 0
                // need 4 bytes
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U32);
                    buff.extend(b.to_vec());
                } else if b[1] != 0
                // 3 byte
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U24);
                    buff.extend(b[1..].to_vec());
                } else if b[2] != 0
                // 2 byte
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U16);
                    buff.extend(b[2..].to_vec());
                } else if b[3] != 0
                // 1 byte
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U8);
                    buff.extend(b[3..].to_vec());
                }
            }
            MacroAction::MouseClick(button) => {
                buff.push(MacroAction::MOUSE_CLICK);
                buff.push(*button as u8);
            }
            MacroAction::MouseScroll(delta) => {
                buff.push(MacroAction::MOUSE_SCROLL);
                buff.push(i8::to_le_bytes(*delta)[0]);
            }
            MacroAction::MouseMove { x, y } => {
                buff.push(MacroAction::MOUSE_MOVE);
                buff.extend(i16::to_be_bytes(*x).iter());
                buff.extend(i16::to_be_bytes(*y).iter());
            }
            z => panic!("Unhandled macro code {:?}", z),
        }
        Ok(buff)
    }
}
impl Default for MacroAction {
    fn default() -> MacroAction {
        MacroAction::None
    }
}

#[derive(Clone, Debug, Default)]
/// Struct definition to hold the actions that make up a macro, only works for macro's that
/// fit within one chunk, mostly used for the unit tests that follow below.
struct MacroActions {
    pub macro_id: u16,
    pub position: u32, // byte offset from start of macro actions
    pub event_length_in_bytes: u8,
    pub events: Vec<MacroAction>,
}
impl FromBytes for MacroActions {
    fn from_bytes(&mut self, src: &[u8], endianness: Endianness) -> Result<usize, String> {
        // let mut tmp: MacroActions = Default::default();
        self.macro_id.from_bytes(&src[0..2], endianness)?;
        self.position.from_bytes(&src[2..6], endianness)?;
        self.event_length_in_bytes
            .from_bytes(&src[6..7], endianness)?;
        self.events.clear();
        let mut offset = 6 + 1;
        let offset_max = offset + self.event_length_in_bytes as usize;
        while offset < offset_max {
            let mut action: MacroAction = Default::default();
            offset += action.from_bytes(&src[offset..], endianness)?;
            self.events.push(action);
        }

        Ok(offset)
    }
}

impl ToBytes for MacroActions {
    fn to_bytes(&self, endianness: Endianness) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = Vec::new();
        buff.extend(self.macro_id.to_bytes(endianness)?);
        buff.extend(self.position.to_bytes(endianness)?);

        buff.extend(self.event_length_in_bytes.to_bytes(endianness)?);

        for event in self.events.iter() {
            buff.extend(event.to_bytes(endianness)?);
        }
        Ok(buff)
    }
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug)]
#[repr(C, packed)]
/// Struct definition to hold the actions that make up a macro.
pub struct MacroActionsPayload {
    pub macro_id: u16,
    pub position: u32, // byte offset from start of macro actions
    pub event_bytes_in_msg: u8,
    #[inspect(dissect_additional_type = "bytes", dissection_hide = "true")]
    pub events: [u8; 0x48],
}
impl Default for MacroActionsPayload {
    fn default() -> MacroActionsPayload {
        MacroActionsPayload {
            events: [0; 0x48],
            macro_id: 0,
            position: 0,
            event_bytes_in_msg: 0,
        }
    }
}

pub fn macro_events_to_size(events: &Vec<MacroAction>) -> usize {
    let mut size: usize = 0;
    for event in events.iter() {
        size += event.to_bytes(Endianness::Big).unwrap().len();
    }
    size
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug)]
#[repr(C)]
/// Retrieve the list of macros currently in memory.
pub struct MacroList {
    pub length: u16,
    #[inspect(dissect_additional_type = "bytes", dissection_hide = "true")]
    pub macro_ids: [u16; 0x20], // 0x20 is approx, may have space for one or two more.
}

impl MacroList {
    pub fn to_vec(&self) -> Vec<MacroId> {
        let mut ids: Vec<MacroId> = Vec::new();
        for i in 0..self.length as usize {
            ids.push(self.macro_ids[i])
        }
        ids
    }
}

impl Default for MacroList {
    fn default() -> Self {
        MacroList {
            length: 0,
            macro_ids: [0; 0x20],
        }
    }
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C)]
/// Retrieve the number of macros in memory.
pub struct MacroCount {
    pub count: u16,
}

pub fn macro_events_to_payloads(
    macro_id: MacroId,
    events: &Vec<MacroAction>,
) -> Vec<MacroActionsPayload> {
    // first, convert the events to the payload bytes.
    let mut buff: Vec<u8> = Vec::new();
    for event in events.iter() {
        buff.extend(event.to_bytes(Endianness::Big).unwrap());
    }

    // Cool, now we have bytes, we just have to chunk it and produce our MacroActionsPayloads.
    let mut payloads: Vec<MacroActionsPayload> = Vec::new();
    let chunk_size = 0x48; // set in stone.

    for (i, payload) in buff.chunks(chunk_size).enumerate() {
        let mut cmd = MacroActionsPayload {
            macro_id,
            position: (i * chunk_size) as u32,
            event_bytes_in_msg: payload.len() as u8,
            ..Default::default()
        };
        for (j, b) in payload.iter().enumerate() {
            cmd.events[j] = *b
        }
        payloads.push(cmd);
    }

    payloads
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Uuid {
    pub uuid: [u8; 16],
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C, packed)]
/// Command for the macro metadata 0x060c (incomplete!)
pub struct MacroMetadata {
    pub macro_id: u16,
    pub page_offset: u16,
    pub something_always_0x00fa: u16,
    pub uuid: Uuid,
    pub action_bytes: u32, // is this... another endianness!?!?
                           // pub name: [u8; 12],    // It can be longer....
                           // Lots of more stuff here, which looks... mostly like dirty memory
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct MacroCreate {
    pub macro_id: u16,
    pub event_bytes: u32,
}

#[derive(Inspectable, FromBytes, ToBytes, Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct MacroDelete {
    pub macro_id: u16,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::commands::helpers::{
        parse_wireshark_truncated, parse_wireshark_value, PAYLOAD_START,
    };

    /*
        On macro metadata; seems to be a... lot of stuff.
        eb:69:30:b1:54:e6:e2:49:8b:6c:20:41:37:cc:47:86:06:00:00:00:61:61:61:00:00:00:00:00:00:00:00:00:50:71:10:06:00:....
                     uuid                              |nr of evnts|name                               | *
        *) Looks like dirty memory at times... perhaps a binary blob representing something?
        There is an f(uuid) -> MacroId function.

        For uuid that consist of just a single byte and the rest null bytes, the uuid can be
        determined with (Python code):

        factors = [1, 3, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 67, 79, 92, 106]
        v = []
        for i in range(len(uid)):
            v.append(factors[i] * uid[i])
        v.append(120)
        macro_id = sum(v)

        This factor sequence is very close to the Lazy Caterer's sequence, just having an extra 3
        on the second index; https://oeis.org/A000124
        [1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 67, 79, 92, 106] # 121

        The keyboard itself seems to give 0 F's about the macro metadata, just allocating a macro,
        giving it actions and assigning a key mapping to it makes it work.
    */

    #[test]
    fn test_macro_events() {
        // This is the actual macro payload, not the metadata, it is chunked in blocks.
        let shift_b = parse_wireshark_truncated(
            "00:1f:00:00:00:0f:06:09:3b:68:00:00:00:00:08:01:e1:01:05:02:05:02:e1",
            0x5b,
        );
        let cmd = MacroActions::from_be_bytes(&shift_b[PAYLOAD_START..]).expect("Should pass");
        println!("{:?}", cmd);
        let and_back = cmd.to_be_bytes().expect("Success");
        assert_eq!(and_back.len(), 15);
        println!("{:?}", and_back);
        assert_eq!(
            &shift_b[PAYLOAD_START..PAYLOAD_START + and_back.len()],
            and_back
        );

        let delay_b_a = parse_wireshark_truncated(
            "00:1f:00:00:00:12:06:09:3b:68:00:00:00:00:0b:12:03:e8:01:05:02:05:01:04:02:04",
            0xbc,
        );
        let cmd = MacroActions::from_be_bytes(&delay_b_a[PAYLOAD_START..]).expect("Should pass");
        println!("{:?}", cmd);
        let and_back = cmd.to_be_bytes().expect("Success");
        assert_eq!(and_back.len(), 18);
        println!("{:?}", and_back);
        assert_eq!(
            &delay_b_a[PAYLOAD_START..PAYLOAD_START + and_back.len()],
            and_back
        );

        let parsing_breaks_1 = parse_wireshark_truncated("00:1f:00:00:00:18:06:09:3b:68:00:00:00:00:11:11:fa:01:05:13:01:d4:c0:02:05:12:13:88:01:04:02:04:00", 0x31);
        let cmd =
            MacroActions::from_be_bytes(&parsing_breaks_1[PAYLOAD_START..]).expect("Should pass");
        let and_back = cmd.to_be_bytes().expect("Success");
        assert_eq!(and_back.len(), 24);
        assert_eq!(
            &parsing_breaks_1[PAYLOAD_START..PAYLOAD_START + and_back.len()],
            and_back
        );

        let set_mouse_stroke_left = parse_wireshark_truncated(
            "00:1f:00:00:00:0b:06:09:3b:68:00:00:00:00:04:08:01:08:00",
            0x52,
        );
        let cmd = MacroActions::from_be_bytes(&set_mouse_stroke_left[PAYLOAD_START..])
            .expect("Should pass");
        let and_back = cmd.to_be_bytes().expect("Success");
        assert_eq!(and_back.len(), 11);
        assert_eq!(
            &set_mouse_stroke_left[PAYLOAD_START..PAYLOAD_START + and_back.len()],
            and_back
        );

        let set_mouse_scroll_up = parse_wireshark_value("0a:01");
        let mouse_up = MacroAction::from_le_bytes(&set_mouse_scroll_up).expect("success");
        assert_eq!(mouse_up, MacroAction::MouseScroll(1));
        let and_back = mouse_up.to_be_bytes().expect("Success");
        assert_eq!(set_mouse_scroll_up, and_back);

        let set_mouse_scroll_down = parse_wireshark_value("0a:ff");
        let mouse_down = MacroAction::from_le_bytes(&set_mouse_scroll_down).expect("success");
        assert_eq!(mouse_down, MacroAction::MouseScroll(-1));
        let and_back = mouse_down.to_be_bytes().expect("Success");
        assert_eq!(set_mouse_scroll_down, and_back);

        let mouse_move_action_input = parse_wireshark_value("15:00:01:ff:ff");
        let mouse_move_action =
            MacroAction::from_le_bytes(&mouse_move_action_input).expect("success");
        assert_eq!(mouse_move_action, MacroAction::MouseMove { x: 1, y: -1 });
        let and_back = mouse_move_action.to_be_bytes().expect("Success");
        assert_eq!(mouse_move_action_input, and_back);
    }

    #[test]
    fn test_macro_create() {
        // This is the create macro command, it takes an id and the number of bytes for the actual
        // events. The metadata is not counted in this.
        let long_create =
            parse_wireshark_truncated("00:1f:00:00:00:06:06:08:1a:2e:00:00:28:cd:00", 0xd9);
        let parsed = MacroCreate::from_be_bytes(&long_create[PAYLOAD_START..]).expect("success");
        let macro_id = parsed.macro_id; // copy to avoid borrowing from packed struct
        let event_bytes = parsed.event_bytes;
        assert_eq!(macro_id, 0x1a2e);
        assert_eq!(event_bytes, 0x28cd);

        let short_create =
            parse_wireshark_truncated("0:1f:00:00:00:06:06:08:7f:39:00:00:00:04:00", 0x4a);
        let parsed = MacroCreate::from_be_bytes(&short_create[PAYLOAD_START..]).expect("success");
        let macro_id = parsed.macro_id;
        let event_bytes = parsed.event_bytes;
        assert_eq!(macro_id, 0x7f39);
        assert_eq!(event_bytes, 0x04);
    }

    // checking against a packed struct, don't want warnings about references to it.
    macro_rules! copied_assert_eq {
        ( $a:expr, $b:expr ) => {{
            let a = $a;
            let b = $b;
            assert_eq!(a, b);
        }};
    }
    #[test]
    fn test_macro_payloads() {
        // Trivial macro that fits in one chunk.
        let mouse_click = vec![
            MacroAction::MouseClick(MouseState::Left),
            MacroAction::MouseClick(MouseState::None),
        ];
        let payloads = macro_events_to_payloads(0x7f39, &mouse_click);
        println!("Payloads: {:?}", payloads);
        let as_bytes = payloads.first().unwrap().to_be_bytes().expect("Success");
        let expected = parse_wireshark_truncated(
            "00:1f:00:00:00:0b:06:09:7f:39:00:00:00:00:04:08:01:08:00",
            0x47,
        );
        assert_eq!(as_bytes, &expected[PAYLOAD_START..expected.len() - 3]);
        assert_eq!(macro_events_to_size(&mouse_click), 0x04);

        // Larger macro that spans two chunks, don't have hard data to test against, check the indices
        // against expectation.
        let mut actions: Vec<MacroAction> = Vec::new();
        for _i in 0..20 {
            actions.push(MacroAction::MouseClick(MouseState::Left));
            actions.push(MacroAction::MouseClick(MouseState::None));
        }
        let payloads = macro_events_to_payloads(0x1337, &actions);
        let total_bytes = macro_events_to_size(&actions);
        println!("Payloads: {:?}", payloads);
        copied_assert_eq!(payloads.len(), 2);
        copied_assert_eq!(payloads[0].macro_id, 0x1337);
        copied_assert_eq!(payloads[1].macro_id, 0x1337);
        copied_assert_eq!(payloads[0].position, 0);
        copied_assert_eq!(payloads[1].position, 0x48 as u32);
        copied_assert_eq!(payloads[0].event_bytes_in_msg, 0x48 as u8);
        copied_assert_eq!(payloads[1].event_bytes_in_msg, (total_bytes - 0x48) as u8);
        copied_assert_eq!(total_bytes, 20 * 2 * 2); // each mouse action is 2 bytes.
    }

    fn print_serialize<T: Serialize + std::fmt::Debug>(v: T) -> String {
        let serialized = serde_json::to_string(&v).unwrap();
        println!("serialize {:?} -> {}", v, serialized);
        serialized
    }
    fn print_deserialize<'a, T: Deserialize<'a> + Sized + std::fmt::Debug>(v: &'a str) -> T {
        let deserialized: T = serde_json::from_str(&v).unwrap();
        println!("deserialize {} -> {:?}", v, deserialized);
        deserialized
    }

    #[test]
    pub fn test_macro_serialize() {
        print_serialize(MacroAction::MouseClick(MouseState::Left));
        print_serialize(MacroAction::KeyboardMake { hid: 0x04 });
        print_serialize(vec![
            MacroAction::KeyboardMake { hid: 0x04 },
            MacroAction::KeyboardBreak { hid: 0x04 },
        ]);

        print_deserialize::<MacroAction>(r#"{"keyboard_make":"KEY_A"}"#);
    }

    #[test]
    pub fn test_macro_get_list() {
        let expected =
            parse_wireshark_truncated("02:1f:00:00:00:ca:06:81:00:02:13:37:13:36:00", 0x4e);
        let parsed = MacroList::from_be_bytes(&expected[PAYLOAD_START..]).expect("success");
        assert_eq!(parsed.length, 2);
        assert_eq!(parsed.macro_ids[0], 0x1337);
        assert_eq!(parsed.macro_ids[1], 0x1336);
        assert_eq!(parsed.to_vec(), vec!(0x1337, 0x1336));
    }
}
