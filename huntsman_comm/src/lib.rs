use struct_helper::*;

/*
keyboard_sniffs$ ./dump.sh 2021 | sort | uniq > /tmp/sorted_all.txt
Shows; 1f is not always 1f, sometimes it's 08

0x01058a05	02:1f:00:00:00:01:05:8a:05:00...
0x02058101	02:1f:00:00:00:02:05:81:01:01:00...
0x030f0401	00:1f:00:00:00:03:0f:04:01:00:ff:00...
0x030f8401	02:1f:00:00:00:03:0f:84:01:05:ff:00...
0x05028d03	02:1f:00:00:00:05:02:8d:03:3e:00:00:00...
0x050f8005	02:1f:00:00:00:05:0f:80:05:19:03:0c:17:00...
0x06028d02	02:1f:00:00:00:06:02:8d:02:7a:01:11:01:09:00...
0x07028d02	02:1f:00:00:00:07:02:8d:02:01:00:02:02:00:35:00...
  ^^----- len                from here  01 02 03 04 05 06 07
    ^^ --- group?
      ^^ -- register?
      8   = 1 << 7, msb write / read flag?
        ^^ is part of the payload? But we only ever see 00, 01, 03 going to the device, see ff coming back rarely.
*/

#[derive(StructHelper, Copy, Clone, Debug)]
#[repr(C)]
pub struct WireCommand
{
    pub status: u8, // status, direction? Only really seen 0, 2 and I think 5 when I was throwing random data it it all.
    pub the_1f: u8, // Almost always 1f.
    _three: [u8; 3], // these bytes always seem to be zero, ALWAYS
    pub len: u8,
    pub cmd_0: u8,
    pub cmd_1: u8,
    pub payload: [u8; 80],
    pub checksum: u8,
    _closing: u8,
}

impl WireCommand
{
    /// Direct implementation to update the checksum based on the currently populated fields.
    pub fn update_checksum(&mut self)
    {
        self.checksum = 0;
        self.checksum ^= self.len;
        self.checksum ^= self.cmd_0;
        self.checksum ^= self.cmd_1;
        for i in 0..self.payload.len() {
            self.checksum ^= self.payload[i];
        }
    }
}

impl Default for WireCommand {
    fn default() -> WireCommand {
        WireCommand {
            status: 0,
            the_1f: 0x1f,
            _three: [0, 0, 0],
            len: 0,
            cmd_0: 0,
            cmd_1: 0,
            payload: [0; 80],
            checksum: 0,
            _closing: 0,
        }
    }
}

// Todo; Clean up this monster.
pub trait Command: std::fmt::Debug {
    fn serialize(&self) -> Vec<u8> {
        // Now follows the command.
        let cmd = self.register();
        let payload = self.payload();

        let mut v: Vec<u8> = vec![0; std::mem::size_of::<WireCommand>()];
        let mut wire: WireCommand = Default::default();
        wire.cmd_0 = cmd.0;
        wire.cmd_1 = cmd.1;
        wire.len = payload.len() as u8;

        // copy the payload.
        for i in 0..payload.len()
        {
            wire.payload[i] = payload[i];
        }
        wire.update_checksum();  // update the checksum based on the currently populated values.
        wire.to_le_bytes(&mut v[..]).expect("Should succeed");  // serialize the struct.
        return v;
    }

    fn register(&self) -> (u8, u8);
    fn payload(&self) -> Vec<u8>;
}

#[derive(StructHelper, Default, Copy, Clone, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SetLedState {
    /// Seems to be row.
    ///                     
    /// |  key row             | 0| 1| 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9|10|11|12|13|14|15|16|17|18|19|20|21|22|
    /// |----------------------|--|  |   |   |   |   |   |   |   |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
    /// |  0          |  |esc  |   | f1  | f2  | f3  | f4  |f5   | f6  | f7 | f8 | f9  |  f10 | f11  | f12 | prt | scrl | pause | prev  | play | next | volume  |  |
    /// |  1          |  | ` |  1 | 2  |  3 | 4 | 5  | 6  | 7  | 8 | 9  | 0  | -  | =  | bksp | ins | home | pup | nmlk | / | * | - |  |
    /// |  2          |  | tab  | q  | w  | e  |  r | t  | y  | u  | i | o | p | [ | ] | \ | del | end  | pwnd | 7 | 8 | 9 | + |  |
    /// |  3          |  | caps | a  | s  | d  |  f | g  | h  | j  | k | l | ; | ' |  | enter |  |  |  | 4 | 5 | 6 |  |  |
    /// |  4          |  | shift  | z  | x  | c  | v  | b  | n  | m  | , | . | / |  | shift |  |  | up |  | 1 | 2 | 3 | enter |  |
    /// |  5          |  | ctrl | win  | alt  |   |   |   | space  |   |  |  | alt | fn | context | ctrl | left | down | right | 0 |  | del |  |  |
    /// |  6*          | ` | f3 |  f4 | f5  |  f7 | f8  | f9  | f10  | f11  | f12 | prt | scrl | pause | play | vol | - | + | enter | del |  |  |  |  |
    /// |  7*          | `? | tab  | caps | shift  | ctrl  |  win | alt  | llspace  | lspace  | rspace | rrspace | alt | fn | context | ctrl | left | down | right | 0 |  |  |  |  |
    /// |  8$          | 0| farleft | midleft  | frontleft  | alt  |  f | b  |  n | k | alt | fn | shift | ctrl | left | right | 0 | 5 | farright | midright | frontright |  |  |  |
    ///
    ///  \[*\] This is the edge lighting, not the keys.
    ///  \[$\] Armrest
    pub id: u8,
    /// Seems to be specifying up to which column?
    pub count: u8,
    pub leds: [RGB; 23], // 22 is the max seen?, corresponds with 0x16 in the count position.
}
impl SetLedState {
    pub const CMD: (u8, u8) = (0x0f, 0x03);
}

#[derive(StructHelper, Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct WireSetLedState
{
    first: u8,
    _p0: u8,  // padding
    pub id: u8,
    _p1: u8,  // padding
    /// Seems to be specifying up to which column?
    pub count: u8,
    pub leds: [RGB; 23], // 22 is the max seen?, corresponds with 0x16 in the count position.
}

impl Command for SetLedState {
    fn register(&self) -> (u8, u8) {
        return SetLedState::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<WireSetLedState>()];
        let wire_ledstate: WireSetLedState = WireSetLedState{first: 0, id: self.id, count: self.count, leds: self.leds, ..Default::default()};
        wire_ledstate.to_le_bytes(&mut v[..]).expect("Should succeed");
        v
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SetBrightness {
    pub value: f32,
}
impl SetBrightness {
    pub const CMD: (u8, u8) = (0x0f, 0x04);
}
#[derive(StructHelper, Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct WireSetBrightness
{
    pub first: u8,
    _p0: u8,  // padding
    pub value: u8
}


impl Command for SetBrightness {
    fn register(&self) -> (u8, u8) {
        return SetBrightness::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<WireSetBrightness>()];
        let wire_setbrightness: WireSetBrightness = WireSetBrightness{first: 0x01, value: (self.value * 255.0) as u8, ..Default::default()};
        wire_setbrightness.to_le_bytes(&mut v[..]).expect("Should succeed");
        v
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SetGameMode {
    pub value: bool,
}
impl SetGameMode {
    pub const CMD: (u8, u8) = (0x03, 0x00);
}

impl Command for SetGameMode {
    fn register(&self) -> (u8, u8) {
        return SetGameMode::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(0); // the first byte is zero.
        v.push(8); // No idea what this 8 means... :/
        v.push(self.value as u8);
        return v;
    }
}

#[derive(Default, Clone, Debug)]
pub struct ArbitraryCommand {
    pub register: (u8, u8),
    pub payload: Vec<u8>,
}

impl Command for ArbitraryCommand {
    fn register(&self) -> (u8, u8) {
        return self.register;
    }
    fn payload(&self) -> Vec<u8> {
        return self.payload.clone();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[cfg(test)]
    fn parse_wireshark_value(z: &str) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        let bytes = z.split(":");
        for b in bytes {
            match u8::from_str_radix(b, 16) {
                Ok(number) => r.push(number),
                Err(e) => panic!("{}; {:?} (full: {:?})", e, b, z),
            };
        }
        return r;
    }

    #[test]
    fn test_set_led_state() {
        let expected = parse_wireshark_value("00:1f:00:00:00:4a:0f:03:00:00:06:00:16:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:a9:00");
        // This is a command for led id 6, with red, except for last 4 bytes, they are dark.
        let mut state: SetLedState = Default::default();
        state.id = 6;
        state.count = 0x16;
        for i in 0..state.count as usize {
            if i <= (state.count - 4).into() {
                state.leds[i].r = 0xff;
            }
        }
        assert_eq!(state.serialize(), expected);
    }

    #[test]
    fn test_set_brightness() {
        let expected_50_pct = parse_wireshark_value("00:1f:00:00:00:03:0f:04:01:00:7f:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:76:00");
        let mut brightness: SetBrightness = Default::default();
        brightness.value = 0.5;
        assert_eq!(brightness.serialize(), expected_50_pct);

        let expected_100_pct = parse_wireshark_value("00:1f:00:00:00:03:0f:04:01:00:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:f6:00");
        brightness.value = 1.0;
        assert_eq!(brightness.serialize(), expected_100_pct);

        brightness.value = 2.5; // cool, 'as u8' clamps.
        assert_eq!(brightness.serialize(), expected_100_pct);
    }

    #[test]
    fn test_set_game_mode() {
        let enable = parse_wireshark_value("00:1f:00:00:00:03:03:00:00:08:01:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:09:00");
        let mut game_mode: SetGameMode = Default::default();
        game_mode.value = true;
        assert_eq!(game_mode.serialize(), enable);

        let disable = parse_wireshark_value("00:1f:00:00:00:03:03:00:00:08:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:08:00");
        game_mode.value = false;
        assert_eq!(game_mode.serialize(), disable);
    }
}
