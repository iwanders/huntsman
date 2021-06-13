extern crate huntsman_comm;
//~ extern crate huntsman;

use huntsman;

mod colors;
use colors::str_to_color;

extern crate clap;
use clap::{App, Arg, SubCommand};

macro_rules! add_colors {
    ($thing: expr) => {
        $thing.arg(
            Arg::with_name("colors")
                .multiple(true)
                .takes_value(true)
                .help("colors..."),
        )
    };
}

fn get_colors(matches: &clap::ArgMatches) -> Vec<huntsman_comm::RGB> {
    let mut res: Vec<huntsman_comm::RGB> = Vec::new();
    if let Some(z) = matches.values_of("colors") {
        // And now... we build whatever color parsing we need...
        for v in z {
            if let Some(c) = str_to_color(v) {
                res.push(c);
                continue; // Cool, we got ourselves a color.
            }
            let v = v.replace("0x", "");
            if v.len() == 6 {
                // Must be a hexadecimal color. Convert the thing.
                if let Ok(z) = u32::from_str_radix(&v, 16) {
                    let r = (z.checked_shr(16).expect("Cant fail?") & 0xFF) as u8;
                    let g = (z.checked_shr(8).expect("Cant fail?") & 0xFF) as u8;
                    let b = (z & 0xFF) as u8;
                    res.push(huntsman_comm::RGB { r, g, b });
                    continue;
                } else {
                    println!("Couldn't parse hex {} to u32", v);
                }
            }
            // Could add float notation here 0.5,1.0,0.25
            println!(
                "No idea what to do with: {:#?}, use (0x)RRGGBB as hex, or color names.",
                v
            );
        }
    }
    res
}

macro_rules! add_duration {
    ($thing: expr) => {
        $thing.arg(
            Arg::with_name("duration")
                .short("d")
                .takes_value(true)
                .default_value("medium")
                .possible_values(&["short", "medium", "long"])
                .help("The duration, either short, medium or long"),
        )
    };
}

fn get_duration(matches: &clap::ArgMatches) -> Result<huntsman_comm::Duration, String> {
    if let Some(v_in) = matches.value_of("duration") {
        return match v_in {
            "short" => Ok(huntsman_comm::Duration::Short),
            "medium" => Ok(huntsman_comm::Duration::Medium),
            "long" => Ok(huntsman_comm::Duration::Long),
            _ => Err(format!("No match for {}", v_in)),
        };
    }
    Err("Couldn't find argument".to_string())
}

fn get_value<T: core::str::FromStr>(matches: &clap::ArgMatches, name: &str) -> Result<T, String> {
    if let Some(v_in) = matches.value_of(name) {
        if let Ok(v) = v_in.to_string().parse::<T>() {
            return Ok(v);
        }
    }

    Err(format!("Couldn't parse argument {}.", name))
}

pub fn main() -> Result<(), String> {
    let mut app = App::new("Huntsman toolie")
        .about("Allows configuring the keyboard in various ways.")
        .arg(
            Arg::with_name("c")
                .short("c")
                .help("Specifies whether to print comms."),
        )
        .arg(
            Arg::with_name("r")
                .short("r")
                .help("Specifies whether to retrieve the report after sending any command.."),
        )
        .subcommand(
            SubCommand::with_name("brightness")
                .about("Sets the brightness")
                .arg(
                    Arg::with_name("value")
                        .takes_value(true)
                        .required(true)
                        .help("The brightness to set as a float [0, 1.0] inclusive."),
                ),
        )
        .subcommand(
            SubCommand::with_name("game_mode")
                .about("Sets game mode on or off")
                .arg(
                    Arg::with_name("value")
                        .takes_value(true)
                        .required(true)
                        .help("The state to set it to."),
                ),
        )
        .subcommand(SubCommand::with_name("serial_number").about("Retrieves the serial number"))
        .subcommand(SubCommand::with_name("dev_run").about("Runs dev_run"))
        .subcommand(SubCommand::with_name("dev_dump_keymaps").about("Dumps all keymappings."))
        .subcommand(add_colors!(SubCommand::with_name("set_color")
            .about("Sets colors in the custom frame.")
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
            .arg(Arg::with_name("index").short("i").takes_value(true))))
        .subcommand(
            SubCommand::with_name("effect")
                .about("Sets an LED effect")
                .subcommand(SubCommand::with_name("off").about("Disables current effect"))
                .subcommand(add_colors!(
                    SubCommand::with_name("fixed").about("Sets a fixed color.")
                ))
                .subcommand(add_colors!(SubCommand::with_name("breathing")
                    .about("Breathes colors, no color is random.")))
                .subcommand(SubCommand::with_name("spectrum").about("Spectrum cycle."))
                .subcommand(
                    SubCommand::with_name("wave")
                        .about("Hue wave through the leds.")
                        .arg(
                            Arg::with_name("reverse")
                                .short("r")
                                .takes_value(false)
                                .help("Set to reverse the motion"),
                        )
                        .arg(
                            Arg::with_name("delay")
                                .short("d")
                                .takes_value(true)
                                .default_value("100")
                                .help("Delay in ms between update cycles"),
                        ),
                )
                .subcommand(add_colors!(add_duration!(SubCommand::with_name(
                    "reactive"
                )
                .about("Colors keystrokes, no color is random"))))
                .subcommand(add_colors!(
                    SubCommand::with_name("ripple").about("Colors keys hit, no color is random")
                ))
                .subcommand(add_colors!(add_duration!(SubCommand::with_name(
                    "starlight"
                )
                .about("Colors random keys, no color is random"))))
                .subcommand(
                    SubCommand::with_name("custom").about("No effect, use frame from SetLedState"),
                ),
        );

    let matches = app.clone().get_matches(); // weird that get_matches() takes 'self', instead of &self

    // Abort with the help if no subcommand is given.
    match matches.subcommand() {
        (_something, Some(_subcmd)) => {}
        _ => {
            &mut app.print_help();
            println!();
            return Err("No subcommand given.".to_string());
        }
    }

    // We have a subcommand, try to make the Huntsman object.
    let mut h = huntsman::Huntsman::new()?;

    // Set the print communication flag.
    h.set_print_comm(matches.occurrences_of("c") == 1);
    h.set_print_retrieve(matches.occurrences_of("r") == 1);

    // Next, follows the invidual subcommand handling.

    if let Some(_matches) = matches.subcommand_matches("dev_run") {
        h.dev_run()?;
    }

    if let Some(_matches) = matches.subcommand_matches("dev_dump_keymaps") {
        h.dev_dump_keymaps()?;
    }
    if let Some(_matches) = matches.subcommand_matches("serial_number") {
        h.get_serial_number()?;
    }

    if let Some(matches) = matches.subcommand_matches("brightness") {
        let value = get_value::<f32>(matches, "value")?;
        h.set_brightness(value)?;
    }

    if let Some(matches) = matches.subcommand_matches("game_mode") {
        let value = get_value::<bool>(matches, "value")?;
        h.set_game_mode(value)?;
    }

    if let Some(matches) = matches.subcommand_matches("effect") {
        match matches.subcommand_name() {
            Some("off") => {
                h.effect_off()?;
            }
            Some("fixed") => {
                let subargs = matches.subcommand_matches("fixed").unwrap();
                let colors = get_colors(subargs);
                h.effect_fixed(&colors[0])?;
            }
            Some("breathing") => {
                let subargs = matches.subcommand_matches("breathing").unwrap();
                let colors = get_colors(subargs);
                h.effect_breathing(&colors)?;
            }
            Some("spectrum") => {
                h.effect_spectrum()?;
            }
            Some("wave") => {
                let subargs = matches.subcommand_matches("wave").unwrap();
                let delay = get_value::<u8>(subargs, "delay")?;
                let reverse: bool = subargs.occurrences_of("reverse") == 0;
                println!("Reverse: {}", reverse);
                h.effect_wave(reverse, delay)?;
            }
            Some("reactive") => {
                let subargs = matches.subcommand_matches("reactive").unwrap();
                let colors = get_colors(subargs);
                let duration = get_duration(subargs)?;
                h.effect_reactive(duration, &colors)?;
            }
            Some("ripple") => {
                let subargs = matches.subcommand_matches("ripple").unwrap();
                let colors = get_colors(subargs);
                h.effect_ripple(&colors)?;
            }
            Some("starlight") => {
                let subargs = matches.subcommand_matches("starlight").unwrap();
                let colors = get_colors(subargs);
                let duration = get_duration(subargs)?;
                h.effect_starlight(duration, &colors)?;
            }
            Some("custom") => {
                h.effect_custom()?;
            }
            None => println!("No subcommand was used"),
            _ => println!("Some other subcommand was used"),
        }
    }

    if let Some(matches) = matches.subcommand_matches("set_color") {
        let colors = get_colors(matches);
        let color = colors[0];
        let index = get_value::<u8>(matches, "index")?;
        let count = get_value::<u8>(matches, "count")?;
        let start = get_value::<u8>(matches, "start")?;
        h.set_color_single(&color, count, index, start)?
    }

    return Ok(());
}
