//! This crate provides an object to interface with the keyboard, it also provides a command line
//! utility that makes use of this object.

use std::{thread, time};
mod hid_hal;
use huntsman_comm::RGB;

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
    fn set_command(&mut self, command: &dyn huntsman_comm::Command) -> Result<(), String> {
        let v = command.serialize();
        if self.print_comm {
            println!("{:?} -> {:?}", command, v);
        }
        let r = self.hal.control(&v.as_slice());
        if self.print_retrieve && r.is_ok() {
            println!("<- {:?}", self.hal.get_report());
        }
        return r;
    }

    fn set_command_box(&mut self, boxed_command: &Box<dyn huntsman_comm::Command>) -> Result<(), String> {
        let v = boxed_command.serialize();
        if self.print_comm {
            println!("{:?} -> {:?}", boxed_command, v);
        }
        let r = self.hal.control(&v.as_slice());
        if self.print_retrieve && r.is_ok() {
            println!("<- {:?}", self.hal.get_report());
        }
        return r;
    }

    /// Test function to make the keyboard flash in various colors.
    pub fn do_flashy_things(&mut self, delay: u64) -> Result<(), String> {
        let mut counter: usize = 0;
        loop {
            for i in 0..9 {
                let mut leds: huntsman_comm::SetLedState = Default::default();
                leds.id = i;
                leds.count = 0x16;
                for l in 0..leds.count as usize {
                    match counter % 3 {
                        0 => leds.leds[l].r = 0xff,
                        1 => leds.leds[l].g = 0xff,
                        2 => leds.leds[l].b = 0xff,
                        _ => {
                            println!("Huh? {}", counter);
                        }
                    }
                }
                match self.set_command(&leds) {
                    Err(e) => return Err(e.to_string()),
                    _ => {}
                }
                //  thread::sleep(time::Duration::from_millis(1));
            }
            thread::sleep(time::Duration::from_millis(delay));
            counter += 1;
        }
        //  return Ok(());
    }

    /// Function to set the entire keyboard to a static color.
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), String> {
        for i in 0..9 {
            let mut leds: huntsman_comm::SetLedState = Default::default();
            leds.count = 0x16;
            leds.id = i;
            for l in 0..leds.count as usize {
                leds.leds[l].r = r;
                leds.leds[l].g = g;
                leds.leds[l].b = b;
            }
            return self.set_command(&leds);
        }
        return Ok(());
    }

    /// Function that sends a single SetLedState instruction, index is the row, start is the column, count is the number
    /// of leds to set from the index.
    pub fn set_color_single(
        &mut self,
        r: u8,
        g: u8,
        b: u8,
        count: u8,
        index: u8,
        start: u8,
    ) -> Result<(), String> {
        let mut leds: huntsman_comm::SetLedState = Default::default();
        leds.count = start + count;
        leds.id = index;
        for l in start as usize..leds.count as usize {
            leds.leds[l].r = r;
            leds.leds[l].g = g;
            leds.leds[l].b = b;
        }
        return self.set_command(&leds);
    }

    /// Set the brightness of the entire keyboard, specify as [0, 1.0].
    pub fn set_brightness(&mut self, value: f32) -> Result<(), String> {
        let mut cmd: huntsman_comm::SetBrightness = Default::default();
        cmd.value = value;
        return self.set_command(&cmd);
    }

    /// Toggle game mode on or off.
    pub fn set_game_mode(&mut self, value: bool) -> Result<(), String> {
        let mut cmd: huntsman_comm::SetGameMode = Default::default();
        cmd.value = value;
        let r = self.set_command(&cmd);
        return r;
    }

    pub fn dev_run(&mut self) -> Result<(), String> {
        self.set_print_comm(true);
        self.set_print_retrieve(true);
        // This makes it black...?
        //  0x060f0200	00:1f:00:00:00:06:0f:02:00:00:08:01:01:00:00:00:00
        //              00:1f:00:00:00:06:0f:02:02:00:03:00:00:00:00:00:00:00:00

        // let cmd = huntsman_comm::ArbitraryCommand {
            // register: huntsman_comm::Cmd{major: 0x0f, minor: 0x02},
            // payload: vec!(),
        // };
        let cmd = huntsman_comm::SetLedEffect::dev();
        return self.set_command_box(&cmd);
    }

    pub fn effect_off(&mut self) -> Result<(), String>
    {
        let cmd = huntsman_comm::SetLedEffect::off();
        return self.set_command(&cmd);
        
    }
    pub fn effect_fixed(&mut self, color: &RGB) -> Result<(), String>
    {
        let cmd = huntsman_comm::SetLedEffect::fixed(&color);
        return self.set_command(&cmd);
    }
    pub fn effect_breathing(&mut self, colors: &Vec<RGB>) -> Result<(), String>
    {
        let cmd = huntsman_comm::SetLedEffect::breathing(&colors);
        return self.set_command(&cmd);
    }
}
