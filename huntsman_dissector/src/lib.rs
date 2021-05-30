extern crate wireshark_dissector_rs;

use wireshark_dissector_rs::dissector;
use wireshark_dissector_rs::dissector::PacketField;
use wireshark_dissector_rs::epan;

extern crate huntsman_comm;

extern crate struct_helper;
use struct_helper::StructHelper;
use huntsman_comm::wire;
// use struct_helper::StructHelper;

// Lift these to make it less verbose.
type FieldType = dissector::FieldType;
type FieldDisplay = dissector::FieldDisplay;
type Encoding = epan::proto::Encoding;

struct HuntsmanDissector {
    field_mapping: Vec<(PacketField, epan::proto::HFIndex)>,
    tree_indices: Vec<epan::proto::ETTIndex>,
}

enum TreeIdentifier
{
    Root,
    Last
}

impl HuntsmanDissector {
    const ROOT: PacketField = PacketField::fixed("Huntsman Protocol","huntsman.proto", FieldType::PROTOCOL,FieldDisplay::BASE_NONE);
    const FULL_PAYLOAD: PacketField = PacketField::fixed("Payload","huntsman.payload",FieldType::BYTES,FieldDisplay::BASE_NONE);

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

    fn get_tree_id(self: &Self, identifier: TreeIdentifier) -> epan::proto::ETTIndex {
        match identifier {
            TreeIdentifier::Root => return self.tree_indices[0],
            TreeIdentifier::Last => {
                panic!("Nope");
            }
        };
    }

    fn new() -> HuntsmanDissector {
        HuntsmanDissector {
            field_mapping: Vec::new(),
            tree_indices: Vec::new(),
        }
    }

    fn dissect_private(
        self: &Self,
        proto: &mut epan::ProtoTree,
        tvb: &mut epan::TVB,
        mut offset: usize,
    ) -> usize {
        let data_start_offset = offset;
        let length = tvb.reported_length() - offset;

        // Now, we can actually do things.
        let mut root_item = proto.add_item(
            self.get_id(&HuntsmanDissector::ROOT),
            tvb,
            offset,
            0,
            Encoding::BIG_ENDIAN,
        );
        let mut root = root_item.add_subtree(self.get_tree_id(TreeIdentifier::Root));

        root.add_item(
            self.get_id(&HuntsmanDissector::FULL_PAYLOAD),
            tvb,
            offset,
            length,
            Encoding::BIG_ENDIAN,
        );

        tvb.reported_length()
    }
}

impl dissector::Dissector for HuntsmanDissector {
    fn get_fields(self: &Self) -> Vec<dissector::PacketField> {
        let mut f = Vec::new();
        f.push(HuntsmanDissector::ROOT);
        f.push(HuntsmanDissector::FULL_PAYLOAD);
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
        return vec![
            //~ dissector::Registration::Post,
            //~ dissector::Registration::UInt {
            //~ abbrev: "usb.product",
            //~ pattern: 0x15320226,
            //~ },

            // We could use a heuristic dissector on the usb.control
            dissector::Registration::Heuristic {
                table: "usb.control",
                display_name: "huntsman",
                internal_name: "huntsman",
                enabled: true,
            },
        ];
    }

    fn get_tree_count(self: &Self) -> usize {
        return TreeIdentifier::Last as usize;
    }

    fn set_tree_indices(self: &mut Self, ett_indices: Vec<epan::proto::ETTIndex>) {
        self.tree_indices = ett_indices;
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




#[derive(Default, Debug, Clone, Copy)]
struct FieldFlags
{
    hidden: bool
}

#[derive(Default, Debug, Clone)]
struct DissectionField
{
    start: usize,
    length: usize,
    abbrev: Vec<String>,
    flags: FieldFlags,
    type_name: &'static str,
}

fn flatten_field_tree(field: &struct_helper::Field, flags: &FieldFlags, prefix: Vec<String>, offset: usize) -> Vec<DissectionField>
{
    let mut res : Vec<DissectionField> = Vec::new();
    let mut updated_prefix = prefix;
    match field.info.name
    {
        Some(n) => {
            updated_prefix.push(n.to_string());
        },
        None =>{},
    }

    let mut updated_flags = flags.clone();
    // println!("attrs: {:?}", field.info.attrs);
    match field.info.attrs.get("dissection_hide")
    {
        Some(v) => updated_flags.hidden = *v == "true",
        None => {},
    }


    for k in 0..field.children.len()
    {
        let c = &field.children[k];
        let mut child_prefix = updated_prefix.clone();
        println!("K: {:#?}", c);
        if field.info.element_type == struct_helper::ElementType::Array
        {
            child_prefix.push(format!("[{}]", k));
        }
        // println!("
        res.append(&mut flatten_field_tree(c, &updated_flags, child_prefix, field.info.start + offset));
    }
    if field.children.is_empty()
    {
        // We are a leaf.
        res.push(DissectionField{
            flags: updated_flags,
            abbrev: updated_prefix,
            start: offset,
            length: field.info.length,
            type_name: field.info.type_name,
        });
    }
    return res;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn wrangle_commands_into_fields()
    {
        let dissection_fields: Vec<PacketField> = Vec::new();
        let command_fields = wire::Command::fields();
        println!("{:#?}", command_fields);
        let flags: FieldFlags = Default::default();
        let command_flattened = flatten_field_tree(&command_fields, &flags, vec!(), 0);
        println!("{:?}", command_flattened);
        println!("{:?}", command_fields.find("checksum"));
        // wire::Command

        let ledstate_fields = wire::SetLedState::fields();
        let ledstate_flattened = flatten_field_tree(&ledstate_fields, &flags, vec!(), command_fields.find("payload").expect("Payload should exist").info.start);
        println!("{:?}", ledstate_flattened);
        
        // wire::SetLedState
        // wire::SetBrightness
    }
}

