use std::process::Command;
extern crate serde_json;
use serde_json::Value;

extern crate huntsman_comm;

// enum Value {
// Null,
// Bool(bool),
// Number(Number),
// String(String),
// Array(Vec<Value>),
// Object(Map<String, Value>),
// }

// This tip from https://www.reddit.com/r/rust/comments/8ilg97/small_tip_on_new_main_result_behavior/
// nice QOL
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = Command::new("tshark")
        .arg("-n")
        .arg("-r")
        .arg("../../test.pcapng")
        .arg("-Tjson")
        .output()
        .expect("failed to execute process");

    let v: serde_json::Value = serde_json::from_str(std::str::from_utf8(res.stdout.as_slice())?)?;

    if let Value::Array(packets) = v {
        for p in packets.iter() {
            let source = p.get("_source");
            if source.is_none() {
                continue;
            }
            if let Value::Object(obj) = source.unwrap() {
                let layer_thing = obj.get("layers");
                if layer_thing.is_none() {
                    continue;
                }
                // println!("{:?}", obj);
                if let Value::Object(layers) = layer_thing.unwrap() {
                    // println!("{:?}", layers.get("huntsman.proto"));
                    let huntsman_thing = layers.get("huntsman.proto");
                    if huntsman_thing.is_none() {
                        continue;
                    }
                    if let Value::Object(proto) = huntsman_thing.unwrap() {
                        // println!("{:?}", proto);
                        let b = if let Value::String(x) = proto.get("huntsman.payload").unwrap() {
                            huntsman_comm::parse_wireshark_value(x.as_str())
                        } else {
                            panic!()
                        };
                        println!("{:?}", b);
                    }
                }
            }
        }
    }

    // println!("v: {:?}",v);

    Ok(())
}
// tshark -n -r "$file" -Y "(huntsman.Command.cmd.u16.cmd == 0x0609 && huntsman.Command.status == 0x00)" -e huntsman.Command.cmd.u16.cmd -e huntsman.Command.len -e huntsman.payload -Tfields
