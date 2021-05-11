extern crate wireshark_dissector_rs;

use wireshark_dissector_rs::dissector;
use wireshark_dissector_rs::epan;


// Lift these to make it less verbose.
type FieldType = dissector::FieldType;
type FieldDisplay = dissector::FieldDisplay;
type Encoding = epan::proto::Encoding;


#[repr(usize)]
enum TreeIdentifier {
    Root,
    Last,
}
struct HuntsmanDissector {
    field_mapping: Vec<(dissector::PacketField, epan::proto::HFIndex)>,
    tree_indices: Vec<epan::proto::ETTIndex>,
}
impl HuntsmanDissector {
    const ROOT: dissector::PacketField = dissector::PacketField {
        name: "Huntsman Protocol",
        abbrev: "huntsman.proto",
        field_type: FieldType::PROTOCOL,
        display: FieldDisplay::BASE_NONE,
    };
    const FULL_PAYLOAD: dissector::PacketField = dissector::PacketField {
        name: "Payload",
        abbrev: "huntsman.payload",
        field_type: FieldType::BYTES,
        display: FieldDisplay::BASE_NONE,
    };
    const DIRECTION: dissector::PacketField = dissector::PacketField {
        name: "Direction or status?",
        abbrev: "huntsman.status",
        field_type: FieldType::UINT8,  // Should really add enum support...
        display: FieldDisplay::BASE_DEC,
    };
    const SEQUENCE: dissector::PacketField = dissector::PacketField {
        name: "Some sequence_number",
        abbrev: "huntsman.status",
        field_type: FieldType::UINT8,
        display: FieldDisplay::BASE_DEC,
    };
    const CHECKSUM: dissector::PacketField = dissector::PacketField {
        name: "Checksum",
        abbrev: "huntsman.checksum",  // second last byte... pretty sure about this one.
        field_type: FieldType::UINT8,
        display: FieldDisplay::BASE_HEX,
    };

    const EXPECTED_MSG_LENGTH: usize = 90;
}

enum Direction
{
    HostToDevice,
    DeviceToHost
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

    fn dissect_private(self: &Self, proto: &mut epan::ProtoTree, tvb: &mut epan::TVB, mut offset: usize) -> usize {
        let data_start_offset = offset;
        let length = tvb.reported_length()-offset;

        // Now, we can actually do things.
        let mut root_item = proto.add_item(self.get_id(&HuntsmanDissector::ROOT), tvb, offset, 0, Encoding::BIG_ENDIAN);
        let mut root = root_item.add_subtree(self.get_tree_id(TreeIdentifier::Root));

        root.add_item(
            self.get_id(&HuntsmanDissector::FULL_PAYLOAD),
            tvb,
            offset,
            length,
            Encoding::BIG_ENDIAN,
        );

        root.add_item(
            self.get_id(&HuntsmanDissector::DIRECTION),
            tvb,
            offset,
            1,
            Encoding::BIG_ENDIAN,
        );
        offset += 1;

        offset += 9;
        root.add_item(
            self.get_id(&HuntsmanDissector::SEQUENCE),
            tvb,
            offset,
            1,
            Encoding::BIG_ENDIAN,
        );
        offset += 1;



        root.add_item(
            self.get_id(&HuntsmanDissector::CHECKSUM),
            tvb,
            data_start_offset + length - 2 ,
            1,
            Encoding::BIG_ENDIAN,
        );
        offset += 1;

        tvb.reported_length()
    }


}


impl dissector::Dissector for HuntsmanDissector {
    fn get_fields(self: &Self) -> Vec<dissector::PacketField> {
        let mut f = Vec::new();
        f.push(HuntsmanDissector::ROOT);
        f.push(HuntsmanDissector::FULL_PAYLOAD);
        f.push(HuntsmanDissector::DIRECTION);
        f.push(HuntsmanDissector::CHECKSUM);
        f.push(HuntsmanDissector::SEQUENCE);
        return f;
    }

    fn set_field_indices(self: &mut Self, hfindices: Vec<(dissector::PacketField, epan::proto::HFIndex)>) {
        self.field_mapping = hfindices;
    }

    fn heuristic_dissect(self: &Self, proto: &mut epan::ProtoTree, tvb: &mut epan::TVB) -> bool
    {
        let remaining = tvb.reported_length();
        let expected_length : usize = HuntsmanDissector::EXPECTED_MSG_LENGTH;
        if remaining < expected_length
        {
            return false;  // message is too short, can never be for us.
        }

        // Grab the last 90 bytes.
        let section = tvb.get_mem(remaining - expected_length, expected_length);

        // Checksum is xor based, if we see the message id increment with same message, the output is increasing by that
        // same message id, it's not a sum, it's an xor and we skip the first byte, first two bytes also seem to have no
        // impact on the value. Last byte of the message is always zero.
        let mut checksum : u8 = 0;
        for i in 2..expected_length-2
        {
            checksum ^= section[i];
        }

        if checksum != section[section.len() - 2]
        {
            return false;  // checksum didn't match, likely not our protocol.
        }

        if *section.last().unwrap() != 0u8  // last byte wasn't zero, all of them have that?
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
