//! This crate holds the communication details and provides the available commands.

use struct_helper::StructHelper;
/// This module holds the structs as they actually go over the USB bus.
pub mod wire;
use wire::RGB;

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
        ^^ is part of the payload? But we only ever see 00, 01, 03 going to the device, see ff coming back rarely. <-0 this is '_first' in most commands.
*/

/// Represents a command that can be sent over USB.
pub trait Command: std::fmt::Debug {
    /// Should provider the raw bytes that are to be sent to the device over usb.
    fn serialize(&self) -> Vec<u8> {
        // Now follows the command.
        let cmd = self.register();
        let payload = self.payload();

        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::Command>()];
        let mut wire: wire::Command = Default::default();
        wire.cmd_major = cmd.0;
        wire.cmd_minor = cmd.1;
        wire.len = payload.len() as u8;

        // copy the payload.
        for i in 0..payload.len() {
            wire.payload[i] = payload[i];
        }
        wire.update_checksum(); // update the checksum based on the currently populated values.
        wire.to_le_bytes(&mut v[..]).expect("Should succeed"); // serialize the struct.
        return v;
    }

    /// Provides the two register addressses that are to be sent in the header.
    fn register(&self) -> (u8, u8);

    /// Provides the payload definition that comes after the header.
    fn payload(&self) -> Vec<u8>;
}

#[derive(Default, Copy, Clone, Debug)]
/// Sets the LED State, providing a direct RGB value for each individual led.
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
pub struct SetLedState {
    /// Id seems to denote the group id of leds, either row or location on the border.
    pub id: u8,

    /// Seems to be specifying up to which column?
    pub count: u8,

    /// The actual values of the leds here.
    pub leds: [RGB; 23], // 22 is the max seen?, corresponds with 0x16 in the count position.
}
impl SetLedState {
    pub const CMD: (u8, u8) = (0x0f, 0x03);
}

impl Command for SetLedState {
    fn register(&self) -> (u8, u8) {
        return SetLedState::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::SetLedState>()];
        let wire_ledstate: wire::SetLedState = wire::SetLedState {
            first: 0,
            id: self.id,
            count: self.count,
            leds: self.leds,
            ..Default::default()
        };
        wire_ledstate
            .to_le_bytes(&mut v[..])
            .expect("Should succeed");
        v
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Sets the brightness value for all leds.
pub struct SetBrightness {
    /// The brightness to set, should be [0, 1].
    pub value: f32,
}
impl SetBrightness {
    pub const CMD: (u8, u8) = (0x0f, 0x04);
}

impl Command for SetBrightness {
    fn register(&self) -> (u8, u8) {
        return SetBrightness::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::SetBrightness>()];
        let wire_setbrightness: wire::SetBrightness = wire::SetBrightness {
            first: 0x01,
            value: (self.value * 255.0) as u8,
            ..Default::default()
        };
        wire_setbrightness
            .to_le_bytes(&mut v[..])
            .expect("Should succeed");
        v
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Toggles game mode on or off.
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
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::SetGameMode>()];
        let wire_cmd = wire::SetGameMode {
            game_mode_enabled: self.value as u8,
            ..Default::default()
        };
        wire_cmd
            .to_le_bytes(&mut v[..])
            .expect("Should succeed");
        v
    }
}

#[derive(Default, Clone, Debug)]
/// Sends an arbitrary payload to a register, use with caution, useful for testing.
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

/// Helper function for the dissector that provides the fields for the provided commands.
pub fn get_command_fields() -> Vec<((u8, u8), Box<dyn Fn() -> struct_helper::Field>)> {
    vec![
        (SetLedState::CMD, Box::new(wire::SetLedState::fields)),
        (SetBrightness::CMD, Box::new(wire::SetBrightness::fields)),
        (SetGameMode::CMD, Box::new(wire::SetGameMode::fields)),
    ]
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
