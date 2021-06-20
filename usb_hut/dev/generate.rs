#[derive(Debug, Clone, Copy)]
enum Usage {
    Selector,
    DynamicFlag,
}

#[derive(Debug, Clone)]
struct Key {
    pub hid: usize,
    pub at101: Option<usize>,
    pub desc: String,
    pub usage: Usage,
}

fn parse_file(fname: &str) -> Vec<Key> {
    let mut res: Vec<Key> = Vec::new();

    let contents = std::fs::read_to_string(fname).expect("Failed to read the file.");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    for l in lines.iter() {
        if l.chars().nth(0) == Some('#') {
            continue; // comment
        }
        if l.get(..3) == Some("---") {
            break; // footnotes after this
        }
        if l.len() < 3 {
            continue; // empty line.
        }
        // Ok, we got here, from here on we have;
        // 00-xx RangeName
        // xx Key description {footnote} DV|Sel #AT-101
        // footnote is optional, as is the AT-101 code
        let first_space = l.find(" ");
        if first_space != Some(2) {
            continue; // ignore ranges for now.
        }
        let first_space = first_space.unwrap();

        let dv_usage = l.find("DV");
        let sel_usage = l.rfind("Sel");
        if dv_usage == None && sel_usage == None {
            continue;
        }
        let usage = dv_usage.unwrap_or(sel_usage.unwrap_or(0));
        let after_usage = usage
            + if dv_usage.is_some() {
                "DV".len()
            } else {
                "Sel".len()
            };
        let hid_id_str = &l[0..first_space];
        let parsed = usize::from_str_radix(hid_id_str, 16);
        let hid: usize;
        if let Err(v) = parsed {
            println!("Failed to parse hid id: {:?}", v);
            continue;
        } else {
            hid = parsed.unwrap();
        }

        let footnote_s = l.find("{");
        let footnote_e = l.find("}");
        let description_end;
        if footnote_s != None && footnote_e != None {
            description_end = std::cmp::min(footnote_s.unwrap_or(l.len()), usage);
        } else {
            description_end = usage;
        }

        let descr = &l[first_space + 1..description_end - 1];

        // println!("{}", l);
        // finally, try to parse an AT-101 code.
        let at101 = usize::from_str_radix(&l[std::cmp::min(after_usage + 1, l.len())..], 10);
        res.push(Key {
            hid,
            at101: at101.ok(),
            desc: descr.to_owned(),
            usage: if dv_usage.is_some() {
                Usage::DynamicFlag
            } else {
                Usage::Selector
            },
        });
        // println!("{:0>2x} '{}' {:?}", hid, descr, at101);
    }
    return res;
}

fn desc_to_name(desc: &str) -> String {
    let tokens = desc.split(" ").collect::<Vec<&str>>();
    // println!("{:?}", tokens);
    if tokens.len() < 2 {
        panic!("Not enough tokens: {}", desc);
    }
    use std::collections::HashMap;
    let replaces: HashMap<&'static str, &'static str> = [
        ("keyboard", "key"),
        ("/", "slash"),
        ("\\", "backslash"),
        (".", "dot"),
        ("escape", "esc"),
        ("spacebar", "space"),
        ("-", "dash"),
        (";", "semicolon"),
        ("=", "equal"),
        ("[", "left_bracket"),
        ("]", "right_bracket"),
        ("(", "left_parenthesis"),
        (")", "right_parenthesis"),
        ("{", "left_brace"),
        ("}", "right_brace"),
        (",", "comma"),
        ("‘", "quote"),
        ("^", "caret"),
        ("+", "plus"),
        ("*", "asterisk"),
        ("<", "lt"),
        (">", "gt"),
        ("&", "ampersand"),
        ("&&", "doubleampersand"),
        ("@", "at"),
        ("!", "exclamation"),
        ("|", "pipe"),
        ("||", "doublepipe"),
        ("gui", "meta"),
        ("#", "hash"),
        (":", "colon"),
        ("%", "pct"),
        ("+/-", "plusminus"),
        ("sub-unit", "subunit"),
        ("keypad", "kpd"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut name_tokens: Vec<String> = Vec::new();
    for i in 0..tokens.len() {
        let token = tokens[i];
        let lowercase_token = token.to_ascii_lowercase();

        if i == 2 && lowercase_token == "and" {
            break;
        }

        let mut new_token;
        if let Some(r) = replaces.get(&lowercase_token.as_str()) {
            new_token = r.to_string();
        } else {
            new_token = token.to_string();
        }

        new_token = new_token.replace("\\", "backslash_");
        new_token = new_token.replace("-", "_");
        new_token = new_token.replace(",", "comma");
        new_token = new_token.replace("/", "_");

        name_tokens.push(new_token);
    }

    match name_tokens.get(0).unwrap().as_str() {
        "key" => {}
        "kpd" => {
            name_tokens.insert(0, "key".to_string());
        }
        _ => {
            name_tokens.insert(0, "key".to_string());
        }
    }

    name_tokens
        .join("_")
        .replace("Return_(ENTER)", "enter")
        .replace("DELETE_(Backspace)", "backspace")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().len() < 2
    {
        println!("use like: cargo run --bin generate -- hut1_22_pdf_keyboard_page_0x07.txt src/generated.rs");
        std::process::exit(1);
    }
    let keys = parse_file(&std::env::args().nth(1).unwrap());
    // println!("{:#?}", keys);
    let mut keydefs: Vec<String> = vec![];
    let mut names: Vec<String> = vec![];
    for k in keys.iter() {
        // if k.at101 == None
        // {
        // continue;
        // }
        let n = desc_to_name(&k.desc);
        println!("{}", n);
        keydefs.push(format!(
            "
    pub const {}: Key = Key {{
        hid: 0x{:0>2x},
        at101: {:?},
        usage: Usage::{:?},
        desc: \"{}\",
    }};",
            n.to_ascii_uppercase(),
            k.hid,
            k.at101,
            k.usage,
            k.desc.replace("\\", "\\\\")
        ));
        names.push(format!("{}", n.to_ascii_uppercase()));
    }

    if std::env::args().len() < 3 {
        println!("{}", keydefs.join("\n"));

        println!("{}", names.join(",\n"));
    } else {
        use std::io::Write;
        let mut file = std::fs::File::create(&std::env::args().nth(2).unwrap())?;
        let fcontent = format!(
            "#[allow(dead_code)]
// This file is generated with dev/generate.rs
use crate::defs::Key;

// keys:
pub mod hid_keys {{
    #[allow(dead_code)]
    use crate::defs::{{Key, Usage}};
{}
}}

pub const fn keys() -> [Key; {}] {{
    [
{},
    ]
}}
",
            keydefs.join("\n"),
            names.len(),
            names
                .iter()
                .map(|x| format!("        hid_keys::{}", x))
                .collect::<Vec<String>>()
                .join(",\n")
        );
        file.write_all(fcontent.as_str().as_bytes())?;
    }

    Ok(())
}
