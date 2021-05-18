// This file is ugly...

fn prepare_checksum(v: &Vec<u8>) -> u8 {
    let mut checksum: u8 = 0;
    for i in 2..v.len() {
        checksum ^= v[i];
    }
    return checksum;
}

pub trait Command {
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

#[derive(Default, Copy, Clone)]
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

#[derive(Default, Copy, Clone)]
pub struct SetLedState {
    pub id: u8,
    pub count: u8,
    pub leds: [RGB; 22], // 22 is the max seen?, corresponds with 0x16 in the count position.
}
impl SetLedState {
    pub const CMD: u32 = 0x4a0f0300;

    pub fn make_test_red() -> SetLedState
    {
        let mut state: SetLedState = Default::default();
        state.id = 6;
        state.count = 0x16;
        for i in 0..state.count as usize {
            if i <= (state.count - 4).into() {
                state.leds[i].r = 0xff;
            }
        }
        return state;
    }
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
    fn test_bad_add() {
        let expected = parse_wireshark_value("00:1f:00:00:00:4a:0f:03:00:00:06:00:16:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:a9:00");
        // This is a command for led id 6, with red, except for last 4 bytes, they are dark.
        let state = SetLedState::make_test_red();
        assert_eq!(state.serialize(), expected);
    }
}
