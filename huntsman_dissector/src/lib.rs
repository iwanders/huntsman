extern crate wireshark_dissector_rs;

use wireshark_dissector_rs::dissector;
use wireshark_dissector_rs::epan;
use wireshark_dissector_rs::plugin;

// Lift these to make it less verbose.
type FieldType = dissector::FieldType;
type FieldDisplay = dissector::FieldDisplay;
type Encoding = epan::proto::Encoding;


/*
    So usb is a bit clunky, we can't register a proper post dissector because the TAP in usb makes the subdissectors
    reentrant, so instead we register a postdissector and inspect whatever the HID USB dissector did and then use those
    fields or offsets.
*/

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
        name: "Direction",
        abbrev: "huntsman.direction",
        field_type: FieldType::UINT8,  // Should really add enum support...
        display: FieldDisplay::BASE_DEC,
    };
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
}

enum Direction
{
    HostToDevice,
    DeviceToHost
}

impl dissector::Dissector for HuntsmanDissector {
    fn get_fields(self: &Self) -> Vec<dissector::PacketField> {
        let mut f = Vec::new();
        f.push(HuntsmanDissector::ROOT);
        f.push(HuntsmanDissector::FULL_PAYLOAD);
        f.push(HuntsmanDissector::DIRECTION);
        return f;
    }

    fn set_field_indices(self: &mut Self, hfindices: Vec<(dissector::PacketField, epan::proto::HFIndex)>) {
        self.field_mapping = hfindices;
    }

    fn dissect(self: &mut Self, proto: &mut epan::ProtoTree, tvb: &mut epan::TVB) -> usize {
        use std::collections::HashMap;

        let mut offset: usize;
        let mut length: usize;

        
        let mut fields: HashMap<String, epan::FieldInfo> = HashMap::new();
        for field in proto.all_finfos() {
            match field.hfinfo() {
                Ok(v) => {
                fields.insert(v.abbrev().to_string(), field);
                },
                Err(e) => println!("{}", e),
            }
        }
    
        let transfer_type = fields.get("usb.transfer_type");
        match transfer_type
        {
            None => return tvb.reported_length(), // Nothing to do here, move along.
            Some(field) => 
            {
                if field.value().get_uinteger() != 0x02
                {
                    return tvb.reported_length(); // Not a USB control message.
                }
            }
        }

        let direction : Direction;
        if fields.contains_key("usb.data_fragment")
        {
            offset = fields.get("usb.data_fragment").unwrap().start() as usize;
            length = fields.get("usb.data_fragment").unwrap().length() as usize;
            direction = Direction::HostToDevice;
        }
        else if fields.contains_key("usb.control_stage")
        {
            offset = (fields.get("usb.control_stage").unwrap().start() + 1) as usize;
            let usb_data_len = fields.get("usb.data_len").unwrap().value().get_uinteger() as usize;
            if offset >= usb_data_len
            {
                return tvb.reported_length(); // nothing to do here...
            }
            length = usb_data_len - offset;
            direction = Direction::DeviceToHost;
        }
        else
        {
            return tvb.reported_length(); // Nothing to do here, move along.
        }


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

        tvb.reported_length()
    }

    fn get_protocol_name(self: &Self) -> (&'static str, &'static str, &'static str) {
        return ("Dissector for Razer Huntsman Elite", "huntsman", "huntsman");
    }

    fn get_registration(self: &Self) -> Vec<dissector::Registration> {
        return vec![
            dissector::Registration::Post,
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
#[no_mangle]
pub fn plugin_register() {
    let z = Box::new(HuntsmanDissector::new());
    plugin::setup(z);
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
