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

    return Ok(());
}
