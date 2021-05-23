use std::{thread, time};
mod hid_hal;


/// Object to interface with the Huntsman Elite keyboard.
pub struct Huntsman {
    hal: hid_hal::HidApiHal,
    print_comm: bool,
}

impl Huntsman {

    /// Construct a new Huntsman instance, this tries to connect to the usb device and errors if it can't be found.
    pub fn new() -> Result<Huntsman, String> {
        match hid_hal::HidApiHal::new() {
            Ok(mut hal) => match &mut hal.connect(0x1532, 0x226, 2) {
                Ok(()) => Ok(Huntsman {
                    hal: hal,
                    print_comm: false,
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

    /// Function to send a command to the control endpoint.
    fn set_command(&mut self, command: &dyn huntsman_comm::Command) -> Result<(), String> {
        let v = command.serialize();
        if self.print_comm {
            println!("{:?} -> {:?}", command, v);
        }
        return self.hal.control(&v.as_slice());
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
                return self.set_command(&leds);
                //~ thread::sleep(time::Duration::from_millis(1));
            }
            thread::sleep(time::Duration::from_millis(delay));
            counter += 1;
        }
        //~ return Ok(());
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
    pub fn set_brightness(&mut self, value: f32) -> Result<(), String> 
    {
        let mut cmd: huntsman_comm::SetBrightness = Default::default();
        cmd.value = value;
        return self.set_command(&cmd);
    }

    /// Toggle game mode on or off.
    pub fn set_game_mode(&mut self, value: bool) -> Result<(), String>
    {
        let mut cmd: huntsman_comm::SetGameMode = Default::default();
        cmd.value = value;
        let r = self.set_command(&cmd);
        //~ println!("Retrieve: {:?}", self.hal.get_report());
        return r;
    }
}
