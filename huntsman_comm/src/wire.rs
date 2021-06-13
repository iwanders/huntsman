use struct_helper::*;

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
/// Denotes a Red, Green and Blue color value.
pub struct RGB {
    /// Value of the red channel [0, 255]
    pub r: u8,
    /// Value of the green channel [0, 255]
    pub g: u8,
    /// Value of the blue channel [0, 255]
    pub b: u8,
}

#[derive(Inspectable, FromBytes, ToBytes, Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(C)]
/// Struct to denote a command registers.
pub struct Cmd {
    /// Major seems to group them `0x0f` is led-related, `0x02` is key bindings.
    pub major: u8,
    /// Minor is the subregister inside that group.
    pub minor: u8,
}

#[derive(Inspectable, FromBytes, ToBytes, Copy, Clone, Debug)]
#[repr(C)]
/// The command header.
pub struct Command {
    pub status: u8, // status, direction? Only really seen 0, 2 and I think 5 when I was throwing random data it it all.
    pub the_1f: u8, // Almost always 1f.
    pub _three: [u8; 3], // these bytes always seem to be zero, ALWAYS
    pub len: u8,
    #[inspect(dissect_additional_type = "u16")] // Also dissect this as an u16.
    pub cmd: Cmd,
    #[inspect(dissection_hide = "true")]
    pub payload: [u8; 80],
    pub checksum: u8,
    pub _closing: u8,
}

impl Command {
    /// Direct implementation to update the checksum based on the currently populated fields.
    pub fn update_checksum(&mut self) {
        self.checksum = 0;
        self.checksum ^= self.len;
        self.checksum ^= self.cmd.major;
        self.checksum ^= self.cmd.minor;
        for i in 0..self.payload.len() {
            self.checksum ^= self.payload[i];
        }
    }
}

impl Default for Command {
    fn default() -> Command {
        Command {
            status: 0,
            the_1f: 0x1f,
            _three: [0, 0, 0],
            len: 0,
            cmd: Cmd { major: 0, minor: 0 },
            payload: [0; 80],
            checksum: 0,
            _closing: 0,
        }
    }
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
/// Payload for the SetLedState command
pub struct SetLedState {
    pub first: u8,
    pub _p0: u8, // padding
    pub id: u8,
    pub _p1: u8, // padding
    /// Seems to be specifying up to which column?
    pub count: u8,
    pub leds: [RGB; 23], // 22 is the max seen?, corresponds with 0x16 in the count position.
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
/// Payload for the SetLedBrightness command.
pub struct SetLedBrightness {
    pub first: u8,
    pub _p0: u8, // padding
    pub value: u8,
}

#[derive(Inspectable, FromBytes, ToBytes, Copy, Clone, Debug)]
#[repr(C)]
/// Payload to set the game mode.
pub struct SetGameMode {
    pub first: u8,
    // No idea, either 0x18, or 0x08, 18 is the volume led... Maybe its just SetLedFlag?
    // 0x18 seems to be sent as cancelOTF?
    pub something: u8,
    pub game_mode_enabled: u8,
}
impl Default for SetGameMode {
    fn default() -> SetGameMode {
        SetGameMode {
            first: 0,
            something: 0x8,
            game_mode_enabled: 0,
        }
    }
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
/// Payload to set the key override.
pub struct SetKeyOverride {
    pub first: u8,
    pub key_code: u8,
    pub hypershift: u8,
    pub mapping_type: u8,
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
/// Payload to set an led effect.
pub struct SetLedEffect {
    pub first: u8,
    pub second: u8,
    pub effect: u8,
    pub direction: u8,
    pub speed: u8,
    pub color_count: u8,
    pub colors: [RGB; 10], // not sure how long this one is.
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C, packed)]
/// Payload to set an led effect.
pub struct GetStorageStatistics {
    pub something: u16,
    #[inspect(dissection_display = "dec")]
    pub total: u32,
    #[inspect(dissection_display = "dec")]
    pub free1: u32,
    #[inspect(dissection_display = "dec")]
    pub free2: u32,
}

// What follows is _really_ ugly :(
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MacroAction {
    KeyboardMake(u8),
    KeyboardBreak(u8),
    Delay(u32),
    None,
}

impl MacroAction {
    const KEYBOARD_MAKE: u8 = 0x01;
    const KEYBOARD_BREAK: u8 = 0x02;

    const KEYBOARD_DELAY_U8: u8 = 0x11;
    const KEYBOARD_DELAY_U16: u8 = 0x12;
    const KEYBOARD_DELAY_U24: u8 = 0x13; // for real... :(
    const KEYBOARD_DELAY_U32: u8 = 0x14;
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
                let mut arr : [u8; 4] = [0; 4];
                // Now copy the correct number of bytes to the correct location
                for i in 0..get_delay_byte_length
                {
                    arr[i + get_delay_byte_length] = src[1 + i];
                }
                // Now, we interpret this as big endian.
                *self = MacroAction::Delay(u32::from_be_bytes(arr));
                return Ok(1 + get_delay_byte_length);
            }
            z => panic!("Unhandled macro code {:?}", z),
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
                if b[0] != 0 // need 4 bytes
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U32);
                    buff.extend(b.to_vec());
                }
                else if b[1] != 0  // 3 byte
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U24);
                    buff.extend(b[1..].to_vec());
                }
                else if b[2] != 0  // 2 byte
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U16);
                    buff.extend(b[2..].to_vec());
                }
                else if b[3] != 0  // 1 byte
                {
                    buff.push(MacroAction::KEYBOARD_DELAY_U16);
                    buff.extend(b[3..].to_vec());
                }
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

// Inspectable, FromBytes, ToBytes, Default,   <- This doesn't work with the MacroEvents type.... :(
#[derive(Clone, Debug, Default)]
pub struct MacroActions {
    pub macro_id: u16,
    pub _pad: u8,
    pub byte_count: u32,
    pub events: Vec<MacroAction>,
}

impl FromBytes for MacroActions {
    fn from_bytes(&mut self, src: &[u8], endianness: Endianness) -> Result<usize, String> {
        // let mut tmp: MacroActions = Default::default();
        self.macro_id.from_bytes(&src[0..2], endianness)?;
        self.byte_count.from_bytes(&src[3..7], endianness)?;
        self.events.clear();
        let mut offset = 6 + 1;
        let offset_max = offset + self.byte_count as usize;
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
        buff.push(0);
        buff.extend(self.byte_count.to_bytes(endianness)?);
        for event in self.events.iter() {
            buff.extend(event.to_bytes(endianness)?);
        }
        Ok(buff)
    }
}
