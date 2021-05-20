extern crate huntsman_comm;
//~ extern crate huntsman;

use huntsman;

//~ use clap::{Arg, App};
extern crate clap;
use clap::{App, Arg, SubCommand};

pub fn main() -> Result<(), String> {
    let matches = App::new("Huntsman Thing")
        .about("Does awesome things")
        .arg(
            Arg::with_name("c")
                .short("c")
                .help("Specifies whether to print comms."),
        )
        .subcommand(
            SubCommand::with_name("flashy_thing")
                .about("controls testing features")
                .arg(
                    Arg::with_name("delay")
                        .short("d")
                        .takes_value(true)
                        .default_value("100")
                        .help("delay duration between things."),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_color")
                .arg(
                    Arg::with_name("red")
                        .short("r")
                        .takes_value(true)
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("green")
                        .short("g")
                        .takes_value(true)
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("blue")
                        .short("b")
                        .takes_value(true)
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("count")
                        .short("c")
                        .takes_value(true)
                        .default_value("22"),
                )
                .arg(
                    Arg::with_name("index")
                        .short("i")
                        .takes_value(true)
                )
                
        )
        .get_matches();

    match matches.occurrences_of("c") {
        1 => {}
        _ => {}
    }

    let mut h = huntsman::Huntsman::new()?;

    if let Some(matches) = matches.subcommand_matches("flashy_thing") {
        let delay_in = matches.value_of("delay").expect("Delay be set");
        let delay = delay_in
            .to_string()
            .parse::<u64>()
            .expect("Parsing delay as a number didn't work");
        h.do_flashy_things(delay);
    }

    if let Some(matches) = matches.subcommand_matches("set_color") {
        let r_in = matches.value_of("red").expect("red be set");
        let r = r_in
            .to_string()
            .parse::<u8>()
            .expect("Parsing r as a number didn't work");
        let g_in = matches.value_of("green").expect("green be set");
        let g = g_in
            .to_string()
            .parse::<u8>()
            .expect("Parsing g as a number didn't work");
        let b_in = matches.value_of("blue").expect("blue be set");
        let b = b_in
            .to_string()
            .parse::<u8>()
            .expect("Parsing b as a number didn't work");

        match matches.value_of("index") {
            Some(v) => {
                let index = v.to_string().parse::<u8>().expect("Failed to parse index as number");
                let count_in = matches.value_of("count").expect("count be set");
                let count = count_in
                    .to_string()
                    .parse::<u8>()
                    .expect("Parsing count as a number didn't work");
                return h.set_color_single(r, g, b, count + 1, index);
            },
            None => {return  h.set_color(r, g, b);},
        }

        
    }

    return Ok(());
}
