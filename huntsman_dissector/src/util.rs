// This is mostly data wrangling, converting struct helper fields to PacketFields and some
// helpers to make names and ids.

use wireshark_dissector_rs::dissector;

extern crate huntsman_comm;
use huntsman_comm::wire;
extern crate struct_helper;
use struct_helper::StructHelper;

type FieldType = dissector::FieldType;
type FieldDisplay = dissector::FieldDisplay;

/// Flags applied through the struct helper on fields.
#[derive(Default, Debug, Clone, Copy)]
pub struct FieldFlags {
    pub hidden: bool,
}

/// Internal data structur we use to represent a dissection field.
#[derive(Default, Debug, Clone)]
pub struct DissectionField {
    start: usize,
    length: usize,
    abbrev: Vec<Prefix>,
    flags: FieldFlags,
    type_name: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Enum to denote the location to the visitor
pub enum Location {
    MultipleChildrenStart,
    MultipleChildrenEnd,
    Leaf,
}

/// Enum to hold an index or string, these elements in a vector make up the name for elements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Prefix {
    Label(String),
    Index(usize),
}

pub type Visitor<'a> =
    &'a mut dyn FnMut(Location, &struct_helper::Field, &Vec<Prefix>, &FieldFlags, usize) -> ();

/// Worker function to traverse the tree of [`struct_helper::Field`], calling a visitor at certain
/// locations of interest.
pub fn field_recurser(
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
            c.info.start + offset,
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
pub fn make_field_abbrev(v: &Vec<Prefix>) -> String {
    v.iter()
        .filter_map(|x| match x {
            Prefix::Label(s) => Some(s.clone()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join(".")
}

/// Make an arbitrary label out of the current vector, including all integers.
pub fn make_fold_label(v: &Vec<Prefix>) -> String {
    v.iter()
        .map(|x| match x {
            Prefix::Label(s) => s.clone(),
            Prefix::Index(i) => i.to_string(),
        })
        .collect::<Vec<String>>()
        .join(".")
}

/// Namespace for all our prefixes / abbreviations.
pub fn prefix_start() -> Vec<Prefix> {
    vec![Prefix::Label("huntsman".to_string())]
}

const LABEL_STR: &'static str = "_LABEL_";

/// Make a fold label from a current prefix.
pub fn make_fold_item_label(v: &Vec<Prefix>) -> Vec<Prefix> {
    let mut label_prefix = v.clone();
    label_prefix.push(fold_item_label());
    label_prefix
}

/// Make the prefix we apply at the end for labels.
pub fn fold_item_label() -> Prefix {
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
pub fn fields_to_dissector(v: &Vec<DissectionField>) -> Vec<dissector::PacketField> {
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
pub fn make_all_fields() -> (Vec<DissectionField>, Vec<String>) {
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
