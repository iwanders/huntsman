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

    const EXPECTED_MSG_LENGTH: usize = 90;
}

impl HuntsmanDissector {
    fn get_id(self: &Self, desired_field: &dissector::PacketField) -> epan::proto::HFIndex {
        for (field, index) in &self.field_mapping {
            if field.name == desired_field.name {
                return *index;
            }
        }
        panic!("Couldn't find field id for {:?}", desired_field);
    }

    fn get_ett_id(&self, name: &str) -> epan::proto::ETTIndex {
        *self
            .fold_mapping
            .get(name)
            .expect("Should have this index, otherwise its a bug.")
    }

    fn get_id_by_name(&self, name: &str) -> epan::proto::HFIndex {
        for (field, index) in &self.field_mapping {
            if field.abbrev == name {
                return *index;
            }
        }
        panic!("Couldn't find field id for {:?}", name);
    }

    fn new() -> HuntsmanDissector {
        let (fields, mut foldouts) = make_all_fields();
        foldouts.push("ROOT".to_string());
        HuntsmanDissector {
            field_mapping: Vec::new(),
            dissection_fields: fields,
            fold_mapping: Default::default(),
            foldouts: foldouts,
        }
    }

    fn dissect_private(
        self: &Self,
        proto: &mut epan::ProtoTree,
        tvb: &mut epan::TVB,
        mut offset: usize,
    ) -> usize {
        let length = tvb.reported_length() - offset;

        // Now, we can actually do things.
        let mut root_item = proto.add_item(
            self.get_id(&HuntsmanDissector::ROOT),
            tvb,
            offset,
            0,
            Encoding::BIG_ENDIAN,
        );
        let mut root = root_item.add_subtree(self.get_ett_id("ROOT"));

        root.add_item(
            self.get_id(&HuntsmanDissector::FULL_PAYLOAD),
            tvb,
            offset,
            length,
            Encoding::BIG_ENDIAN,
        );

        // Iterate over all the fields.
        let flags: FieldFlags = Default::default();
        let command_fields = wire::Command::fields();
        self.dissection_recurser(
            tvb,
            &mut root,
            &command_fields,
            vec!["huntsman".to_string()],
            offset,
            flags,
        );
        // Should we somehow return the value on which we expect the next parser to build? The payload chunk?

        // Cheat here, just retrieve the byte slice from wireshark, then construct the command from that in one go.
        // We could also have wireshark assembly it through the references during the tree traversal.
        let left = (tvb.reported_length_remaining(offset)) as usize;
        let command_block = tvb.get_mem(offset, left);
        let command: wire::Command =
            wire::Command::from_le_bytes(&command_block).expect("Should be good");

        // We have the command, now we can match on the payload.
        offset += command_fields
            .find("payload")
            .expect("Payload should exist")
            .info
            .start;
        let cmd_id = (command.cmd_major, command.cmd_minor);
        match cmd_id {
            huntsman_comm::SetLedState::CMD => {
                let fields = wire::SetLedState::fields();
                self.dissection_recurser(
                    tvb,
                    &mut root,
                    &fields,
                    vec!["huntsman".to_string()],
                    offset,
                    flags,
                );
            }
            huntsman_comm::SetBrightness::CMD => {
                let fields = wire::SetBrightness::fields();
                self.dissection_recurser(
                    tvb,
                    &mut root,
                    &fields,
                    vec!["huntsman".to_string()],
                    offset,
                    flags,
                );
            }
            _ => {}
        }

        let _z = offset;

        tvb.reported_length()
    }

    fn dissection_recurser(
        &self,
        tvb: &mut epan::TVB,
        tree: &mut epan::ProtoTree,
        field: &struct_helper::Field,
        prefix: Vec<String>,
        offset: usize,
        flags: FieldFlags,
    ) {
        let mut updated_prefix = prefix;
        match field.info.name {
            Some(n) => {
                updated_prefix.push(n.to_string());
            }
            None => {}
        }

        let mut updated_flags = flags.clone();
        // println!("attrs: {:?}", field.info.attrs);
        match field.info.attrs.get("dissection_hide") {
            Some(v) => updated_flags.hidden = *v == "true",
            None => {}
        }

        let mut use_tree: epan::ProtoTree;
        if field.children.len() > 1 {
            let subtree_id = make_id(&updated_prefix);
            let ettid = self.get_ett_id(&subtree_id);
            // first have to make an item, only then can we make the tree onto the item.
            let mut root_item = tree.add_item(
                self.get_id(&HuntsmanDissector::ROOT),
                tvb,
                offset,
                0,
                Encoding::BIG_ENDIAN,
            );
            use_tree = root_item.add_subtree(ettid);
        } else {
            use_tree = *tree;
        }

        for k in 0..field.children.len() {
            let c = &field.children[k];
            let child_prefix = updated_prefix.clone();
            self.dissection_recurser(
                tvb,
                &mut use_tree,
                c,
                child_prefix,
                field.info.start + offset,
                updated_flags,
            );
        }

        if field.children.is_empty() && !updated_flags.hidden {
            let name = make_id(&updated_prefix);
            let hfid = self.get_id_by_name(&name);
            tree.add_item(
                hfid,
                tvb,
                offset + field.info.start,
                field.info.length,
                Encoding::BIG_ENDIAN,
            );
        }
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

// Below here is just flatenning of the fields such that we get a vector for registration.
// it's pretty much a duplicate of the recursive dissection function, we should replace both a nice visitor pattern.

#[derive(Default, Debug, Clone, Copy)]
struct FieldFlags {
    hidden: bool,
}

#[derive(Default, Debug, Clone)]
struct DissectionField {
    start: usize,
    length: usize,
    abbrev: Vec<String>,
    flags: FieldFlags,
    type_name: &'static str,
}

fn flatten_field_tree(
    field: &struct_helper::Field,
    flags: &FieldFlags,
    prefix: Vec<String>,
    offset: usize,
) -> (Vec<DissectionField>, Vec<String>) {
    let mut res: Vec<DissectionField> = Vec::new();
    let mut foldouts: Vec<String> = Vec::new();

    let mut updated_prefix = prefix;
    match field.info.name {
        Some(n) => {
            updated_prefix.push(n.to_string());
        }
        None => {}
    }

    let mut updated_flags = flags.clone();
    // println!("attrs: {:?}", field.info.attrs);
    match field.info.attrs.get("dissection_hide") {
        Some(v) => updated_flags.hidden = *v == "true",
        None => {}
    }

    if field.children.len() > 1 {
        foldouts.push(make_id(&updated_prefix));
    }

    for k in 0..field.children.len() {
        let c = &field.children[k];
        let child_prefix = updated_prefix.clone();
        let (mut fields, mut folds) =
            flatten_field_tree(c, &updated_flags, child_prefix, field.info.start + offset);
        res.append(&mut fields);
        foldouts.append(&mut folds);
    }

    if field.children.is_empty() {
        // We are a leaf, add the final field we'll be dissecting as.
        res.push(DissectionField {
            flags: updated_flags,
            abbrev: updated_prefix,
            start: offset,
            length: field.info.length,
            type_name: field.info.type_name,
        });
    }
    return (res, foldouts);
}

fn make_id(v: &Vec<String>) -> String {
    v.join(".")
}

fn fields_to_dissector(v: &Vec<DissectionField>) -> Vec<dissector::PacketField> {
    v.iter()
        .map(|x| {
            dissector::PacketField {
                name: dissector::StringContainer::String(String::from(x.abbrev.last().unwrap())),
                abbrev: dissector::StringContainer::String(String::from(make_id(&x.abbrev))),
                field_type: FieldType::UINT8, // need to select this based on the type.
                display: FieldDisplay::BASE_HEX,
            }
        })
        .collect()
}

fn collect_payloads<T: StructHelper>(
    dissections: &mut Vec<DissectionField>,
    folds: &mut Vec<String>,
    offset: usize,
) {
    let ledstate_fields = <T>::fields();
    let flags: FieldFlags = Default::default();
    let (mut field_res, mut fold_res) = flatten_field_tree(
        &ledstate_fields,
        &flags,
        vec!["huntsman".to_string()],
        offset,
    );
    dissections.append(&mut field_res);
    folds.append(&mut fold_res);
}

fn make_all_fields() -> (Vec<DissectionField>, Vec<String>) {
    let mut all_fields: Vec<DissectionField> = Vec::new();
    let mut folds: Vec<String> = Vec::new();

    let command_fields = wire::Command::fields();
    let flags: FieldFlags = Default::default();
    let (mut field_res, mut fold_res) =
        flatten_field_tree(&command_fields, &flags, vec!["huntsman".to_string()], 0);
    all_fields.append(&mut field_res);
    folds.append(&mut fold_res);

    let payload_offset = command_fields
        .find("payload")
        .expect("Payload should exist")
        .info
        .start;

    collect_payloads::<wire::SetLedState>(&mut all_fields, &mut folds, payload_offset);
    collect_payloads::<wire::SetBrightness>(&mut all_fields, &mut folds, payload_offset);

    folds.sort_unstable();
    folds.dedup();

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
