//! This crate provides an object to interface with the keyboard, it also provides a command line
//! utility that makes use of this object.

mod hid_hal;

pub mod commands;

pub use commands::RGB;

pub mod configuration;
mod hut_util;
mod keymap_util;

/// Object to interface with the Huntsman Elite keyboard.
pub struct Huntsman {
    hal: Box<dyn hid_hal::HidHal>,
    print_comm: bool,
    print_retrieve: bool,
}

type Error = Box<dyn std::error::Error>;

impl Huntsman {
    /// Construct a new Huntsman instance, this tries to connect to the usb device and errors if it can't be found.
    pub fn new() -> Result<Huntsman, Error> {
        match hid_hal::HidApiHal::new() {
            Ok(mut hal) => match hal.connect(0x1532, 0x226, 2) {
                Ok(()) => Ok(Huntsman {
                    hal: hal,
                    print_comm: false,
                    print_retrieve: false,
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn dry_new() -> Result<Huntsman, Error> {
        match hid_hal::DryHidHal::new() {
            Ok(mut hal) => match hal.connect(0x1532, 0x226, 2) {
                Ok(()) => Ok(Huntsman {
                    hal: hal,
                    print_comm: false,
                    print_retrieve: false,
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// Toggle printing the outgoing communication on or off.
    pub fn set_print_comm(&mut self, state: bool) {
        self.print_comm = state;
    }

    /// Toggle whether to retrieve the feature report after sending a command
    pub fn set_print_retrieve(&mut self, state: bool) {
        self.print_retrieve = state;
    }

    /// Function to send a command to the control endpoint.
    fn set_command(&mut self, command: &dyn commands::Command) -> Result<Option<Vec<u8>>, Error> {
        let v = command.serialize();
        if self.print_comm {
            println!("{:?} -> {:?}", command, v);
            println!(
                "{}",
                (v.clone())
                    .iter()
                    .map(|x| format!("{:0>2x}", x))
                    .collect::<Vec<String>>()
                    .join(":")
            );
        }
        let r = self.hal.control(&v.as_slice());
        if r.is_ok() {
            let result = self.hal.get_report();
            if self.print_retrieve {
                println!("<- {:?}", result);
            }

            if result.is_ok() {
                let res = result.unwrap();
                if self.print_retrieve {
                    println!(
                        "{}",
                        (res.clone())
                            .iter()
                            .map(|x| format!("{:0>2x}", x))
                            .collect::<Vec<String>>()
                            .join(":")
                    );
                }
                return Ok(Some(res));
            } else {
                return Err(result.expect_err("Error"));
            }
        }
        return Err(r.expect_err("Something went wrong"));
    }

    /// Function to send a Boxed command to the control endpoint.
    fn set_command_box(
        &mut self,
        boxed_command: &Box<dyn commands::Command>,
    ) -> Result<Option<Vec<u8>>, Error> {
        self.set_command(boxed_command.as_ref())
    }

    /// Function that sends a single SetLedState instruction, index is the row, start is the column, count is the number
    /// of leds to set from the index.
    pub fn set_color_single(
        &mut self,
        color: &RGB,
        count: u8,
        index: u8,
        start: u8,
    ) -> Result<(), Error> {
        let mut leds: commands::SetLedState = Default::default();
        leds.count = start + count;
        leds.id = index;
        for l in start as usize..leds.count as usize {
            leds.leds[l].r = color.r;
            leds.leds[l].g = color.g;
            leds.leds[l].b = color.b;
        }
        return self.set_command(&leds).and_then(|_v| Ok(()));
    }

    pub fn set_color(&mut self, row: u8, color: &[RGB]) -> Result<(), Error> {
        let mut leds: commands::SetLedState = Default::default();
        leds.count = color.len() as u8;
        leds.id = row;
        for l in 0..leds.count as usize {
            leds.leds[l].r = color[l].r;
            leds.leds[l].g = color[l].g;
            leds.leds[l].b = color[l].b;
        }
        return self.set_command(&leds).and_then(|_v| Ok(()));
    }

    /// Set the brightness of the entire keyboard, specify as [0, 1.0].
    pub fn set_brightness(&mut self, profile: u8, value: f32) -> Result<(), Error> {
        let mut cmd: commands::SetLedBrightness = Default::default();
        cmd.profile = profile;
        cmd.value = value;
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Toggle game mode on or off.
    pub fn set_game_mode(&mut self, value: bool) -> Result<(), Error> {
        let mut cmd: commands::SetGameMode = Default::default();
        cmd.value = value;
        let r = self.set_command(&cmd).and_then(|_v| Ok(()));
        return r;
    }

    /// Dev function exposed to the commandline utility.
    pub fn dev_run(&mut self) -> Result<(), Error> {
        self.set_print_comm(true);
        self.set_print_retrieve(true);
        let cmd = commands::dev_run_cmd();
        return self.set_command_box(&cmd).and_then(|_v| Ok(()));
    }

    /// Retrieve the serial number
    pub fn get_serial_number(&mut self) -> Result<(), Error> {
        let cmd: commands::GetSerialNumber = Default::default();
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let response = response
            .downcast_ref::<commands::GetSerialNumber>()
            .unwrap();
        println!("{}", response.serial.as_ref().unwrap());
        Ok(())
    }

    /// Set a key mapping
    pub fn set_mapping(
        &mut self,
        profile: u8,
        key: commands::mappings::Key,
        mapping: commands::mappings::KeyMapping,
    ) -> Result<(), Error> {
        let cmd: commands::SetKeyMap = commands::SetKeyMap(commands::mappings::KeyMap {
            profile,
            key,
            mapping,
        });
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let _response = response.downcast_ref::<commands::SetKeyMap>().unwrap();
        Ok(())
    }

    /// Dump keymappings.
    pub fn get_mapping(
        &mut self,
        profile: u8,
        key: commands::mappings::Key,
    ) -> Result<commands::mappings::KeyMap, Error> {
        let cmd: commands::GetKeyMap = commands::GetKeyMap(commands::mappings::KeyMap {
            profile: profile,
            key: key,
            ..Default::default()
        });

        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let response = response.downcast_ref::<commands::GetKeyMap>().unwrap();
        Ok(response.0)
    }

    /// Disables led effects, turning off each led. See also [`commands::SetLedEffect::off()`]
    pub fn effect_off(&mut self) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::off();
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Sets a fixed color on all leds. See also [`commands::SetLedEffect::fixed()`]
    pub fn effect_fixed(&mut self, color: &RGB) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::fixed(&color);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Applies the breathing effect, fading colors in and out sequentially. See also [`commands::SetLedEffect::breathing()`]
    pub fn effect_breathing(&mut self, colors: &Vec<RGB>) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::breathing(&colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Spectrum cycle, entire keyboard cycles the hue. See also [`commands::SetLedEffect::spectrum()`]
    pub fn effect_spectrum(&mut self) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::spectrum();
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// A hue wave moved over the keyboard. See also [`commands::SetLedEffect::wave()`]
    pub fn effect_wave(&mut self, direction: bool, delay: u8) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::wave(direction, delay);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Lights up keys after they are pressed. See also [`commands::SetLedEffect::reactive()`]
    /// Only takes a single color.
    pub fn effect_reactive(
        &mut self,
        duration: commands::Duration,
        colors: &Vec<RGB>,
    ) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::reactive(duration, colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Waves propagate outwards from pressed keys. See also [`commands::SetLedEffect::ripple()`]
    /// Only takes a single color
    pub fn effect_ripple(&mut self, colors: &Vec<RGB>) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::ripple(&colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Keys light up randomly. See also [`commands::SetLedEffect::starlight()`]
    /// Only takes up to two colors.
    pub fn effect_starlight(
        &mut self,
        duration: commands::Duration,
        colors: &Vec<RGB>,
    ) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::starlight(duration, colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Display the custom frame. See also [`commands::SetLedEffect::custom()`]
    pub fn effect_custom(&mut self) -> Result<(), Error> {
        let cmd = commands::SetLedEffect::custom();
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Method to retrieve the macros currently on the device.
    pub fn macro_list(&mut self) -> Result<Vec<commands::MacroId>, Error> {
        let cmd: commands::GetActiveMacros = Default::default();
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let response = response
            .downcast_ref::<commands::GetActiveMacros>()
            .unwrap();
        Ok(response.0.to_vec())
    }
    /// Method to retrieve the macros currently on the device.
    pub fn macro_count(&mut self) -> Result<u16, Error> {
        let cmd: commands::GetActiveMacroCount = Default::default();
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let response = response
            .downcast_ref::<commands::GetActiveMacroCount>()
            .unwrap();
        Ok(response.0.count)
    }

    /// Delete macro by its id.
    pub fn macro_delete(&mut self, macro_id: u16) -> Result<(), Error> {
        let mut cmd: commands::MacroDelete = Default::default();
        cmd.0.macro_id = macro_id;
        let _result = self.set_command(&cmd)?;
        Ok(())
    }

    /// Create a macro of a certain size, not setting the payload.
    pub fn macro_create(&mut self, macro_id: u16, size: usize) -> Result<(), Error> {
        let mut cmd: commands::MacroCreate = Default::default();
        cmd.0.macro_id = macro_id;
        cmd.0.event_bytes = size as u32;
        let _result = self.set_command(&cmd)?;
        Ok(())
    }

    /// Create macro of correct size for acctions, allocate & assign actions.
    pub fn macro_create_actions(
        &mut self,
        macro_id: u16,
        actions: &Vec<commands::macros::MacroAction>,
    ) -> Result<(), Error> {
        // First, delete the macro if it already exists.
        let _ = self.macro_delete(macro_id); // ignore the result of this, it can fail if the id doesn't exist.
        let total_bytes = commands::macros::macro_events_to_size(&actions);

        // Create the macro
        self.macro_create(macro_id, total_bytes)?;

        // Then, set the payloads
        let mut payloads = commands::macros::macro_events_to_payloads(macro_id, &actions);
        for payload in payloads.drain(..) {
            let mut cmd: commands::MacroActionsPayload = Default::default();
            cmd.0 = payload;
            let _ = self.set_command(&cmd)?; // set the payload chunk
        }
        // should be it... :O
        Ok(())
    }

    /// Method to retrieve the profiles currently on the device.
    pub fn profile_list(&mut self) -> Result<Vec<commands::ProfileId>, Error> {
        let cmd: commands::GetActiveProfiles = Default::default();
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let response = response
            .downcast_ref::<commands::GetActiveProfiles>()
            .unwrap();
        Ok(response.0.to_vec())
    }
    /// Method to retrieve the macros currently on the device.
    pub fn profile_count(&mut self) -> Result<commands::ProfileId, Error> {
        let cmd: commands::GetActiveProfileCount = Default::default();
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let response = response
            .downcast_ref::<commands::GetActiveProfileCount>()
            .unwrap();
        Ok(response.0.count)
    }

    /// Delete profile by its id, one cannot delete the currently active profile.
    pub fn profile_delete(&mut self, profile_id: commands::ProfileId) -> Result<(), Error> {
        if profile_id < 2 || profile_id > 5 {
            // Not too sure what happens if we throw out 1...
            panic!("Profile ids must be 2, 3, 4 or 5.");
        }
        let mut cmd: commands::ProfileDelete = Default::default();
        cmd.0.profile_id = profile_id;
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let _response = response.downcast_ref::<commands::ProfileDelete>().unwrap();
        Ok(())
    }

    /// Create a profile by the provided id..
    pub fn profile_create(&mut self, profile_id: commands::ProfileId) -> Result<(), Error> {
        if profile_id < 2 || profile_id > 5 {
            panic!("Profile ids must be 2, 3, 4 or 5.");
        }
        let mut cmd: commands::ProfileCreate = Default::default();
        cmd.0.profile_id = profile_id;
        let _result = self.set_command(&cmd)?;
        Ok(())
    }

    /// Retrieve the currently active profile.
    pub fn profile_get_current(&mut self) -> Result<commands::ProfileId, Error> {
        let cmd: commands::GetProfileCurrent = Default::default();
        let result = self.set_command(&cmd)?;
        let response = commands::Command::response(&cmd, &result.unwrap())?;
        let response = response
            .downcast_ref::<commands::GetProfileCurrent>()
            .unwrap();
        Ok(response.0.profile_id)
    }
    /// Set the active profile to the id provided.
    pub fn profile_set_current(&mut self, profile_id: commands::ProfileId) -> Result<(), Error> {
        let mut cmd: commands::SetProfileCurrent = Default::default();
        cmd.0.profile_id = profile_id;
        let _result = self.set_command(&cmd)?;
        Ok(())
    }
}
