use struct_helper::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseButton {
    None = 0,
    Left = 1,
    Right = 2,
    Scroll = 4,
    M4 = 8,
    M5 = 16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Enum to represent an action in a macro.
pub enum MacroAction {
    /// HID key id.
    KeyboardMake(u8),
    /// HID key id.
    KeyboardBreak(u8),
    /// Delay in milliseconds.
    Delay(u32),
    /// Sets the mouse click state (bitmask), use again with 0 to release
    MouseClick(MouseButton),
    /// scroll with the mouse
    MouseScroll(i8),
    /// move mouse relative, with x and y values.
    MouseMove(i16, i16),
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
                *self = MacroAction::KeyboardMake(src[1]);
                return Ok(2);
            }
            MacroAction::KEYBOARD_BREAK => {
                *self = MacroAction::KeyboardBreak(src[1]);
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
                    i if i == MouseButton::None as u8 => MouseButton::None,
                    i if i == MouseButton::Left as u8 => MouseButton::Left,
                    i if i == MouseButton::Right as u8 => MouseButton::Right,
                    i if i == MouseButton::Scroll as u8 => MouseButton::Scroll,
                    i if i == MouseButton::M4 as u8 => MouseButton::M4,
                    i if i == MouseButton::M5 as u8 => MouseButton::M5,
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
                *self = MacroAction::MouseMove(i16::from_be_bytes(x), i16::from_be_bytes(y));
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
            MacroAction::KeyboardMake(v) => {
                buff.push(MacroAction::KEYBOARD_MAKE);
                buff.push(*v);
            }
            MacroAction::KeyboardBreak(v) => {
                buff.push(MacroAction::KEYBOARD_BREAK);
                buff.push(*v);
            }
            MacroAction::Delay(v) => {
                let b = v.to_be_bytes()?;
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
            MacroAction::MouseScroll(value) => {
                buff.push(MacroAction::MOUSE_SCROLL);
                buff.push(i8::to_le_bytes(*value)[0]);
            }
            MacroAction::MouseMove(x, y) => {
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::helpers::{parse_wireshark_truncated, parse_wireshark_value, PAYLOAD_START};

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
        assert_eq!(mouse_move_action, MacroAction::MouseMove(1, -1));
        let and_back = mouse_move_action.to_be_bytes().expect("Success");
        assert_eq!(mouse_move_action_input, and_back);
    }
}
