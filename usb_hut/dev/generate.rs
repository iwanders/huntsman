#[derive(Debug,Clone)]
struct Key
{
    pub hid: usize,
    pub at101: Option<usize>,
    pub desc: String,
}

fn parse_file(fname: &str) -> Vec<Key>
{
    let mut res: Vec<Key> = Vec::new();

    let contents = std::fs::read_to_string(fname).expect("Failed to read the file.");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    for l in lines.iter()
    {
        if l.chars().nth(0) == Some('#')
        {
            continue;  // comment
        }
        if l.get(..3) == Some("---")
        {
            break;  // footnotes after this
        }
        if l.len() < 3
        {
            continue; // empty line.
        }
        // Ok, we got here, from here on we have;
        // 00-xx RangeName
        // xx Key description {footnote} DV|Sel #AT-101
        // footnote is optional, as is the AT-101 code
        let first_space = l.find(" ");
        if first_space != Some(2)
        {
            continue; // ignore ranges for now.
        }
        let first_space = first_space.unwrap();

        let DV = l.find("DV");
        let Sel = l.rfind("Sel");
        if DV == None && Sel == None
        {
            continue;
        }
        let usage = DV.unwrap_or(Sel.unwrap_or(0));
        let after_usage = usage + if DV.is_some() {"DV".len()} else {"Sel".len()};
        let HIDstr = &l[0..first_space];
        let parsed = usize::from_str_radix(HIDstr, 16);
        let hid: usize;
        if let Err(v) = parsed
        {
            println!("Failed to parse hid id: {:?}", v);
            continue;
        }
        else 
        {
            hid = parsed.unwrap();
        }

        let footnote_s = l.find("{");
        let footnote_e = l.find("}");
        let description_end;
        if footnote_s != None && footnote_e != None
        {
            description_end = std::cmp::min(footnote_s.unwrap_or(l.len()), usage);
        }
        else
        {
            description_end = usage;
        }
        
        let descr = &l[first_space+1..description_end - 1];

        // println!("{}", l);
        // finally, try to parse an AT-101 code.
        let at101 = usize::from_str_radix(&l[std::cmp::min(after_usage+1, l.len())..], 10);
        res.push(Key{
            hid,
            at101: at101.ok(),
            desc: descr.to_owned()
        });
        // println!("{:0>2x} '{}' {:?}", hid, descr, at101);
    }
    return res;
}

fn desc_to_name(desc: &str) -> String
{
    let tokens = desc.split(" ").collect::<Vec<&str>>();
    println!("{:?}", tokens);
    if tokens.len() < 2
    {
        panic!("Not enough tokens: {}", desc);
    }
    use std::collections::HashMap;
    let replaces: HashMap<&'static str, &'static str> = [("keyboard", "key"),
     ("/", "slash"),
     ("keypad", "kpd")].iter().cloned().collect();
    "kldsjflksdjf".to_owned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let keys = parse_file(&std::env::args().nth(1).unwrap());
    // println!("{:#?}", keys);
    for k in keys.iter()
    {
        if k.at101 == None
        {
            continue;
        }
        let n = desc_to_name(&k.desc);
        println!("{}", n);
    }

    Ok(())
}
