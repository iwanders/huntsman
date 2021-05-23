extern crate huntsman_comm;
//~ extern crate huntsman;

use huntsman;

//~ use clap::{Arg, App};
extern crate clap;
use clap::{App, Arg, SubCommand};

pub fn main() -> Result<(), String> {
    let mut app = App::new("Huntsman Thing")
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
            SubCommand::with_name("set_brightness")
                .about("Sets the brightness")
                .arg(
                    Arg::with_name("value")
                        //~ .short("d")
                        .takes_value(true)
                        .required(true)
                        .help("The brightness to set as a float [0, 1.0] inclusive."),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_game_mode")
                .about("Sets game mode on or off")
                .arg(
                    Arg::with_name("value")
                        .takes_value(true)
                        .required(true)
                        .help("The state to set it to."),
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
                    Arg::with_name("start")
                        .short("s")
                        .takes_value(true)
                        .default_value("0"),
                )
                .arg(Arg::with_name("index").short("i").takes_value(true))
        );
    let matches = app.clone().get_matches();  // weird that get_matches() takes 'self', instead of &self

    // Abort with the help if no subcommand is given.
    match matches.subcommand()
    {
        (_something, Some(_subcmd)) => {},
        _ => {  &mut app.print_help();
                println!();
                return Err("No subcommand given.".to_string());
            }
    }


    // We have a subcommand, try to make the Huntsman object.
    let mut h = huntsman::Huntsman::new()?;

    // Set the print communication flag.
    match matches.occurrences_of("c") {
        1 => {
            h.set_print_comm(true);
        }
        _ => {}
    }

    if let Some(matches) = matches.subcommand_matches("flashy_thing") {
        let delay_in = matches.value_of("delay").expect("Delay be set");
        let delay = delay_in
            .to_string()
            .parse::<u64>()
            .expect("Parsing delay as a number didn't work");
        h.do_flashy_things(delay)?;
    }

    if let Some(matches) = matches.subcommand_matches("set_brightness") {
        let value_in = matches.value_of("value").expect("Value to be set");
        let value = value_in
            .to_string()
            .parse::<f32>()
            .expect("Parsing value as a float didn't work");
        h.set_brightness(value)?;
    }
    if let Some(matches) = matches.subcommand_matches("set_game_mode") {
        let value_in = matches.value_of("value").expect("Value to be set");
        let value = value_in
            .to_string()
            .parse::<bool>()
            .expect("Parsing value as a bool didn't work");
        h.set_game_mode(value)?;
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
                let index = v
                    .to_string()
                    .parse::<u8>()
                    .expect("Failed to parse index as number");
                let count_in = matches.value_of("count").expect("count be set");
                let count = count_in
                    .to_string()
                    .parse::<u8>()
                    .expect("Parsing count as a number didn't work");
                let start_in = matches.value_of("start").expect("start be set");
                let start = start_in
                    .to_string()
                    .parse::<u8>()
                    .expect("Parsing start as a number didn't work");
                return h.set_color_single(r, g, b, count, index, start);
            }
            None => {
                return h.set_color(r, g, b);
            }
        }
    }

    return Ok(());
}
