use std::{thread, time};
mod hid_hal;
pub struct Huntsman {
    hal: hid_hal::HidApiHal,
    print_comm: bool,
}

impl Huntsman {
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

    pub fn set_print_comm(&mut self, state: bool) {
        self.print_comm = state;
    }

    fn set_command(&mut self, command: &dyn huntsman_comm::Command) -> Result<(), String> {
        let v = command.serialize();
        if self.print_comm {
            println!("{:?} -> {:?}", command, v);
        }
        return self.hal.control(&v.as_slice());
    }

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
}
