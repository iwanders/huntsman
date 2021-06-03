extern crate wireshark_dissector_rs;

use wireshark_dissector_rs::{dissector, dissector::PacketField, epan};

// Lift these to make it less verbose.
type FieldType = dissector::FieldType;
type FieldDisplay = dissector::FieldDisplay;
type Encoding = epan::proto::Encoding;

extern crate huntsman_comm;
use huntsman_comm::wire;

extern crate struct_helper;
use struct_helper::StructHelper;

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
                  field: &struct_helper::Field,
                  prefix: &Vec<Prefix>,
                  flags: &FieldFlags,
                  offset: usize| {
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
                            offset + field.info.start,
                            field.info.length,
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
                            offset,
                            field.info.length,
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
            &command_fields,
            &flags,
            prefix_start(),
            offset,
            &mut dissection_visitor,
        );

        // We have the command, now we can match on the payload.
        offset += command_fields
            .find("payload")
            .expect("Payload should exist")
            .info
            .start;

        let cmd_id = (command.cmd_major, command.cmd_minor);
        let mut fields: Option<struct_helper::Field> = None;

        for (cmd, field_fun) in huntsman_comm::get_command_fields().iter() {
            if cmd_id == *cmd {
                fields = Some(field_fun());
                break;
            }
        }

        if let Some(f) = fields {
            field_recurser(&f, &flags, prefix_start(), offset, &mut dissection_visitor);
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

// Below of this is mostly data wrangling, converting struct helper fields to PacketFields and some
// helpers to make names and ids.


/// Flags applied through the struct helper on fields.
#[derive(Default, Debug, Clone, Copy)]
struct FieldFlags {
    hidden: bool,
}

/// Internal data structur we use to represent a dissection field.
#[derive(Default, Debug, Clone)]
struct DissectionField {
    start: usize,
    length: usize,
    abbrev: Vec<Prefix>,
    flags: FieldFlags,
    type_name: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Enum to denote the location to the visitor
enum Location {
    MultipleChildrenStart,
    MultipleChildrenEnd,
    Leaf,
}

/// Enum to hold an index or string, these elements in a vector make up the name for elements.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Prefix {
    Label(String),
    Index(usize),
}

type Visitor<'a> =
    &'a mut dyn FnMut(Location, &struct_helper::Field, &Vec<Prefix>, &FieldFlags, usize) -> ();

/// Worker function to traverse the tree of [`struct_helper::Field`], calling a visitor at certain
/// locations of interest.
fn field_recurser(
    field: &struct_helper::Field,
    flags: &FieldFlags,
    prefix: Vec<Prefix>,
    offset: usize,
    visitor: &mut Visitor,
) {
    let mut updated_prefix = prefix;
    match field.info.name {
        Some(n) => {
            updated_prefix.push(Prefix::Label(n.to_string()));
        }
        None => {}
    }

    let mut updated_flags = flags.clone();
    match field.info.attrs.get("dissection_hide") {
        Some(v) => updated_flags.hidden = *v == "true",
        None => {}
    }

    if field.children.len() > 1 {
        visitor(
            Location::MultipleChildrenStart,
            &field,
            &updated_prefix,
            &updated_flags,
            offset,
        );
    }

    for k in 0..field.children.len() {
        let c = &field.children[k];
        let mut child_prefix = updated_prefix.clone();
        if field.children.len() > 1 {
            child_prefix.push(Prefix::Index(k));
        }
        field_recurser(
            &c,
            &updated_flags,
            child_prefix,
            field.info.start + offset,
            visitor,
        );
    }

    if field.children.len() > 1 {
        visitor(
            Location::MultipleChildrenEnd,
            &field,
            &updated_prefix,
            &updated_flags,
            offset,
        );
    }

    if field.children.is_empty() {
        // We are a leaf, add the final field we'll be dissecting as.
        visitor(
            Location::Leaf,
            &field,
            &updated_prefix,
            &updated_flags,
            offset,
        );
    }
}

/// Concantenate all strings in the vector.
fn make_field_abbrev(v: &Vec<Prefix>) -> String {
    v.iter()
        .filter_map(|x| match x {
            Prefix::Label(s) => Some(s.clone()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join(".")
}

/// Make an arbitrary label out of the current vector, including all integers.
fn make_fold_label(v: &Vec<Prefix>) -> String {
    v.iter()
        .map(|x| match x {
            Prefix::Label(s) => s.clone(),
            Prefix::Index(i) => i.to_string(),
        })
        .collect::<Vec<String>>()
        .join(".")
}

/// Namespace for all our prefixes / abbreviations.
fn prefix_start() -> Vec<Prefix> {
    vec![Prefix::Label("huntsman".to_string())]
}

const LABEL_STR: &'static str = "_LABEL_";

/// Make a fold label from a current prefix.
fn make_fold_item_label(v: &Vec<Prefix>) -> Vec<Prefix> {
    let mut label_prefix = v.clone();
    label_prefix.push(fold_item_label());
    label_prefix
}

/// Make the prefix we apply at the end for labels.
fn fold_item_label() -> Prefix {
    Prefix::Label(LABEL_STR.to_string())
}

/// Last element from the vector, expecting its a string.
fn get_name(v: &Vec<Prefix>) -> String {
    let mut x = v.clone();
    loop {
        let last_element = x.pop().expect("should have something");

        match &last_element {
            Prefix::Label(s) => {
                if *s == LABEL_STR.to_string() {
                    continue;
                }
            }
            _ => {}
        }

        match last_element {
            Prefix::Label(s) => {
                return s.clone();
            }
            _ => panic!(),
        }
    }
}

/// Function to conver the dissection field we use in this dissector to a packet field that's
/// used by the Dissector object.
fn fields_to_dissector(v: &Vec<DissectionField>) -> Vec<dissector::PacketField> {
    v.iter()
        .map(|x| dissector::PacketField {
            name: dissector::StringContainer::String(String::from(get_name(&x.abbrev))),
            abbrev: dissector::StringContainer::String(String::from(make_field_abbrev(&x.abbrev))),
            field_type: match x.type_name {
                "label" => FieldType::NONE,
                "u8" => FieldType::UINT8,
                _ => panic!("Unsupport type name found, add it in the dissector."),
            },
            display: match x.type_name {
                "label" => FieldDisplay::BASE_NONE,
                "u8" => FieldDisplay::BASE_HEX,
                _ => panic!("Unsupport type name found, add it in the dissector."),
            },
        })
        .collect()
}

/// Function to build the full list of fields and foldouts to be used when we register the plugin.
fn make_all_fields() -> (Vec<DissectionField>, Vec<String>) {
    let mut all_fields: Vec<DissectionField> = Vec::new();
    let mut folds: Vec<String> = Vec::new();

    let command_fields = wire::Command::fields();
    let flags: FieldFlags = Default::default();

    let mut all_leaf_fields: Visitor =
        &mut |loc: Location,
              field: &struct_helper::Field,
              prefix: &Vec<Prefix>,
              flags: &FieldFlags,
              offset: usize| {
            if flags.hidden {
                return;
            }
            match loc {
                Location::Leaf => {
                    // Actual field to dissect.
                    all_fields.push(DissectionField {
                        flags: *flags,
                        abbrev: prefix.clone(),
                        start: offset,
                        length: field.info.length,
                        type_name: field.info.type_name,
                    });
                }
                Location::MultipleChildrenStart => {
                    folds.push(make_fold_label(&prefix));
                    // Placehold field just such that we can get a nice label.
                    all_fields.push(DissectionField {
                        flags: *flags,
                        abbrev: make_fold_item_label(prefix),
                        start: offset,
                        length: field.info.length,
                        type_name: "label",
                    });
                }
                Location::MultipleChildrenEnd => {}
            };
        };

    // Actually recurse.
    field_recurser(
        &command_fields,
        &flags,
        prefix_start(),
        0,
        &mut all_leaf_fields,
    );

    let payload_offset = command_fields
        .find("payload")
        .expect("Payload should exist")
        .info
        .start;

    for (_cmd, field_fun) in huntsman_comm::get_command_fields().iter() {
        field_recurser(
            &field_fun(),
            &flags,
            prefix_start(),
            payload_offset,
            &mut all_leaf_fields,
        );
    }

    (all_fields, folds)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn wrangle_commands_into_fields() {
        let command_fields = make_all_fields();
        println!("{:#?}", command_fields.0);
        println!("{:#?}", command_fields.1);
    }
}
