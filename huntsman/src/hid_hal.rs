///! Encapsulate the hardware interaction in the HidApiHal object.
extern crate hidapi;

/// Struct to provide hardware / crate abstraction layer.
pub struct HidApiHal {
    api: hidapi::HidApi,
    connected_device: Option<hidapi::HidDevice>,
}

/// Helper function to prepend a zero to a byte slice, send_feature_report requires this.
fn prepend_zero(v: &[u8]) -> Vec<u8> {
    let mut new_v: Vec<u8> = Vec::new();
    new_v.push(0);
    for i in 0..v.len() {
        new_v.push(v[i])
    }
    return new_v;
}

impl HidApiHal {
    /// Attempt to instantiate the hid api.
    pub fn new() -> Result<HidApiHal, String> {
        match hidapi::HidApi::new() {
            Err(e) => Err(e.to_string()),
            Ok(api) => Ok(HidApiHal {
                api: api,
                connected_device: None,
            }),
        }
    }

    /// Connect to a particular usb device and endpoint id.
    pub fn connect(
        &mut self,
        vendor_id: u16,
        product_id: u16,
        endpoint_id: u32,
    ) -> Result<(), String> {
        for device in self.api.device_list() {
            if device.vendor_id() == vendor_id
                && device.product_id() == product_id
                && device.interface_number() == endpoint_id as i32
            {
                match device.open_device(&self.api) {
                    Ok(d) => {
                        self.connected_device = Some(d);
                        return Ok(());
                    }
                    Err(z) => {
                        return Err(z.to_string());
                    }
                }
            }
        }
        return Err("No device found.".to_string());
    }

    /// Send bytes as a control message.
    pub fn control(&mut self, payload: &[u8]) -> Result<(), String> {
        match &mut self.connected_device {
            None => Err("No connected device.".to_string()),
            Some(d) => match d.send_feature_report(prepend_zero(payload).as_slice()) {
                Err(e) => Err(e.to_string()),
                Ok(()) => Ok(()),
            },
        }
    }

    /// Retrieve the report after sending a control message. This is an echo / ack?
    pub fn get_report(&mut self) -> Result<Vec<u8>, String> {
        let mut buff: [u8; 91] = [0; 91]; // This also specifies the length.
        buff[0] = 0;
        match &mut self.connected_device {
            None => Err("No connected device.".to_string()),
            Some(d) => match d.get_feature_report(&mut buff) {
                Err(e) => Err(e.to_string()),
                Ok(len) => {
                    let mut z: Vec<u8> = Vec::new();
                    for i in 1..len {
                        z.push(buff[i]);
                    }
                    return Ok(z);
                }
            },
        }
    }
}
