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
    pub leds: [RGB; 23],
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
/// Payload for the SetLedBrightness command.
pub struct SetLedBrightness {
    pub profile: u8,
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
    pub profile: u8,
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

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Uuid {
    pub uuid: [u8; 16],
}

#[derive(Inspectable, FromBytes, ToBytes, Default, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct MacroMetadata {
    pub macro_id: u16,
    pub page_offset: u16,
    pub something_always_0x00fa: u16,
    pub uuid: Uuid,
    pub action_bytes: u32, // is this... another endianness!?!?
                           // pub name: [u8; 12],    // It can be longer....
                           // Lots of more stuff here, which looks... mostly like dirty memory
}
