// This file is ugly...

fn prepare_checksum(v: &Vec<u8>) -> u8 {
    let mut checksum: u8 = 0;
    for i in 2..v.len() {
        checksum ^= v[i];
    }
    return checksum;
}

pub trait Command: std::fmt::Debug {
    fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(0);
        v.push(0x1f);
        v.push(0x0);
        v.push(0x0);
        v.push(0x0);

        // Now follows the command.
        let cmd = self.command_id();
        v.push((cmd >> (8 * 3)) as u8);
        v.push((cmd >> (8 * 2)) as u8);
        v.push((cmd >> (8 * 1)) as u8);
        v.push((cmd >> 0) as u8);

        v.append(&mut self.payload());

        // After the payload, we pad up to 88 bytes.
        while v.len() < 88 {
            v.push(0);
        }

        let checksum = prepare_checksum(&v);
        v.push(checksum);
        v.push(0);

        return v;
    }

    fn command_id(&self) -> u32;
    fn payload(&self) -> Vec<u8>;
}

#[derive(Default, Copy, Clone, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl RGB {
    pub fn payload(&self) -> Vec<u8> {
        return vec![self.r, self.g, self.b];
    }
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
    pub leds: [RGB; 22], // 22 is the max seen?, corresponds with 0x16 in the count position.
}
impl SetLedState {
    pub const CMD: u32 = 0x4a0f0300;
}

impl Command for SetLedState {
    fn command_id(&self) -> u32 {
        return SetLedState::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(0); // 0 byte
        v.push(self.id);
        v.push(0); // another 0.
        v.push(self.count); // Count of leds, though always 0x16 in data seen.
        for rgb in self.leds.iter() {
            v.append(&mut rgb.payload());
        }
        return v;
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SetBrightness {
    pub value: f32,
}
impl SetBrightness {
    pub const CMD: u32 = 0x030f0401;
}

impl Command for SetBrightness {
    fn command_id(&self) -> u32 {
        return SetBrightness::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(0); // 0 byte
        v.push((self.value * 255.0) as u8);
        return v;
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SetGameMode {
    pub value: bool,
}
impl SetGameMode {
    pub const CMD: u32 = 0x03030000;
}

impl Command for SetGameMode {
    fn command_id(&self) -> u32 {
        return SetGameMode::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(8); // No idea what this 8 means... :/
        v.push(self.value as u8);
        return v;
    }
}

#[derive(Default, Clone, Debug)]
pub struct ArbitraryCommand {
    pub cmd: u32,
    pub payload: Vec<u8>,
}

impl Command for ArbitraryCommand {
    fn command_id(&self) -> u32 {
        return self.cmd;
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
