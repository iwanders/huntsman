use struct_helper::*;

#[derive(StructHelper, Default, Copy, Clone, Debug)]
/// Denotes a Red, Green and Blue color value.
pub struct RGB {
    /// Value of the red channel [0, 255]
    pub r: u8,
    /// Value of the green channel [0, 255]
    pub g: u8,
    /// Value of the blue channel [0, 255]
    pub b: u8,
}

#[derive(StructHelper, Copy, Clone, Debug)]
#[repr(C)]
/// The command header.
pub struct Command {
    pub status: u8, // status, direction? Only really seen 0, 2 and I think 5 when I was throwing random data it it all.
    pub the_1f: u8, // Almost always 1f.
    pub _three: [u8; 3], // these bytes always seem to be zero, ALWAYS
    pub len: u8,
    pub cmd_major: u8,
    pub cmd_minor: u8,
    #[struct_helper(dissection_hide = "true")]
    pub payload: [u8; 80],
    pub checksum: u8,
    pub _closing: u8,
}

impl Command {
    /// Direct implementation to update the checksum based on the currently populated fields.
    pub fn update_checksum(&mut self) {
        self.checksum = 0;
        self.checksum ^= self.len;
        self.checksum ^= self.cmd_major;
        self.checksum ^= self.cmd_minor;
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
            cmd_major: 0,
            cmd_minor: 0,
            payload: [0; 80],
            checksum: 0,
            _closing: 0,
        }
    }
}

#[derive(StructHelper, Default, Copy, Clone, Debug)]
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

#[derive(StructHelper, Default, Copy, Clone, Debug)]
#[repr(C)]
/// Payload for the SetBrightness command.
pub struct SetBrightness {
    pub first: u8,
    pub _p0: u8, // padding
    pub value: u8,
}

#[derive(StructHelper, Copy, Clone, Debug)]
#[repr(C)]
/// Payload for the SetBrightness command.
pub struct SetGameMode {
    pub first: u8,
    pub something: u8, // no idea
    pub game_mode_enabled: u8,
}
impl Default for SetGameMode {
    fn default() -> SetGameMode {
        SetGameMode {
            first: 0,
            something: 8,
            game_mode_enabled: 0,
        }
    }
}

