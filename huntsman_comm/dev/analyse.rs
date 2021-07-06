use std::process::Command;
extern crate serde_json;

extern crate huntsman_comm;
use huntsman_comm::*;

extern crate clap;
use clap::{App, Arg, SubCommand};

use serde_json::Value;
// enum Value {
// Null,
// Bool(bool),
// Number(Number),
// String(String),
// Array(Vec<Value>),
// Object(Map<String, Value>),
// }

// Making an error that takes a string is... more work than hoped.
#[derive(Debug, Clone)]
struct StrError {
    details: String,
}
impl StrError {
    fn new(msg: &str) -> Box<StrError> {
        Box::new(StrError {
            details: msg.to_string(),
        })
    }
}
impl std::fmt::Display for StrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl std::error::Error for StrError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Default, Clone)]
struct Frame {
    frame_time: String,
    frame_time_epoch: f64,
    data: Vec<u8>,
}

fn as_obj(v: &Value) -> Option<&serde_json::Map<String, serde_json::Value>> {
    if let Value::Object(frame) = v {
        return Some(frame);
    }
    None
}

fn as_string(v: &Value) -> Option<&String> {
    if let Value::String(f) = v {
        return Some(f);
    }
    None
}

fn obtain_frames(filename: &str) -> Result<Vec<Frame>, Box<dyn std::error::Error>> {
    let mut frames: Vec<Frame> = Vec::new();

    let res = Command::new("tshark")
        .arg("-n")
        .arg("-r")
        .arg(filename)
        .arg("-Y")
        .arg("(huntsman.Command.cmd.u16.cmd)")
        .arg("-Tjson")
        .output()
        .expect("failed to execute process");

    // println!("Res: {:?}", res);
    let v: serde_json::Value = serde_json::from_str(std::str::from_utf8(res.stdout.as_slice())?)?;

    if let Value::Array(packets) = v {
        for p in packets.iter() {
            let source = p.get("_source");
            if source.is_none() {
                continue;
            }

            let obj = as_obj(source.unwrap());
            if obj.is_none() {
                continue;
            }

            let layer_thing = obj.unwrap().get("layers");
            if layer_thing.is_none() {
                continue;
            }

            // println!("{:?}", obj);
            let layers = as_obj(layer_thing.unwrap()).unwrap();
            // println!("{:?}", layers.get("frame"));
            // println!("{:?}", layers);
            let huntsman_thing = layers.get("huntsman.proto");
            if huntsman_thing.is_none() {
                continue;
            }
            let frame_thing = layers.get("frame");
            if frame_thing.is_none() {
                continue;
            }

            let frame = as_obj(frame_thing.unwrap()).unwrap();
            // println!("{:?}", frame);
            let frame_time = as_string(frame.get("frame.time").unwrap()).unwrap().clone();
            let frame_time_epoch = as_string(frame.get("frame.time_epoch").unwrap())
                .unwrap()
                .parse::<f64>()
                .unwrap();
            let huntsman_proto = as_obj(huntsman_thing.unwrap()).unwrap();
            let data = parse_wireshark_value(
                as_string(huntsman_proto.get("huntsman.payload").unwrap()).unwrap(),
            );
            let frame = Frame {
                frame_time,
                frame_time_epoch,
                data,
            };

            frames.push(frame);
        }
    }
    Ok(frames)
}

fn payload_as<T: FromBytes + Default + std::fmt::Debug>(command: &wire::Command) -> String {
    format!("{:?}", T::from_be_bytes(&command.payload).unwrap())
}

fn payload_str(command: &wire::Command) -> Result<String, Box<dyn std::error::Error>> {
    let p = &command;
    match command.cmd {
        GetStorageStatistics::CMD => Ok(payload_as::<wire::GetStorageStatistics>(p)),
        SetLedBrightness::CMD => Ok(payload_as::<wire::SetLedBrightness>(p)),
        SetGameMode::CMD => Ok(payload_as::<wire::SetGameMode>(p)),
        SetKeyOverride::CMD => Ok(payload_as::<wire::SetKeyOverride>(p)),
        MacroActions::CMD => Ok(payload_as::<wire::MacroActionsPayload>(p)),
        MacroMetadata::CMD => Ok(payload_as::<wire::MacroMetadata>(p)),
        _ => Ok("".to_string()),
    }
}

fn command_dump(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<_> = matches.values_of("files").unwrap().collect();
    for k in files.iter() {
        let frames = obtain_frames(k);
        if frames.is_err()
        {
            println!("Failed to parse {}, because of {}, continuing", k, frames.expect_err("failed"));
            continue;
        }
        let frames = frames?;

        for f in frames.iter() {
            let command = wire::Command::from_be_bytes(&f.data)?;
            let payload = payload_str(&command)?;
            let dir = if command.status == 0 { ">" } else { "<" };
            if payload.len() == 0 {
                continue;
            }

            if matches.occurrences_of("storage") != 0 && command.cmd == GetStorageStatistics::CMD {
                if command.status != 2 {
                    continue;
                }
                print!("{:.3}", f.frame_time_epoch);
                print!(
                    "f.data: {:?}",
                    to_wireshark_value(&f.data[WIRESHARK_PAYLOAD_START..30])
                );
                println!(" {:?}", payload_as::<wire::GetStorageStatistics>(&command));
            }

            if matches.occurrences_of("macro_id") != 0 && command.cmd == MacroMetadata::CMD {
                let parsed = wire::MacroMetadata::from_be_bytes(&command.payload).unwrap();
                if command.status == 2 || parsed.page_offset != 0 {
                    continue;
                }
                let macro_id = parsed.macro_id;
                println!(
                    "((0x{:0>2x}, 0x{:0>2x}), {:?}),",
                    macro_id / 256,
                    macro_id & 0xFF,
                    parsed.uuid.uuid
                );
                continue;
            }

            if matches.occurrences_of("mappings") != 0 && command.cmd == SetKeyOverride::CMD {
                let parsed = wire::SetKeyOverride::from_be_bytes(&command.payload).unwrap();
                if command.status == 2 {
                    // continue;
                }
                println!(
                    "{:.3} {} {:0>2x} {:0>2x} {:?} {}",
                    f.frame_time_epoch,
                    dir,
                    command.cmd.major,
                    command.cmd.minor,
                    payload,
                    f.data
                        .iter()
                        .map(|x| { format!("{:0>2x}", x) })
                        .collect::<Vec<String>>()
                        .join(":")
                );
                continue;
            }

            if matches.occurrences_of("dump_all") != 0 {
                println!(
                    "{:.3} {} {:0>2x} {:0>2x} {:?}",
                    f.frame_time_epoch, dir, command.cmd.major, command.cmd.minor, payload
                );
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("Huntsman pcapng analyse tool").subcommand(
        SubCommand::with_name("dump")
            .about("run dump on all provided pcap files.")
            .arg(
                Arg::with_name("macro_id")
                    .short("-m")
                    .help("print macro ids"),
            )
            .arg(
                Arg::with_name("storage")
                    .short("-s")
                    .help("print storage retrievals"),
            )
            .arg(
                Arg::with_name("mappings")
                    .short("-k")
                    .help("print mappings"),
            )
            .arg(
                Arg::with_name("dump_all")
                    .short("-d")
                    .help("dump all commands"),
            )
            .arg(Arg::with_name("files").multiple(true)),
    );
    let matches = app.clone().get_matches(); // weird that get_matches() takes 'self', instead of &self
    match matches.subcommand() {
        (_something, Some(_subcmd)) => {}
        _ => {
            &mut app.print_help();
            println!();
            return Err(StrError::new("No subcommand given."));
        }
    }

    if let Some(matches) = matches.subcommand_matches("dump") {
        command_dump(matches)?;
    }

    Ok(())
}
