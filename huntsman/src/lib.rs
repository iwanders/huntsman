use std::{thread, time};
mod hid_hal;
pub struct Huntsman {
    hal: hid_hal::HidApiHal,
}

impl Huntsman {
    pub fn new() -> Result<Huntsman, String> {
        match hid_hal::HidApiHal::new() {
            Ok(mut hal) => match &mut hal.connect(0x1532, 0x226, 2) {
                Ok(()) => Ok(Huntsman { hal: hal }),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn do_flashy_things(&mut self, delay: u64) -> Result<(), String> {
        let mut counter: usize = 0;
        while (counter < 500) {
            for i in 0..9 {
                let mut leds = huntsman_comm::SetLedState::make_test_red();
                leds.id = i;
                for l in 0..leds.count as usize {
                    match (counter % 3) {
                        0 => leds.leds[l].r = 0xff,
                        1 => leds.leds[l].g = 0xff,
                        2 => leds.leds[l].b = 0xff,
                        _ => {
                            println!("Huh? {}", counter);
                        }
                    }
                }
                let res = self
                    .hal
                    .control(huntsman_comm::Command::serialize(&leds).as_slice())?;
                //~ thread::sleep(time::Duration::from_millis(1));
            }
            thread::sleep(time::Duration::from_millis(delay));
            counter += 1;
        }
        return Ok(());
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), String> {
        let mut counter: usize = 0;
        for i in 0..9 {
            let mut leds = huntsman_comm::SetLedState::make_test_red();
            leds.id = i;
            for l in 0..leds.count as usize {
                leds.leds[l].r = r;
                leds.leds[l].g = g;
                leds.leds[l].b = b;
            }
            let res = self
                .hal
                .control(huntsman_comm::Command::serialize(&leds).as_slice())?;
        }
        return Ok(());
    }
}
