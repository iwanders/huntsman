
extern crate huntsman_comm;

fn prepend_zero(v: &[u8]) -> Vec<u8>
{
    let mut new_v : Vec<u8> = Vec::new();
    new_v.push(0);
    for i in 0..v.len()
    {
        new_v.push(v[i])
    }
    return new_v;
}

use std::{thread, time};

pub fn main()
{
    extern crate hidapi;

    let api = hidapi::HidApi::new().unwrap();
    // Print out information about all connected devices
    for device in api.device_list() {
        //~ println!("{:#?}", device);
        //~ println!("{:#?}", device.interface_number());
        if device.interface_number() == 2 && device.vendor_id() == 0x1532
        {
            match device.open_device(&api)
            {
                Ok(d) => {
                    println!("Device opened correcvtly");
                    //~ println!("{}", d.get_product_string().unwrap().unwrap());
                    //~ println!("{}", d.get_serial_number_string().unwrap().unwrap());
                    let mut counter : usize = 0;
                    while (counter < 500)
                    {
                        for i in 0..9
                        {
                            let mut leds = huntsman_comm::SetLedState::make_test_red();
                            leds.id = i;
                            for l in 0..leds.count as usize
                            {
                                match (counter % 3)
                                {
                                    0 => leds.leds[l].r = 0xff,
                                    1 => leds.leds[l].g = 0xff,
                                    2 => leds.leds[l].b = 0xff,
                                    _ => {println!("Huh? {}", counter);}
                                }
                                
                            }
                            //~ let res = d.write(prepend_zero(huntsman_comm::Command::serialize(&leds).as_slice()).as_slice());
                            let res = d.send_feature_report(prepend_zero(huntsman_comm::Command::serialize(&leds).as_slice()).as_slice());
                            println!("res: {:?}", res);
                            thread::sleep(time::Duration::from_millis(1));
                        }
                        thread::sleep(time::Duration::from_millis(100));
                        counter += 1;
                    }
                },
                Err(z) =>
                {
                    println!("Bummer {:?}", z);
                }
            }
        }
    }

    // Connect to device using its VID and PID
    //~ let (VID, PID) = (0x0123, 0x3456);
    //~ let device = api.open(VID, PID).unwrap();

    // Read data from device
    //~ let mut buf = [0u8; 8];
    //~ let res = device.read(&mut buf[..]).unwrap();
    //~ println!("Read: {:?}", &buf[..res]);

    // Write data to device
    //~ let buf = [0u8, 1, 2, 3, 4];
    //~ let res = device.write(&buf).unwrap();
    //~ println!("Wrote: {:?} byte(s)", res);
}