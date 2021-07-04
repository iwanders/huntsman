pub const WIRESHARK_PAYLOAD_START: usize = 8;
pub fn parse_wireshark_value(z: &str) -> Vec<u8> {
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

pub fn to_wireshark_value(v: &[u8]) -> String {
    (v.clone())
        .iter()
        .map(|x| format!("{:0>2x}", x))
        .collect::<Vec<String>>()
        .join(":")
}

pub const PAYLOAD_START: usize = WIRESHARK_PAYLOAD_START;
/// Parses a wireshark value, but assumes null bytes for the remainder, does calculate
/// and compare the checksum against the provided check u8, asserts if this fails.
pub fn parse_wireshark_truncated(z: &str, check: u8) -> Vec<u8> {
    let mut v = parse_wireshark_value(z);
    const LENGTH: usize = 90;
    while v.len() != LENGTH {
        v.push(0);
    }
    let mut checksum: u8 = 0;
    for i in 2..LENGTH {
        checksum ^= v[i];
    }
    assert_eq!(check, checksum);
    v[LENGTH - 2] = checksum;
    v
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_helper() {
        let real = parse_wireshark_value("02:1f:00:00:00:0e:06:8e:ff:ff:00:01:8f:f0:00:01:8a:78:00:01:8a:78:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:f8:00");
        let truncated = parse_wireshark_truncated(
            "02:1f:00:00:00:0e:06:8e:ff:ff:00:01:8f:f0:00:01:8a:78:00:01:8a:78",
            0xf8,
        );
        assert_eq!(real, truncated);
        let real = parse_wireshark_value("00:1f:00:00:00:03:0f:04:01:00:7f:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:76:00");
        let truncated = parse_wireshark_truncated("00:1f:00:00:00:03:0f:04:01:00:7f", 0x76);
        assert_eq!(real, truncated);
    }
}
