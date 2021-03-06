//! This module holds the commands that can be sent to the device.

use struct_helper::{Inspectable, ToBytes};

pub mod wire;
use struct_helper::FromBytes;
pub use wire::Cmd;
pub use wire::RGB;

pub mod macros;
pub mod mappings;
pub mod profiles;

pub use std::any::Any;

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

On profiles:
0x00 is current? Seems volatile?
0x01 seems to be the boot state?
0x02 red
0x03 green
0x04 blue
0x05 cyan

*/

/// Represents a command that can be sent over USB.
pub trait Command: std::fmt::Debug {
    /// Should provider the raw bytes that are to be sent to the device over usb.
    fn serialize(&self) -> Vec<u8> {
        // Now follows the command.
        let cmd = self.register();
        let payload = self.payload();

        let mut wire: wire::Command = Default::default();
        wire.cmd = cmd;
        wire.len = payload.len() as u8;

        // copy the payload.
        for i in 0..payload.len() {
            wire.payload[i] = payload[i];
        }
        wire.update_checksum(); // update the checksum based on the currently populated values.
        wire.to_le_bytes().expect("Should succeed")
    }

    /// Provides the two register addressses that are to be sent in the header.
    fn register(&self) -> Cmd;

    /// Provides the payload definition that comes after the header.
    fn payload(&self) -> Vec<u8>;

    /// Processes the response and returns an any holding it.
    fn response(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let mut wire: wire::Command = wire::Command::from_le_bytes(data)?;
        let original_checksum = wire.checksum;
        wire.update_checksum();
        if original_checksum != wire.checksum {
            return Err(format!(
                "Checksum did not pass, got {:?} expected {:?}",
                original_checksum, wire.checksum
            ));
        }
        if data[0] != 0x02 {
            return Err(format!("Return code was not 0x02",));
        }
        self.response_payload(&wire.payload[..])
    }

    /// If commands only care about the payload (most do), this method gets called from
    /// the default implementation of [`self.response()`] and passes just the payload slice.
    fn response_payload(&self, _data: &[u8]) -> Result<Box<dyn Any>, String> {
        unimplemented!("Reading response payload is not implemented for this command.");
    }
}

#[derive(Default, Clone, Debug)]
/// Retrieves the serial number
pub struct GetSerialNumber {
    pub serial: Option<String>,
}
impl GetSerialNumber {
    pub const CMD: Cmd = Cmd {
        major: 0x00,
        minor: 0x82,
    };
}
impl Command for GetSerialNumber {
    fn register(&self) -> Cmd {
        return GetSerialNumber::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        vec![0; 0x16]
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let end = data.iter().position(|&x| x == 0);
        if end.is_none() {
            return Err("Couldn't find 0 termination.".to_string());
        }
        let end = end.unwrap();
        match std::str::from_utf8(&data[..end]) {
            Ok(v) => {
                return Ok(Box::new(GetSerialNumber {
                    serial: Some(v.to_string()),
                }));
            }
            Err(v) => {
                return Err(format!("{}", v));
            }
        }
    }
}

/*
0x0004 firmware mode.
// last command on exit, log reports setting to firmware mode.
let cmd = huntsman_comm::ArbitraryCommand {
            // register: huntsman_comm::Cmd{major: 0x00, minor: 0x04},
            // payload: vec![0x00, 0x00], //
// att start of synapse open we see 0x0004 0x03 0x00, corresponding to the log stating DRIVER mode.
}
*/

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

    /// Sets the profile to apply this led effect on.
    pub fn set_profile(&mut self, profile: u8) {
        self.payload.profile = profile;
    }

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
        // Notice we don't care about the length here, it doesn't seem to be checked by the firmware
        // and it makes our life much easier if we can use the same payload struct for all effects.

        // The second byte in the payload seems to be some kind of response, it is 5 in most cases.
        self.payload.to_le_bytes().expect("Should succeed")
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
    pub leds: [RGB; 23],
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
        let wire_ledstate: wire::SetLedState = wire::SetLedState {
            first: 0,
            id: self.id,
            count: self.count,
            leds: self.leds,
            ..Default::default()
        };
        wire_ledstate.to_le_bytes().expect("Should succeed")
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Sets the brightness for the entire keyboard.
pub struct SetLedBrightness {
    /// The brightness to set, should be [0, 1].
    pub value: f32,
    pub profile: u8,
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
        let wire_setbrightness: wire::SetLedBrightness = wire::SetLedBrightness {
            profile: self.profile,
            value: (self.value * 255.0) as u8,
            ..Default::default()
        };
        wire_setbrightness.to_le_bytes().expect("Should succeed")
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
        let wire_cmd = wire::SetGameMode {
            game_mode_enabled: self.value as u8,
            ..Default::default()
        };
        wire_cmd.to_le_bytes().expect("Should succeed")
    }
}

pub use profiles::ProfileId;
#[derive(Default, Clone, Debug)]
/// Set a key mapping.
pub struct SetKeyMap(pub mappings::KeyMap);
impl SetKeyMap {
    pub const CMD: Cmd = Cmd {
        major: 0x02,
        minor: 0x0D,
    };
}
impl Command for SetKeyMap {
    fn register(&self) -> Cmd {
        return SetKeyMap::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let serialized = self.0.to_be_bytes().expect("Success");
        serialized
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let res = mappings::KeyMap::from_be_bytes(&data).expect("Should pass");
        let cmd = SetKeyMap(res);
        Ok(Box::new(cmd))
    }
}

#[derive(Default, Clone, Debug)]
/// Retrieve a keymapping
pub struct GetKeyMap(pub mappings::KeyMap);
impl GetKeyMap {
    pub const CMD: Cmd = Cmd {
        major: 0x02,
        minor: 0x8D,
    };
}
impl Command for GetKeyMap {
    fn register(&self) -> Cmd {
        return GetKeyMap::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        let serialized = self.0.to_be_bytes().expect("Success");
        serialized
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let res = mappings::KeyMap::from_be_bytes(&data).expect("Should pass");
        let cmd = GetKeyMap(res);
        Ok(Box::new(cmd))
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Retrieve the number of profiles's on the device.
pub struct GetActiveProfileCount(pub profiles::ProfileCount);
impl GetActiveProfileCount {
    pub const CMD: Cmd = Cmd {
        major: 0x05,
        minor: 0x80,
    };
}
impl Command for GetActiveProfileCount {
    fn register(&self) -> Cmd {
        return GetActiveProfileCount::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let macro_count = profiles::ProfileCount::from_be_bytes(&data).expect("Should pass");
        let cmd = GetActiveProfileCount(macro_count);
        Ok(Box::new(cmd))
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Get the list of active profiles
pub struct GetActiveProfiles(pub profiles::ProfileList);
impl GetActiveProfiles {
    pub const CMD: Cmd = Cmd {
        major: 0x05,
        minor: 0x81,
    };
}
impl Command for GetActiveProfiles {
    fn register(&self) -> Cmd {
        return GetActiveProfiles::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }

    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let macro_list = profiles::ProfileList::from_be_bytes(&data).expect("Should pass");
        let cmd = GetActiveProfiles(macro_list);
        Ok(Box::new(cmd))
    }
}
// Allocate profile 0x0502, profile_id

#[derive(Default, Copy, Clone, Debug)]
/// Create a profile
pub struct ProfileCreate(pub profiles::ProfileCreate);
impl ProfileCreate {
    pub const CMD: Cmd = Cmd {
        major: 0x05,
        minor: 0x02,
    };
}
impl Command for ProfileCreate {
    fn register(&self) -> Cmd {
        return ProfileCreate::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
}

// Delete profile 0x0503, profile_id,
#[derive(Default, Copy, Clone, Debug)]
/// Create a profile
pub struct ProfileDelete(pub profiles::ProfileDelete);
impl ProfileDelete {
    pub const CMD: Cmd = Cmd {
        major: 0x05,
        minor: 0x03,
    };
}
impl Command for ProfileDelete {
    fn register(&self) -> Cmd {
        return ProfileDelete::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        // no real response, but we can't delete the profile that's active.
        let z = profiles::ProfileDelete::from_be_bytes(&data).expect("Should pass");
        let cmd = ProfileDelete(z);
        Ok(Box::new(cmd))
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Get the currently active profile.
pub struct GetProfileCurrent(pub profiles::ProfileCurrent);
impl GetProfileCurrent {
    pub const CMD: Cmd = Cmd {
        major: 0x05,
        minor: 0x84,
    };
}
impl Command for GetProfileCurrent {
    fn register(&self) -> Cmd {
        return GetProfileCurrent::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let res = profiles::ProfileCurrent::from_be_bytes(&data).expect("Should pass");
        let cmd = GetProfileCurrent(res);
        Ok(Box::new(cmd))
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Get the currently active profile.
pub struct SetProfileCurrent(pub profiles::ProfileCurrent);
impl SetProfileCurrent {
    pub const CMD: Cmd = Cmd {
        major: 0x05,
        minor: 0x04,
    };
}
impl Command for SetProfileCurrent {
    fn register(&self) -> Cmd {
        return SetProfileCurrent::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
}

// Set/Read profile metadata: 0x0508,.... read metadata 0x0588

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
        let wire_cmd = wire::GetStorageStatistics {
            ..Default::default()
        };
        wire_cmd.to_le_bytes().expect("Should succeed")
    }
}

pub use macros::MacroId;
#[derive(Default, Copy, Clone, Debug)]
/// Get a list of macro's on the device. (Speculated, unconfirmed)
pub struct GetActiveMacros(pub macros::MacroList);
impl GetActiveMacros {
    pub const CMD: Cmd = Cmd {
        major: 0x06,
        minor: 0x81,
    };
}
impl Command for GetActiveMacros {
    fn register(&self) -> Cmd {
        return GetActiveMacros::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let macro_list = macros::MacroList::from_be_bytes(&data).expect("Should pass");
        let cmd = GetActiveMacros(macro_list);
        Ok(Box::new(cmd))
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Retrieve the number of macro's on the device.
pub struct GetActiveMacroCount(pub macros::MacroCount);
impl GetActiveMacroCount {
    pub const CMD: Cmd = Cmd {
        major: 0x06,
        minor: 0x80,
    };
}
impl Command for GetActiveMacroCount {
    fn register(&self) -> Cmd {
        return GetActiveMacroCount::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
    fn response_payload(&self, data: &[u8]) -> Result<Box<dyn Any>, String> {
        let macro_count = macros::MacroCount::from_be_bytes(&data).expect("Should pass");
        let cmd = GetActiveMacroCount(macro_count);
        Ok(Box::new(cmd))
    }
}

/// The command to create a macro.
#[derive(Debug, Default)]
pub struct MacroCreate(pub macros::MacroCreate);
impl MacroCreate {
    pub const CMD: Cmd = Cmd {
        major: 0x06,
        minor: 0x08,
    };
}
impl Command for MacroCreate {
    fn register(&self) -> Cmd {
        return MacroCreate::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
}

/// The command to create a macro.
#[derive(Debug, Default)]
pub struct MacroDelete(pub macros::MacroDelete);
impl MacroDelete {
    pub const CMD: Cmd = Cmd {
        major: 0x06,
        minor: 0x03,
    };
}
impl Command for MacroDelete {
    fn register(&self) -> Cmd {
        return MacroDelete::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
}

/// Holds the macro payload.
#[derive(Debug, Default)]
pub struct MacroActionsPayload(pub macros::MacroActionsPayload);
impl MacroActionsPayload {
    pub const CMD: Cmd = Cmd {
        major: 0x06,
        minor: 0x09,
    };
}
impl Command for MacroActionsPayload {
    fn register(&self) -> Cmd {
        return MacroActionsPayload::CMD;
    }
    fn payload(&self) -> Vec<u8> {
        self.0.to_be_bytes().expect("cannot fail")
    }
}

#[derive(Default, Copy, Clone, Debug)]
/// Set the macro metadata
pub struct MacroMetadata {}
impl MacroMetadata {
    pub const CMD: Cmd = Cmd {
        major: 0x06,
        minor: 0x0c,
    };
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

pub fn _dev_run_cmd_profiles() -> Box<dyn Command> {
    Box::new(GetActiveProfiles {
        ..Default::default()
    })
}
pub fn dev_run_cmd() -> Box<dyn Command> {
    // set right control back to right control.
    Box::new(ArbitraryCommand {
        register: Cmd {
            major: 0x05,
            minor: 0x84,
        },
        payload: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    })
}

fn make_read_command(cmd: Cmd) -> Cmd {
    Cmd {
        major: cmd.major,
        minor: cmd.minor | 0x80,
    }
}

/// Helper function for the dissector that provides the fields for the provided commands.
pub fn get_command_fields() -> Vec<(Cmd, Box<dyn Fn() -> Box<dyn struct_helper::Inspectable>>)> {
    vec![
        (SetGameMode::CMD, Box::new(wire::SetGameMode::inspect)),
        (SetKeyMap::CMD, Box::new(wire::SetKeyOverride::inspect)),
        (
            make_read_command(SetKeyMap::CMD),
            Box::new(wire::SetKeyOverride::inspect),
        ),
        (SetLedEffect::CMD, Box::new(wire::SetLedEffect::inspect)),
        (SetLedState::CMD, Box::new(wire::SetLedState::inspect)),
        (
            SetLedBrightness::CMD,
            Box::new(wire::SetLedBrightness::inspect),
        ),
        (
            GetStorageStatistics::CMD,
            Box::new(wire::GetStorageStatistics::inspect),
        ),
        (MacroMetadata::CMD, Box::new(macros::MacroMetadata::inspect)),
        (
            MacroActionsPayload::CMD,
            Box::new(macros::MacroActionsPayload::inspect),
        ),
        (MacroCreate::CMD, Box::new(macros::MacroCreate::inspect)),
        (MacroDelete::CMD, Box::new(macros::MacroDelete::inspect)),
    ]
}

mod helpers;
pub use helpers::{
    parse_wireshark_truncated, parse_wireshark_value, to_wireshark_value, WIRESHARK_PAYLOAD_START,
};

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use helpers::{parse_wireshark_truncated, PAYLOAD_START};

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
            profile: 0x01,
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
    fn test_profile_deletion() {
        // Also issued before we 'upload' a profile.
        let _remove_profile_1_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:02", 0x05); // red
        let _remove_profile_2_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:03", 0x04); // green
        let _remove_profile_3_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:04", 0x03); // blue
        let _remove_profile_4_event = parse_wireshark_truncated("00:1f:00:00:00:01:05:03:05", 0x02);
        // cyan
        // This profile deletion supports well the concept of the first payload byte being something of a profile indicator.

        // 0x0502 adds a profile
        // 0x0508 adds the profile metadata / guid stuffs
        // 0x0588 u8 u16:  u8 profile id, u16 page.
        //
        let _potential_list_profiles = parse_wireshark_truncated("00:1f:00:00:00:41:05:81", 0xc5);

        // Storage metric retrieval before write goes through 0x068e
    }

    #[test]
    fn test_get_storage() {
        let request = parse_wireshark_truncated("00:1f:00:00:00:0e:06:8e", 0x86);
        let request_cmd: GetStorageStatistics = GetStorageStatistics {
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
        // let respons2 = parse_wireshark_truncated("02:1f:00:00:00:0e:06:8e:ff:ff:00:01:8f:f0:00:01:8a:78:00:07:b2:08:");
        // CRSy3_OnboardMem2::GetData: storage: max[102368], free[9075328], percent[8865.40]
        // Aight.... lets assume logging and the current parsing is broken.

        // 0x06, 0x03; delete macro!
        let _delete_macro = parse_wireshark_truncated("00:1f:00:00:00:02:06:03:3b:02:00", 0x3e);
        // returns status 02 if exists, 03 if it doesnt

        // 0x06, 0x08; add macro (by id? Or memory address??)
    }

    #[test]
    fn test_set_key_full() {
        let right_ctrl_right_click =
            parse_wireshark_truncated("00:1f:00:00:00:0a:02:0d:01:40:00:01:01:02:00", 0x46);
        let request_cmd: SetKeyMap = SetKeyMap(mappings::KeyMap {
            profile: 0x01,
            key: mappings::Key {
                id: 0x40,
                hypershift: false,
            },
            mapping: mappings::KeyMapping::Mouse(mappings::MouseButton::Right),
        });
        assert_eq!(request_cmd.serialize(), right_ctrl_right_click);

        let right_ctrl_right_click_resp =
            parse_wireshark_truncated("02:1f:00:00:00:0a:02:0d:01:40:00:01:01:02:00", 0x46);
        let response =
            Command::response(&request_cmd, &right_ctrl_right_click_resp).expect("success");
        let response = response.downcast_ref::<SetKeyMap>().unwrap();
        assert_eq!((*response).0, request_cmd.0);
    }
}
