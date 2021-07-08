///! Encapsulate the hardware interaction in the HidApiHal object.
extern crate hidapi;

#[derive(Debug)]
struct HalError {
    details: String,
}

impl HalError {
    fn new(msg: &str) -> HalError {
        HalError {
            details: msg.to_string(),
        }
    }
    fn boxed(msg: &str) -> Box<HalError> {
        Box::new(HalError::new(msg))
    }
}
impl std::fmt::Display for HalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HalError {}", self.details)
    }
}
impl std::error::Error for HalError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub trait HidHal {
    /// Connect to a particular usb device and endpoint id.
    fn connect(
        &mut self,
        vendor_id: u16,
        product_id: u16,
        endpoint_id: u32,
    ) -> Result<(), Box<dyn std::error::Error>>;
    /// Send bytes as a control message.
    fn control(&mut self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
    /// Retrieve the report after sending a control message. This is an echo / ack?
    fn get_report(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

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
    pub fn new() -> Result<Box<dyn HidHal>, Box<dyn std::error::Error>> {
        match hidapi::HidApi::new() {
            Err(e) => Err(Box::new(e)),
            Ok(api) => Ok(Box::new(HidApiHal {
                api: api,
                connected_device: None,
            })),
        }
    }
}

impl HidHal for HidApiHal {
    fn connect(
        &mut self,
        vendor_id: u16,
        product_id: u16,
        endpoint_id: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                        return Err(Box::new(z));
                    }
                }
            }
        }
        return Err(HalError::boxed("No device found."));
    }

    fn control(&mut self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.connected_device {
            None => Err(HalError::boxed("No connected device.")),
            Some(d) => match d.send_feature_report(prepend_zero(payload).as_slice()) {
                Err(e) => Err(Box::new(e)),
                Ok(()) => Ok(()),
            },
        }
    }

    fn get_report(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buff: [u8; 91] = [0; 91]; // This also specifies the length.
        buff[0] = 0;
        match &mut self.connected_device {
            None => Err(HalError::boxed("No connected device.")),
            Some(d) => match d.get_feature_report(&mut buff) {
                Err(e) => Err(HalError::boxed(&format!("{:?}", e))),
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

pub struct DryHidHal {
    pub buffer: [u8; 90],
    pub len: usize,
}

impl DryHidHal {
    /// Attempt to instantiate the hid api.
    pub fn new() -> Result<Box<dyn HidHal>, Box<dyn std::error::Error>> {
        Ok(Box::new(DryHidHal {
            buffer: [0; 90],
            len: 0,
        }))
    }
}

impl HidHal for DryHidHal {
    fn connect(
        &mut self,
        _vendor_id: u16,
        _product_id: u16,
        _endpoint_id: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Send bytes as a control message.
    fn control(&mut self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.buffer.fill(0);
        for (i, x) in payload.iter().enumerate() {
            self.buffer[i] = *x;
        }
        self.len = payload.len();
        Ok(())
    }

    /// Retrieve the report after sending a control message. This is an echo / ack?
    fn get_report(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.buffer[0] = 0x02;
        return Ok(self.buffer.to_vec());
    }
}
