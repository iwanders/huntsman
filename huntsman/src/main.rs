extern crate huntsman_comm;
//~ extern crate huntsman;

use huntsman;

mod colors;
use colors::str_to_color;

//~ use clap::{Arg, App};
extern crate clap;
use clap::{App, Arg, SubCommand};

// There's probably a more proper way of doing this...
macro_rules! add_color_arg {
    ($thing: expr) => {
        $thing.arg(
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
    }
}

fn get_rgb(matches: &clap::ArgMatches) -> huntsman_comm::RGB
{
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
    
    huntsman_comm::RGB{r, g, b}
}


macro_rules! add_colors {
    ($thing: expr) => {
        $thing.arg(
            Arg::with_name("colors")
                    .multiple(true)
                    .takes_value(true)
                    .help("colors...")
            )
    }
}

fn get_colors(matches: &clap::ArgMatches) -> Vec<huntsman_comm::RGB>
{
    let mut res: Vec<huntsman_comm::RGB> = Vec::new();
    if let Some(z) = matches.values_of("colors")
    {
        // And now... we build whatever color parsing we need...
        for v in z
        {
            if let Some(c) = str_to_color(v)
            {
                res.push(c);
                continue;  // Cool, we got ourselves a color.
            }
            let v = v.replace("0x", "");
            if v.len() == 6
            {
                // Must be a hexadecimal color. Convert the thing.
                if let Ok(z) = u32::from_str_radix(&v, 16)
                {
                    let r = (z.checked_shr(16).expect("Cant fail?") & 0xFF) as u8;
                    let g = (z.checked_shr(8).expect("Cant fail?") & 0xFF) as u8;
                    let b = (z & 0xFF) as u8;
                    res.push(huntsman_comm::RGB{r, g, b});
                    continue;
                }
                else
                {
                    println!("Couldn't parse hex {} to u32", v);
                }
            }
            // Could add float notation here 0.5,1.0,0.25
            println!("No idea what to do with: {:#?}, use (0x)RRGGBB as hex, or color names.", v);
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
                    .help("The duration, either short, medium or long")
            )
    }
}

fn get_duration(matches: &clap::ArgMatches) -> Result<huntsman_comm::Duration, String>
{
    if let Some(v_in) = matches.value_of("duration")
    {
        return match v_in
        {
            "short" => Ok(huntsman_comm::Duration::Short),
            "medium" => Ok(huntsman_comm::Duration::Medium),
            "long" => Ok(huntsman_comm::Duration::Long),
            _ => Err(format!("No match for {}", v_in)),
        };
    }
    Err("Couldn't find argument".to_string())
}

fn get_value<T: core::str::FromStr>(matches: &clap::ArgMatches, name: &str) -> Result<T, String>
{
    if let Some(v_in) = matches.value_of(name)
    {
        if let Ok(v) = v_in.to_string().parse::<T>()
        {
            return Ok(v);
        }
    }
    
    Err(format!("Couldn't parse argument {}.", name))
}


pub fn main() -> Result<(), String> {
    let mut app = App::new("Huntsman Thing")
        .about("Does awesome things")
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
                        //~ .short("d")
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
        .subcommand(SubCommand::with_name("dev_run").about("Runs dev_run"))
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
                .arg(Arg::with_name("index").short("i").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("effect").about("Sets an LED effect").subcommand(
                SubCommand::with_name("off").about("Disables current effect")
            ).subcommand(
                add_color_arg!(SubCommand::with_name("fixed").about("Sets a fixed color."))
            ).subcommand(
                add_colors!(SubCommand::with_name("breathing").about("Breathes colors, no color is random."))
            ).subcommand(
                SubCommand::with_name("spectrum").about("Spectrum cycle.")
            ).subcommand(
                SubCommand::with_name("wave").about("Hue wave through the leds.").arg(
                    Arg::with_name("reverse")
                        .short("r")
                        .takes_value(false).help("Set to reverse the motion")
                ).arg(
                    Arg::with_name("delay")
                        .short("d")
                        .takes_value(true)
                        .default_value("100").help("Delay in ms between update cycles"),
                )
            ).subcommand(
                add_colors!(add_duration!(SubCommand::with_name("reactive").about("Colors keys hit, no color is random")))
            ).subcommand(
                add_colors!(SubCommand::with_name("ripple").about("Colors keys hit, no color is random"))
            )
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
    match matches.occurrences_of("c") {
        1 => {
            h.set_print_comm(true);
        }
        _ => {}
    }
    match matches.occurrences_of("r") {
        1 => {
            h.set_print_retrieve(true);
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

    if let Some(_matches) = matches.subcommand_matches("dev_run") {
        h.dev_run()?;
    }

    if let Some(matches) = matches.subcommand_matches("brightness") {
        let value_in = matches.value_of("value").expect("Value to be set");
        let value = value_in
            .to_string()
            .parse::<f32>()
            .expect("Parsing value as a float didn't work");
        h.set_brightness(value)?;
    }

    if let Some(matches) = matches.subcommand_matches("game_mode") {
        let value_in = matches.value_of("value").expect("Value to be set");
        let value = value_in
            .to_string()
            .parse::<bool>()
            .expect("Parsing value as a bool didn't work");
        h.set_game_mode(value)?;
    }

    if let Some(matches) = matches.subcommand_matches("effect") {
        println!("{:#?}", matches);
            match matches.subcommand_name() {
            Some("off") => {
                h.effect_off()?;
            },
            Some("fixed") => {
                let subargs = matches.subcommand_matches("fixed").unwrap();
                h.effect_fixed(&get_rgb(subargs))?;
            },
            Some("breathing") => {
                let subargs = matches.subcommand_matches("breathing").unwrap();
                let colors = get_colors(subargs);
                h.effect_breathing(&colors)?;
            },
            Some("spectrum") => {
                h.effect_spectrum()?;
            },
            Some("wave") => {
                let subargs = matches.subcommand_matches("wave").unwrap();
                let delay = get_value::<u8>(subargs, "delay")?;
                let reverse: bool = subargs.occurrences_of("reverse") == 0;
                println!("Reverse: {}", reverse);
                h.effect_wave(reverse, delay)?;
            },
            Some("reactive") => {
                let subargs = matches.subcommand_matches("reactive").unwrap();
                let colors = get_colors(subargs);
                let duration = get_duration(subargs)?;
                h.effect_reactive(duration, &colors)?;
            },
            Some("ripple") => {
                let subargs = matches.subcommand_matches("ripple").unwrap();
                let colors = get_colors(subargs);
                h.effect_ripple(&colors)?;
            },
            None => println!("No subcommand was used"),
            _ => println!("Some other subcommand was used"),
        }
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
