//! Not much in the sense of external documentation here. This combines the [`wireshark_dissector_rs`]
//! , [`struct_helper`] and [`huntsman_comm`] together to automatically dissect the structures that
//! go over the USB bus.

extern crate wireshark_dissector_rs;

use wireshark_dissector_rs::{dissector, dissector::PacketField, epan};

// Lift these to make it less verbose.
type FieldType = dissector::FieldType;
type FieldDisplay = dissector::FieldDisplay;
type Encoding = epan::proto::Encoding;

extern crate huntsman_comm;
use huntsman_comm::wire;

extern crate struct_helper;
use struct_helper::{Inspectable, FromBytes};

mod util;
use util::*;

use std::collections::HashMap;

struct HuntsmanDissector {
    // Pre-setup population of the to be dissected fields..
    dissection_fields: Vec<DissectionField>,
    foldouts: Vec<String>,

    // Post setup lookups.
    field_mapping: Vec<(PacketField, epan::proto::HFIndex)>,
    fold_mapping: HashMap<String, epan::proto::ETTIndex>,
}

impl HuntsmanDissector {
    // Two static entries, just for the fields we manually specify and always have available.
    const ROOT: PacketField = PacketField::fixed(
        "Huntsman Protocol",
        "huntsman.proto",
        FieldType::PROTOCOL,
        FieldDisplay::BASE_NONE,
    );
    const FULL_PAYLOAD: PacketField = PacketField::fixed(
        "Payload",
        "huntsman.payload",
        FieldType::BYTES,
        FieldDisplay::BASE_NONE,
    );

    /// Size of the msgs we are interested in for the heuristic dissector.
    const EXPECTED_MSG_LENGTH: usize = 90;
}

impl HuntsmanDissector {
    /// Retrieve the hf index for the provided PacketField, lookup by abbreviation.
    fn get_id(self: &Self, desired_field: &dissector::PacketField) -> epan::proto::HFIndex {
        self.get_id_by_abbrev(desired_field.abbrev.as_str())
    }

    /// Retrieve the hf index based on the abbreviation.
    fn get_id_by_abbrev(&self, name: &str) -> epan::proto::HFIndex {
        for (field, index) in &self.field_mapping {
            if field.abbrev == name {
                return *index;
            }
        }
        panic!("Couldn't find field id for {:?}", name);
    }

    /// Retrieve the tree foldout index.
    fn get_ett_id(&self, name: &str) -> epan::proto::ETTIndex {
        *self
            .fold_mapping
            .get(name)
            .expect(format!("Should have '{}' in index, otherwise its a bug.", name).as_str())
    }

    /// Create a new instance of our dissector.
    fn new() -> HuntsmanDissector {
        let (fields, mut foldouts) = make_all_fields();
        foldouts.push("ROOT".to_string());
        foldouts.push("PAYLOAD".to_string());
        HuntsmanDissector {
            field_mapping: Vec::new(),
            dissection_fields: fields,
            fold_mapping: Default::default(),
            foldouts: foldouts,
        }
    }

    /// The actual dissection message called from the heuristic dissector.
    fn dissect_private(
        self: &Self,
        proto: &mut epan::ProtoTree,
        tvb: &mut epan::TVB,
        mut offset: usize,
    ) -> usize {
        let length = tvb.reported_length() - offset;

        // Now, we can actually do things, first, we add our root element & foldout
        let mut root_item = proto.add_item(
            self.get_id(&HuntsmanDissector::ROOT),
            tvb,
            offset,
            0,
            Encoding::BIG_ENDIAN,
        );
        let mut root = root_item.add_subtree(self.get_ett_id("ROOT"));

        // Then, add an element for the payload
        let mut payload_item = root.add_item(
            self.get_id(&HuntsmanDissector::FULL_PAYLOAD),
            tvb,
            offset,
            length,
            Encoding::BIG_ENDIAN,
        );
        let mut _payload_root = payload_item.add_subtree(self.get_ett_id("PAYLOAD"));

        // Iterate over all the fields.
        let flags: FieldFlags = Default::default();
        let command_fields = wire::Command::fields();

        let mut proto_stack: Vec<epan::ProtoTree> = vec![root];

        // Cheat here, just retrieve the byte slice from wireshark, then construct the command from that in one go.
        // We could also have wireshark assembly it through the references during the tree traversal.
        let left = (tvb.reported_length_remaining(offset)) as usize;
        let command_block = tvb.get_mem(offset, left);

        // We don't support the 'union' here, where we nest subparsers easily... If we fail with this
        // parse, we should really fail the dissection nicely instead of panic.
        let command: wire::Command =
            wire::Command::from_le_bytes(&command_block).expect("Should be good");

        // The actual dissection happens in this visitor.
        let mut dissection_visitor: Visitor =
            &mut |loc: Location,
                  field: &dyn Inspectable,
                  prefix: &Vec<Prefix>,
                  flags: &FieldFlags,
                  visit_offset: usize| {
                if flags.hidden {
                    return; // skip over it, like the individual bytes in the payload.
                }

                match loc {
                    Location::Leaf => {
                        // it's a leaf, add an item representing these bytes.
                        let name = make_field_abbrev(prefix);
                        let hfid = self.get_id_by_abbrev(&name);
                        proto_stack.last_mut().as_mut().unwrap().add_item(
                            hfid,
                            tvb,
                            visit_offset,
                            field.length(),
                            Encoding::BIG_ENDIAN,
                        );
                    }
                    Location::MultipleChildrenStart => {
                        // We're going into something with multiple children, add a foldout and
                        // a section.
                        let name = make_field_abbrev(&make_fold_item_label(prefix));
                        let hfid = self.get_id_by_abbrev(&name);
                        let mut root_item = proto_stack.last_mut().as_mut().unwrap().add_item(
                            hfid,
                            tvb,
                            visit_offset,
                            field.length(),
                            Encoding::BIG_ENDIAN,
                        );

                        let thing = root_item
                            .add_subtree(self.get_ett_id(&make_fold_label(&prefix)))
                            .clone();
                        proto_stack.push(thing);
                    }
                    Location::MultipleChildrenEnd => {
                        proto_stack.pop();
                    }
                };
            };

        // Recurse over the command fields.
        field_recurser(
            &command,
            &flags,
            prefix_start(),
            offset,
            &mut dissection_visitor,
        );

        // We have the command, now we can match on the payload.
        offset += command
            .get("payload")
            .expect("Payload should exist")
            .start();

        let cmd_id = huntsman_comm::Cmd {
            major: command.cmd.major,
            minor: command.cmd.minor,
        };
        let mut fields: Option<Box<dyn Inspectable>> = None;

        // Iterate over all known commands from the comms side, and dissect their wire definitions
        // if available.
        for (cmd, field_fun) in huntsman_comm::get_command_fields().iter() {
            if cmd_id == *cmd {
                fields = Some(field_fun());
                break;
            }
        }

        if let Some(f) = fields {
            field_recurser(f.as_ref(), &flags, prefix_start(), offset, &mut dissection_visitor);
        }

        // Return how many bytes we read.
        tvb.reported_length()
    }
}

impl dissector::Dissector for HuntsmanDissector {
    fn get_fields(self: &Self) -> Vec<dissector::PacketField> {
        let mut f = Vec::new();
        f.push(HuntsmanDissector::ROOT);
        f.push(HuntsmanDissector::FULL_PAYLOAD);
        f.append(&mut fields_to_dissector(&self.dissection_fields));
        return f;
    }

    fn set_field_indices(
        self: &mut Self,
        hfindices: Vec<(dissector::PacketField, epan::proto::HFIndex)>,
    ) {
        self.field_mapping = hfindices;
    }

    fn heuristic_dissect(self: &Self, proto: &mut epan::ProtoTree, tvb: &mut epan::TVB) -> bool {
        let remaining = tvb.reported_length();
        let expected_length: usize = HuntsmanDissector::EXPECTED_MSG_LENGTH;
        if remaining < expected_length {
            return false; // message is too short, can never be for us.
        }

        // Grab the last 90 bytes.
        let section = tvb.get_mem(remaining - expected_length, expected_length);

        // Checksum is xor based, if we see the message id increment with same message, the output is increasing by that
        // same message id, it's not a sum, it's an xor and we skip the first byte, first two bytes also seem to have no
        // impact on the value. Last byte of the message is always zero.
        let mut checksum: u8 = 0;
        for i in 2..expected_length - 2 {
            checksum ^= section[i];
        }

        if checksum != section[section.len() - 2] {
            return false; // checksum didn't match, likely not our protocol.
        }

        if *section.last().unwrap() != 0u8
        // last byte wasn't zero, all of them have that?
        {
            return false;
        }

        // Yes, it's for us, let us dissect it.
        self.dissect_private(proto, tvb, remaining - expected_length);
        return true;
    }

    fn get_protocol_name(self: &Self) -> (&'static str, &'static str, &'static str) {
        return ("Dissector for Razer Huntsman Elite", "huntsman", "huntsman");
    }

    fn get_registration(self: &Self) -> Vec<dissector::Registration> {
        return vec![dissector::Registration::Heuristic {
            table: "usb.control",
            display_name: "huntsman",
            internal_name: "huntsman",
            enabled: true,
        }];
    }

    fn get_tree_count(self: &Self) -> usize {
        return self.foldouts.len();
    }

    fn set_tree_indices(self: &mut Self, ett_indices: Vec<epan::proto::ETTIndex>) {
        ett_indices.iter().enumerate().for_each(|(i, index)| {
            self.fold_mapping.insert(self.foldouts[i].clone(), *index);
        });
    }
}

// This function is the main entry point for the plugin. It's the only symbol called automatically.
use std::rc::Rc;
#[no_mangle]
pub fn plugin_register() {
    let z = Rc::new(HuntsmanDissector::new());
    dissector::setup(z);
}

// And we need these public symbols to tell wireshark we are the right version.
#[no_mangle]
static plugin_version: [libc::c_char; 4] = [50, 46, 54, 0]; // "2.6"
#[no_mangle]
static plugin_release: [libc::c_char; 4] = [50, 46, 54, 0]; // "2.6"

// Later versions of wireshark also want these integers.
#[no_mangle]
static plugin_want_major: u32 = 3;
#[no_mangle]
static plugin_want_minor: u32 = 5;
