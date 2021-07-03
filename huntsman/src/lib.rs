//! This crate provides an object to interface with the keyboard, it also provides a command line
//! utility that makes use of this object.

mod hid_hal;
pub use huntsman_comm::RGB;

/// Object to interface with the Huntsman Elite keyboard.
pub struct Huntsman {
    hal: hid_hal::HidApiHal,
    print_comm: bool,
    print_retrieve: bool,
}

impl Huntsman {
    /// Construct a new Huntsman instance, this tries to connect to the usb device and errors if it can't be found.
    pub fn new() -> Result<Huntsman, String> {
        match hid_hal::HidApiHal::new() {
            Ok(mut hal) => match &mut hal.connect(0x1532, 0x226, 2) {
                Ok(()) => Ok(Huntsman {
                    hal: hal,
                    print_comm: false,
                    print_retrieve: false,
                }),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
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
    fn set_command(
        &mut self,
        command: &dyn huntsman_comm::Command,
    ) -> Result<Option<Vec<u8>>, String> {
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
                if result.is_ok() {
                    println!(
                        "{}",
                        (result.clone().unwrap())
                            .iter()
                            .map(|x| format!("{:0>2x}", x))
                            .collect::<Vec<String>>()
                            .join(":")
                    );
                }
            }

            if result.is_ok() {
                return Ok(Some(result.unwrap()));
            } else {
                return Err(result.expect_err("Error"));
            }
        }
        return Err(r.expect_err("Something went wrong"));
    }

    /// Function to send a Boxed command to the control endpoint.
    fn set_command_box(
        &mut self,
        boxed_command: &Box<dyn huntsman_comm::Command>,
    ) -> Result<Option<Vec<u8>>, String> {
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
    ) -> Result<(), String> {
        let mut leds: huntsman_comm::SetLedState = Default::default();
        leds.count = start + count;
        leds.id = index;
        for l in start as usize..leds.count as usize {
            leds.leds[l].r = color.r;
            leds.leds[l].g = color.g;
            leds.leds[l].b = color.b;
        }
        return self.set_command(&leds).and_then(|_v| Ok(()));
    }

    pub fn set_color(&mut self, row: u8, color: &[RGB]) -> Result<(), String> {
        let mut leds: huntsman_comm::SetLedState = Default::default();
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
    pub fn set_brightness(&mut self, profile: u8, value: f32) -> Result<(), String> {
        let mut cmd: huntsman_comm::SetLedBrightness = Default::default();
        cmd.profile = profile;
        cmd.value = value;
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Toggle game mode on or off.
    pub fn set_game_mode(&mut self, value: bool) -> Result<(), String> {
        let mut cmd: huntsman_comm::SetGameMode = Default::default();
        cmd.value = value;
        let r = self.set_command(&cmd).and_then(|_v| Ok(()));
        return r;
    }

    /// Dev function exposed to the commandline utility.
    pub fn dev_run(&mut self) -> Result<(), String> {
        self.set_print_comm(true);
        self.set_print_retrieve(true);
        let cmd = huntsman_comm::dev_run_cmd();
        return self.set_command_box(&cmd).and_then(|_v| Ok(()));
    }

    /// Retrieve the serial number
    pub fn get_serial_number(&mut self) -> Result<(), String> {
        let cmd: huntsman_comm::GetSerialNumber = Default::default();
        let result = self.set_command(&cmd)?;
        let response = huntsman_comm::Command::response(&cmd, &result.unwrap())?;
        let response = response
            .downcast_ref::<huntsman_comm::GetSerialNumber>()
            .unwrap();
        println!("{}", response.serial.as_ref().unwrap());
        Ok(())
    }

    /// Dump keymappings.
    pub fn dev_dump_keymaps(&mut self, hypershift: bool, profile: u8) -> Result<(), String> {
        self.set_print_comm(true);
        self.set_print_retrieve(true);
        let hypershift_flag: u8;
        if hypershift {
            hypershift_flag = 0x01;
        } else {
            hypershift_flag = 0x00;
        }
        for i in 0..=255 {
            println!("Retrieving keymapping for key {} (0x{:0>2x})", i, i);
            let cmd = huntsman_comm::ArbitraryCommand {
                register: huntsman_comm::Cmd {
                    major: 0x02,
                    minor: 0x8D,
                },
                payload: vec![profile, i, hypershift_flag],
            };
            self.set_command(&cmd)?;
            println!();
        }
        Ok(())
    }

    /// Disables led effects, turning off each led. See also [`huntsman_comm::SetLedEffect::off()`]
    pub fn effect_off(&mut self) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::off();
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Sets a fixed color on all leds. See also [`huntsman_comm::SetLedEffect::fixed()`]
    pub fn effect_fixed(&mut self, color: &RGB) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::fixed(&color);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Applies the breathing effect, fading colors in and out sequentially. See also [`huntsman_comm::SetLedEffect::breathing()`]
    pub fn effect_breathing(&mut self, colors: &Vec<RGB>) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::breathing(&colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Spectrum cycle, entire keyboard cycles the hue. See also [`huntsman_comm::SetLedEffect::spectrum()`]
    pub fn effect_spectrum(&mut self) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::spectrum();
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// A hue wave moved over the keyboard. See also [`huntsman_comm::SetLedEffect::wave()`]
    pub fn effect_wave(&mut self, direction: bool, delay: u8) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::wave(direction, delay);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Lights up keys after they are pressed. See also [`huntsman_comm::SetLedEffect::reactive()`]
    /// Only takes a single color.
    pub fn effect_reactive(
        &mut self,
        duration: huntsman_comm::Duration,
        colors: &Vec<RGB>,
    ) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::reactive(duration, colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Waves propagate outwards from pressed keys. See also [`huntsman_comm::SetLedEffect::ripple()`]
    /// Only takes a single color
    pub fn effect_ripple(&mut self, colors: &Vec<RGB>) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::ripple(&colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Keys light up randomly. See also [`huntsman_comm::SetLedEffect::starlight()`]
    /// Only takes up to two colors.
    pub fn effect_starlight(
        &mut self,
        duration: huntsman_comm::Duration,
        colors: &Vec<RGB>,
    ) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::starlight(duration, colors);
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }

    /// Display the custom frame. See also [`huntsman_comm::SetLedEffect::custom()`]
    pub fn effect_custom(&mut self) -> Result<(), String> {
        let cmd = huntsman_comm::SetLedEffect::custom();
        return self.set_command(&cmd).and_then(|_v| Ok(()));
    }
}
