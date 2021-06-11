//! This crate holds the communication details and provides the available commands.

use struct_helper::StructHelper;
/// The wire module holds the structs as they actually go over the USB bus.
pub mod wire;
pub use wire::Cmd;
pub use wire::RGB;

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
        wire.cmd = cmd;
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
    fn register(&self) -> Cmd;

    /// Provides the payload definition that comes after the header.
    fn payload(&self) -> Vec<u8>;
}

// 1, short, 2 medium, 3 long, 3 values matches slider in ui.
#[repr(u8)]
/// Duration, short, medium or long, like the slider in the ui.
pub enum Duration {
    Short = 0x01,
    Medium = 0x02,
    Long = 0x03,
}

#[derive(Default, Copy, Clone, Debug)]
/// An instruction to set an LED effect on the keyboard.
pub struct SetLedEffect {
    pub payload: wire::SetLedEffect,
}
impl SetLedEffect {
    pub const CMD: Cmd = Cmd {
        major: 0x0f,
        minor: 0x02,
    };

    /// Helper function to populate the payload with zero or more colors.
    fn set_colors(&mut self, colors: &Vec<RGB>) {
        self.payload.color_count = std::cmp::min(colors.len(), self.payload.colors.len()) as u8;
        for i in 0..self.payload.color_count as usize {
            self.payload.colors[i] = colors[i];
        }
    }

    /// Disables effects.
    pub fn off() -> SetLedEffect {
        // payload: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // led effects off?
        Default::default()
    }

    /// Sets a static color over the entire keyboard.
    pub fn fixed(color: &RGB) -> SetLedEffect {
        // static, 0xAA = R, 0x44 = G, 0xBB = B
        // payload: vec![0x00, 0x00, 0x01, 0x00, 0x00, 0x01, 0xAA, 0x44, 0xBB],
        //                                               ^ must be non zero...

        let mut msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x01,
                color_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        msg.payload.colors[0] = *color;
        msg
    }

    /// Sets breathing; fades the provided colors in and out sequentially.
    pub fn breathing(colors: &Vec<RGB>) -> SetLedEffect {
        // Fades spectrum in and out; 'breathing'?
        //  payload: vec![0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00],
        //  payload: vec![0x00, 0x00, 0x02, 0x02, 0x01, 0x03, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xff, 0x00],  // breathing 3 colors.

        let mut msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x02,
                ..Default::default()
            },
            ..Default::default()
        };
        // No colors results in random color.
        msg.payload.direction = 0x00; // Doesn't seem to do anything at all.
        msg.payload.speed = 0x00; // Timed it, 3 colors always takes ~ 50s, regardless of value.
        msg.set_colors(&colors);
        msg
    }

    /// Sets a spectrum cycle.
    pub fn spectrum() -> SetLedEffect {
        // Length is actually 6 for this instruction, but firmware doesn't care.
        //  payload: vec![0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00], // Cycles spectrum
        let msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x03,
                ..Default::default()
            },
            ..Default::default()
        };
        // Speed doesn't seem to do anything.
        msg
    }

    /// Lets a wave in hue move over the keyboard.
    /// Default is left to right, reverse is right to left. Delay seems to be delay in milliseconds
    /// between updates, the actual update amount seems to be hardcoded, so if delay is too large
    /// it will just look not smooth.
    pub fn wave(reverse: bool, delay: u8) -> SetLedEffect {
        // payload: vec![0x00, 0x00, 0x04, 0x01, 0xFF],
        //                                    ^ 1 or 2, direction.
        //                                         ^ Speed, lower is faster, probably delay in msec.
        // Doesn't seem to have a field for the 'step', which is what would make sense...
        // changing direction to anything else doesn't seem to work... odd, I would implement step as well.
        let mut msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x04,
                ..Default::default()
            },
            ..Default::default()
        };
        msg.payload.direction = if reverse { 0x01 } else { 0x02 }; // 0x01 or 0x02, 0x00 makes it a nop
        msg.payload.speed = delay; // probably delay in ms?
        msg
    }

    /// Sets reactive, touched keys glow for a duration in the provided or random color.
    /// Does not take more than one color, zero or one color is used.
    pub fn reactive(duration: Duration, colors: &Vec<RGB>) -> SetLedEffect {
        // payload: vec![0x00, 0x00, 0x05, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00],  // Should be reactive, also 0x02, 0x01, 0x00, 0x00 passed <- So cool, spectrum reactive
        // payload: vec![0x00, 0x00, 0x05, 0x02, 0x01, 0x01, 0xAA, 0x44, 0xBB],  // Fixed Color reactive.
        //  //                                           ^ Specifies color or not.

        let mut msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x05,
                ..Default::default()
            },
            ..Default::default()
        };
        // No colors results in random color.
        msg.payload.direction = 0x00; //
        msg.payload.speed = duration as u8;
        msg.set_colors(&colors);
        msg
    }

    /// Makes a ripple / circle propagate outwards from keys that are pressed.
    /// Does not take more than one color, zero or one color is used.
    pub fn ripple(colors: &Vec<RGB>) -> SetLedEffect {
        // payload: vec![0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00],  // waves propagating out of the keys, random color
        //  payload: vec![0x00, 0x00, 0x06, 0x00, 0x00, 0x01, 0xAA, 0x44, 0xBB],  // Fixed Color waves, same pattern as reactive for the arguments.
        let mut msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x06,
                ..Default::default()
            },
            ..Default::default()
        };
        // No colors results in random color.
        msg.set_colors(&colors);
        msg
    }

    /// Lights up keys in random patterns. If no colors provided, colors are random, accepts up to
    /// two colors.
    pub fn starlight(duration: Duration, colors: &Vec<RGB>) -> SetLedEffect {
        // payload: vec![0x00, 0x00, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00, 0xFF],  // keys lighting up randomly, different colors.
        //  payload: vec![0x00, 0x00, 0x07, 0x01, 0x01, 0x01, 0xAA, 0x44, 0xBB],  // Fixed Color randomly lighting keys, same pattern as reactive.
        // payload: vec![0x00, 0x00, 0x07, 0x01, 0x05, 0x02, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00],  // Starlight two colors.
        let mut msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x07,
                ..Default::default()
            },
            ..Default::default()
        };
        // No colors results in random color.
        msg.payload.direction = 0x01; // ??
        msg.payload.speed = duration as u8;
        msg.set_colors(&colors);
        msg
    }

    /// Disables effects and sets a custom state we can provide through SetLedState.
    pub fn custom() -> SetLedEffect {
        // payload: vec![0x00, 0x00, 0x08, 0x05, 0x05, 0x05, 0xAA, 0x44, 0xBB],  // Good question... makes the keyboard green..?
        // payload: vec![0x00, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00],  // Good question... makes the keyboard green..?
        // Only made it green because that's what I set it to with SetLedState, this disables the effects?
        // or sets the effect to take the values from the SetLedState?
        let msg = SetLedEffect {
            payload: wire::SetLedEffect {
                effect: 0x08,
                ..Default::default()
            },
            ..Default::default()
        };
        msg
    }


    #[rustfmt::skip] // Really don't want this to get formatted...
    pub fn dev() -> Box<dyn Command>
    {
        Box::new(ArbitraryCommand {
            // 0x06 is len, but as we've seen here, it seems pretty much ignored.

            register: Cmd{major: 0x0f, minor: 0x02},
            //  cmd: 0x450f8200,


            payload: vec![0x00, 0x00, 0x09, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00],  // Flickering hue panel? Seems to be a combination of Fire and spectrum or something?
            // payload: vec![0x00, 0x00, 0x09, 0x00, 0x01, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00],  // Flickering hue panel? Seems to be a combination of Fire and spectrum or something?


            // payload: vec![0x00, 0x00, 0x0a, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00],  // can't get any colors.
            // payload: vec![0x00, 0x00, 0x0b, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00],  // can't get any colors.
            // payload: vec![0x00, 0x00, 0x0c, 0x01, 0x01, 0x01, 0xFF, 0xAA, 0x00],  // can't get any colors.
            // payload: vec![0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0xFF, 0xAA, 0x00],  // can't get any colors.
            // payload: vec![0x00, 0x00, 0x0e, 0x01, 0x01, 0x01, 0xFF, 0xAA, 0x00],  // can't get any colors.
            // payload: vec![0x00, 0x00, 0x0f, 0x01, 0x01, 0x01, 0xFF, 0xAA, 0x00],  // can't get any colors.
            // payload: vec![0x00, 0x00, 0x10, 0x01, 0x01, 0x01, 0xFF, 0xAA, 0x00],  // can't get any colors.
            // payload: vec![0x00, 0x00, 0x11, 0x01, 0x01, 0x01, 0xFF, 0xAA, 0x00],  // can't get any colors.
            // There's 17 effects only... 



            // This returns... something.
            // register: huntsman_comm::Cmd{major: 0x06, minor: 0x8e},
            // payload: vec![0x00], //

        })
    }
}
impl Command for SetLedEffect {
    fn register(&self) -> Cmd {
        return SetLedEffect::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::SetLedEffect>()];
        self.payload
            .to_le_bytes(&mut v[..])
            .expect("Should succeed");
        v
    }
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
    pub leds: [RGB; 23], // 22 is max seen used, but 23 matches size from length field.
}
impl SetLedState {
    pub const CMD: Cmd = Cmd {
        major: 0x0f,
        minor: 0x03,
    };
}

impl Command for SetLedState {
    fn register(&self) -> Cmd {
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
/// Sets the brightness for the entire keyboard.
pub struct SetLedBrightness {
    /// The brightness to set, should be [0, 1].
    pub value: f32,
    pub first: u8,
}
impl SetLedBrightness {
    pub const CMD: Cmd = Cmd {
        major: 0x0f,
        minor: 0x04,
    };
}

impl Command for SetLedBrightness {
    fn register(&self) -> Cmd {
        return SetLedBrightness::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::SetLedBrightness>()];
        let wire_setbrightness: wire::SetLedBrightness = wire::SetLedBrightness {
            first: self.first, // 0x01 or 0x00, doesn't seem to matter much.
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
    pub const CMD: Cmd = Cmd {
        major: 0x03,
        minor: 0x00,
    };
}

impl Command for SetGameMode {
    fn register(&self) -> Cmd {
        return SetGameMode::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::SetGameMode>()];
        let wire_cmd = wire::SetGameMode {
            game_mode_enabled: self.value as u8,
            ..Default::default()
        };
        wire_cmd.to_le_bytes(&mut v[..]).expect("Should succeed");
        v
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Override a key with a new functionality. Still very much WIP, see tests.
pub struct SetKeyOverride {}
impl SetKeyOverride {
    pub const CMD: Cmd = Cmd {
        major: 0x02,
        minor: 0x0D,
    };
}

impl Command for SetKeyOverride {
    fn register(&self) -> Cmd {
        return SetKeyOverride::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::SetKeyOverride>()];
        let wire_cmd = wire::SetKeyOverride {
            ..Default::default()
        };
        wire_cmd.to_le_bytes(&mut v[..]).expect("Should succeed");
        v
    }
}

#[derive(Default, Clone, Debug)]
/// Sends an arbitrary payload to a register, use with caution, useful for testing.
pub struct ArbitraryCommand {
    pub register: Cmd,
    pub payload: Vec<u8>,
}

impl Command for ArbitraryCommand {
    fn register(&self) -> Cmd {
        return self.register;
    }
    fn payload(&self) -> Vec<u8> {
        return self.payload.clone();
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Get the memory storage statistics.
pub struct GetStorageStatistics {}
impl GetStorageStatistics {
    pub const CMD: Cmd = Cmd {
        major: 0x06,
        minor: 0x8E,
    };
}
impl Command for GetStorageStatistics {
    fn register(&self) -> Cmd {
        return GetStorageStatistics::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; std::mem::size_of::<wire::GetStorageStatistics>()];
        let wire_cmd = wire::GetStorageStatistics {
            ..Default::default()
        };
        wire_cmd.to_le_bytes(&mut v[..]).expect("Should succeed");
        v
    }
}

/// Helper function for the dissector that provides the fields for the provided commands.
pub fn get_command_fields() -> Vec<(Cmd, Box<dyn Fn() -> struct_helper::Field>)> {
    vec![
        (SetGameMode::CMD, Box::new(wire::SetGameMode::fields)),
        (SetKeyOverride::CMD, Box::new(wire::SetKeyOverride::fields)),
        (SetLedEffect::CMD, Box::new(wire::SetLedEffect::fields)),
        (SetLedState::CMD, Box::new(wire::SetLedState::fields)),
        (
            SetLedBrightness::CMD,
            Box::new(wire::SetLedBrightness::fields),
        ),
    ]
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const PAYLOAD_START: usize = 8;
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

    /// Parses a wireshark value, but assumes null bytes for the remainder, does calculate
    /// and compare the checksum against the provided check u8, asserts if this fails.
    fn parse_wireshark_truncated(z: &str, check: u8) -> Vec<u8> {
        let mut v = parse_wireshark_value(z);
        const LENGTH: usize = 90;
        while v.len() != LENGTH {
            v.push(0);
        }
        let mut checksum: u8 = 0;
        for i in 2..LENGTH {
            checksum ^= v[i];
        }
        assert_eq!(check, checksum);
        v[LENGTH - 2] = checksum;
        v
    }
    #[test]
    fn test_helper() {
        let real = parse_wireshark_value("02:1f:00:00:00:0e:06:8e:ff:ff:00:01:8f:f0:00:01:8a:78:00:01:8a:78:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:f8:00");
        let truncated = parse_wireshark_truncated(
            "02:1f:00:00:00:0e:06:8e:ff:ff:00:01:8f:f0:00:01:8a:78:00:01:8a:78",
            0xf8,
        );
        assert_eq!(real, truncated);
        let real = parse_wireshark_value("00:1f:00:00:00:03:0f:04:01:00:7f:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:76:00");
        let truncated = parse_wireshark_truncated("00:1f:00:00:00:03:0f:04:01:00:7f", 0x76);
        assert_eq!(real, truncated);
    }

    #[test]
    fn test_set_led_state() {
        let expected = parse_wireshark_truncated("00:1f:00:00:00:4a:0f:03:00:00:06:00:16:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff", 0xa9);
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
        let expected_50_pct = parse_wireshark_truncated("00:1f:00:00:00:03:0f:04:01:00:7f", 0x76);
        let mut brightness: SetLedBrightness = SetLedBrightness {
            first: 0x01,
            ..Default::default()
        };
        brightness.value = 0.5;
        assert_eq!(brightness.serialize(), expected_50_pct);

        let expected_100_pct = parse_wireshark_truncated("00:1f:00:00:00:03:0f:04:01:00:ff", 0xf6);
        brightness.value = 1.0;
        assert_eq!(brightness.serialize(), expected_100_pct);

        brightness.value = 2.5; // cool, 'as u8' clamps.
        assert_eq!(brightness.serialize(), expected_100_pct);
    }

    #[test]
    fn test_set_game_mode() {
        let enable = parse_wireshark_truncated("00:1f:00:00:00:03:03:00:00:08:01", 0x09);
        let mut game_mode: SetGameMode = Default::default();
        game_mode.value = true;
        assert_eq!(game_mode.serialize(), enable);

        let disable = parse_wireshark_truncated("00:1f:00:00:00:03:03:00:00:08", 0x08);
        game_mode.value = false;
        assert_eq!(game_mode.serialize(), disable);

        // 00:1f:00:00:00:03:03:00:00:08:00:00:00...
        // 00:1f:00:00:00:03:03:00:00:08:01:00:00...
        // 00:1f:00:00:00:03:03:00:00:18:00:00:00...
        // What does this 18 mean!? Seems to toggle the volume led!? O_o
        // It did disable my override on right control, which is sketchy...
    }

    #[test]
    fn test_effects() {
        // Failing the length here, lets not worry about that as it doesn't seem checked in the firmware
        // and it makes our handling much easier if we can use the same payload struct for one entity.

        // the second byte in the payload seems to be some kind of response, it holds 5 in most cases.

        // Seen messages;
        // let spectrum_expect = parse_wireshark_value("00:1f:00:00:00:06:0f:02:01:00:03:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:09:00");
        // let mut spectrum: SetLedEffect = SetLedEffect::spectrum();
        // spectrum.payload.first = 0x01;
        // assert_eq!(spectrum.serialize(), spectrum_expect);

        // We've seen this; but it doesn't seem to do anything atm?
        // 00:1f:00:00:00:06:0f:02:00:00:08:01:01:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:03:00
    }

    #[test]
    fn overrides_for_keys() {
        // Readable version of the usb HID id's
        // https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/translate.pdf

        // Readable key id lookup
        // https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/scancode.doc

        // Right shift to k, top two entries seem to be THE thing.
        // TSetMapping MOD: 0x00000000	MAP: MAPPING_SINGLEKEY Key: MC=0x25, EX=0x00, Mods=0x00000000
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:39:00:02:02:00:0e:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:39:00:02:02:00:0e:00...
        //                                                                        ^^  hid id for 'k'

        // Right shift to default:
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:39:00:02:02:00:e5:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:39:00:02:02:00:e5:00...
        //                                                                        ^^  hid id for Right shift.

        // Unbind hypershift minus (on numpad)
        //                              00:1f:00:00:00:0a:02:0d:01:69:01:02:02:00:56:00...

        // Right control to mouse rightclick.
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:40:00:01:01:02:00:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:40:00:01:01:02:00:00...
        // Hypershift 3 as rightclick:
        //                              00:1f:00:00:00:0a:02:0d:01:04:01:01:01:02:00:00...
        //                                                            ^^   is hypershift / modifier?

        // Right control to mouse leftclick.
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:01:40:00:01:01:01:00:00...
        // 0x0000000a	0x0000020d	00:1f:00:00:00:0a:02:0d:02:40:00:01:01:01:00:00...
        //                                profile or activation?^^
        //                                                    key  ^^                      :check:
        //                                                                        ^^  hid id key if keyboard
        //                                                                     ^^  Mouse buttion / action?
        //                                              kbd or mouse?    ^^  01 is mouse, 02 is kbd?
        //                                                                  ^^ ^^    ???

        // This looks more like a disable...
        // Bind hypershift plus to brightness up:
        //                              00:1f:00:00:00:0a:02:0d:01:6a:01:00:00:00:00:00....
        // Bind hypershift minus to brightness down:
        //                              00:1f:00:00:00:0a:02:0d:01:69:01:00:00:00:00:00...

        // Switch profile with right alt;
        //                              00:1f:00:00:00:0a:02:0d:01:40:00:01:01:01:00:00:00:00
        //                              00:1f:00:00:00:0a:02:0d:01:3e:00:00:00:00:00:00:00:00
        //                              00:1f:00:00:00:0a:02:0d:01:02:01:00:00:00:00:00:00:00
        //                              00:1f:00:00:00:0a:02:0d:01:03:01:01:01:01:00:00:00:00

        // Key locations match;
        // Microsoft Keyboard Scan Code Specification (Appendix C, "USB Keyboard/Keypad Page (0x07)"),
        // https://en.wikipedia.org/wiki/Scancode#USB
        // Most certainly, Key location microsofts keyboard scan code lists matches.
        // key 0x39 is 57, which is R SHIFT, p18
        // key 0x40 is 64, which is R CTRL, p18
    }

    #[test]
    fn test_profile_deletion() {
        // Also issued before we 'upload' a profile.
        let remove_profile_1_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:02", 0x05); // red
        let remove_profile_2_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:03", 0x04); // green
        let remove_profile_3_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:04", 0x03); // blue
        let remove_profile_4_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:05", 0x02);
        // cyan
        // This profile deletion supports well the concept of the first payload byte being something of a profile indicator.
    }

    #[test]
    fn test_get_storage() {
        let request = parse_wireshark_truncated("00:1f:00:00:00:0e:06:8e", 0x86);
        let mut request_cmd: GetStorageStatistics = GetStorageStatistics {
            ..Default::default()
        };
        assert_eq!(request_cmd.serialize(), request);
        let respons = parse_wireshark_truncated(
            "02:1f:00:00:00:0e:06:8e:ff:ff:00:01:8f:f0:00:01:8a:78:00:01:8a:78",
            0xf8,
        );
        // max[102384], free[201968], percent[197.27]
        //                                                                    |102384     |100984     |100984     |
        // 100984 + 100984 = 201968
        assert_eq!(0x0e, std::mem::size_of::<wire::GetStorageStatistics>());
        // println!("{:?}", &respons[PAYLOAD_START..]);
        // TODO: Whelp, the endianness here is wrong :(
        let decoded = wire::GetStorageStatistics::from_be_bytes(&respons[PAYLOAD_START..])
            .expect("Should pass");
        let something = decoded.something;
        let total = decoded.total;
        assert_eq!(something, 0xFFFF);
        assert_eq!(total, 102384);
        // Either the log is lying, or the data is incorrrect.
    }
}

/*
// last command on exit, log reports setting to firmware mode.
let cmd = huntsman_comm::ArbitraryCommand {
            // register: huntsman_comm::Cmd{major: 0x00, minor: 0x04},
            // payload: vec![0x00, 0x00], //
}



*/
