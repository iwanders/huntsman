use huntsman;
use huntsman::commands;

mod colors;
use colors::str_to_color;

mod profile_util;

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

type Error = Box<dyn std::error::Error>;

fn get_colors(matches: &clap::ArgMatches) -> Vec<commands::RGB> {
    let mut res: Vec<commands::RGB> = Vec::new();
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
                    res.push(commands::RGB { r, g, b });
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

fn get_duration(matches: &clap::ArgMatches) -> Result<commands::Duration, Error> {
    if let Some(v_in) = matches.value_of("duration") {
        return match v_in {
            "short" => Ok(commands::Duration::Short),
            "medium" => Ok(commands::Duration::Medium),
            "long" => Ok(commands::Duration::Long),
            _ => Err(Box::new(clap::Error::with_description(
                "Invalid duration",
                clap::ErrorKind::InvalidValue,
            ))),
        };
    }
    Err(Box::new(clap::Error::with_description(
        "Couldn't find duration.",
        clap::ErrorKind::EmptyValue,
    )))
}

fn get_value<T: core::str::FromStr>(matches: &clap::ArgMatches, name: &str) -> Result<T, String> {
    if let Some(v_in) = matches.value_of(name) {
        if let Ok(v) = v_in.to_string().parse::<T>() {
            return Ok(v);
        }
    }

    Err(format!("Couldn't parse argument {}.", name))
}

/// Parse a value from the commandline, interpreting as hex if the value starts with 0x
fn get_numeric_u64(matches: &clap::ArgMatches, name: &str) -> Result<u64, String> {
    if let Some(v_in) = matches.value_of(name) {
        if v_in.starts_with("0x") {
            let v = v_in.replace("0x", "");
            if let Ok(v) = u64::from_str_radix(&v, 16) {
                return Ok(v);
            }
        }
        if let Ok(v) = v_in.to_string().parse::<u64>() {
            return Ok(v);
        }
    }

    Err(format!("Couldn't parse argument {}.", name))
}

fn get_profile_id(matches: &clap::ArgMatches) -> Result<commands::ProfileId, String> {
    if let Some(v_in) = matches.value_of("profile_id") {
        return Ok(profile_util::str_to_profile_id(&v_in.to_string()));
    }
    Err(format!("Couldn't parse argument profile_id."))
}

pub fn main() -> Result<(), Error> {
    let mut app = App::new("Huntsman toolie").setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .about("Allows configuring the keyboard in various ways.")
        .arg(
            Arg::with_name("c")
                .short("c")
                .help("Specifies whether to print outgoing commands."),
        )
        .arg(
            Arg::with_name("r")
                .short("r")
                .help("Specifies whether to print the reply sending any command."),
        )
        .arg(
            Arg::with_name("d")
                .short("d")
                .help("Dry run, don't actually connect to the device, echo all commands."),
        )
        .subcommand(
            SubCommand::with_name("brightness")
                .about("Sets the brightness")
                .arg(
                    Arg::with_name("value")
                        .takes_value(true)
                        .required(true)
                        .help("The brightness to set as a float [0, 1.0] inclusive."),
                )
                .arg(
                    Arg::with_name("profile")
                        .short("p")
                        .takes_value(true)
                        .default_value("1")
                        .help("The profile to use."),
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
            SubCommand::with_name("effect").setting(clap::AppSettings::SubcommandRequiredElseHelp)
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
        )
        .subcommand(
            SubCommand::with_name("macro").setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .about("Configuration for macros")
                .subcommand(SubCommand::with_name("list").about("List macros on the device"))
                .subcommand(
                    SubCommand::with_name("count").about("Show the nr of macros on the device."),
                )
                .subcommand(
                    SubCommand::with_name("del")
                        .about("Remove a macro from the device")
                        .arg(
                            Arg::with_name("macro_id")
                                .takes_value(true)
                                .required(true)
                                .help("The macro_id to remove."),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("load")
                        .about("Load a macro from file to device")
                        .arg(
                            Arg::with_name("file")
                                .takes_value(true)
                                .required(true)
                                .help("The filename to read the macro from."),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("profile").setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .about(
                    "Configuration for profiles, profiles 2, 3, 4 and 5 can be removed / created.",
                )
                .subcommand(SubCommand::with_name("list").about("List profiles on the device"))
                .subcommand(SubCommand::with_name("current").about("Show the currently active profile."))
                .subcommand(
                    SubCommand::with_name("count").about("Show the nr of profiles on the device."),
                )
                .subcommand(
                    SubCommand::with_name("del")
                        .about("Remove a profile from the device")
                        .arg(
                            Arg::with_name("profile_id")
                                .takes_value(true)
                                .required(true)
                                .help("The profile_id to remove, must exist (or by color)."),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("create")
                        .about("Create a profile by id")
                        .arg(
                            Arg::with_name("profile_id")
                                .takes_value(true)
                                .required(true)
                                .help("The profile_id to allocate 2, 3, 4 or 5 (or by color)."),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("activate")
                        .about("Activate (switch to) a profile by id")
                        .arg(
                            Arg::with_name("profile_id")
                                .takes_value(true)
                                .required(true)
                                .help("The profile_id to set the keyboard to 1, 2, 3, 4 or 5 (or by color)."),
                        ),
                ),
        ).subcommand(
            SubCommand::with_name("mapping").setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .about(
                    "Modify the key mappings",
                ).subcommand(
                SubCommand::with_name("load")
                    .about("Load mapping(s) from yaml")
                    .arg(
                        Arg::with_name("file")
                            .takes_value(true)
                            .required(true)
                            .help("The filename to read the mappings from."),
                    )
                    .arg(
                        Arg::with_name("profile")
                            .takes_value(true)
                            .required(true)
                            .help("The profile to use."),
                    ),
                )
                .subcommand(
                    SubCommand::with_name("retrieve")
                        .about("Dumps all keymappings for a profile. key_name(:hypershift) at101 -> mapping")
                        .arg(
                            Arg::with_name("hypershift")
                                .short("s")
                                .help("Retrieve hypershift commands or not."),
                        )
                        .arg(
                            Arg::with_name("showall")
                                .short("a")
                                .help("Show all retrievals, or only modified from default."),
                        )
                        .arg(
                            Arg::with_name("profile_id")
                                .takes_value(true)
                                .required(false)
                                .default_value("all")
                                .help("The profile id to retrieve. May be 'all' to retrieve all profiles."),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("set")
                        .about("Set a single key mapping")
                        .arg(
                            Arg::with_name("hypershift")
                                .short("s")
                                .help("Retrieve hypershift commands or not."),
                        )
                        .arg(
                            Arg::with_name("profile_id")
                                .takes_value(true)
                                .required(true)
                                .help("The profile id to retrieve."),
                        )
                        .arg(
                            Arg::with_name("key")
                                .required(true)
                                .help("The key to set. (example: `a`, `3`, `left_alt`)"),
                        )
                        .arg(
                            Arg::with_name("mapping")
                                .default_value("default")
                                .help("The mapping to set, leave empty for default key value.
disabled - disables this key
default - reverts this key to default
some_other_key_identifier - use this key's default value.
\"key: { id: a , modifiers:[alt] }\" - use alt 'a' as this key's mapping.
\"macro: {macro_id: 0x1337, count: 1}\" - Map this key to a macro
... - and so on for all other mappings.
"),
                        ),
                )
    );

    let matches = app.clone().get_matches(); // weird that get_matches() takes 'self', instead of &self

    // Abort with the help if no subcommand is given.
    match matches.subcommand() {
        (_something, Some(_subcmd)) => {}
        _ => {
            &mut app.print_help();
            println!();
            return Err(Box::new(clap::Error::with_description(
                "No subcommand given",
                clap::ErrorKind::MissingSubcommand,
            )));
        }
    }

    let dry_run = matches.occurrences_of("d") == 1;

    let mut h: huntsman::Huntsman;
    if dry_run {
        h = huntsman::Huntsman::dry_new()?;
        // h = huntsman::Huntsman::new()?;
    } else {
        // We have a subcommand, try to make the Huntsman object.
        h = huntsman::Huntsman::new()?;
    }

    // Set the print communication flag.
    h.set_print_comm(matches.occurrences_of("c") == 1);
    h.set_print_retrieve(matches.occurrences_of("r") == 1);

    // Next, follows the invidual subcommand handling.

    if let Some(_matches) = matches.subcommand_matches("dev_run") {
        h.dev_run()?;
    }

    if let Some(_matches) = matches.subcommand_matches("serial_number") {
        h.get_serial_number()?;
    }

    if let Some(matches) = matches.subcommand_matches("brightness") {
        let value = get_value::<f32>(matches, "value")?;
        let profile = get_value::<u8>(matches, "profile")?;
        h.set_brightness(profile, value)?;
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

    if let Some(matches) = matches.subcommand_matches("macro") {
        match matches.subcommand_name() {
            Some("list") => {
                let ids = h.macro_list()?;
                if ids.len() != 0 {
                    println!("Macro's in memory:");
                    for id in ids.iter() {
                        println!("   - 0x{:0>4x}", id);
                    }
                } else {
                    println!("No macro's in memory.");
                }
            }
            Some("count") => {
                let count = h.macro_count()?;
                println!("{}", count);
            }
            Some("load") => {
                let submatches = matches.subcommand_matches("load").unwrap();
                let file = get_value::<String>(submatches, "file")?;
                let macro_config =
                    huntsman::configuration::load_macro(&file).map_err(|x| format!("{:?}", x))?;
                println!("Loading macro:\n{:?}", macro_config);
                h.macro_create_actions(macro_config.macro_id, &macro_config.events)?;
                println!("Macro 0x{:0>4x} succesfully loaded.", macro_config.macro_id);
            }
            Some("del") => {
                let submatches = matches.subcommand_matches("del").unwrap();
                let macro_id = get_numeric_u64(submatches, "macro_id")? as u16;
                let ids = h.macro_list()?;
                for id in ids.iter() {
                    if *id == macro_id {
                        // found the thing, remove it.
                        h.macro_delete(macro_id)?;
                        println!("Macro 0x{:0>4x} removed.", macro_id);
                        return Ok(());
                    }
                }
                println!(
                    "The device doesn't report macro 0x{:0>4x} is present.",
                    macro_id
                );
            }
            None => println!("No subcommand was used"),
            _ => println!("Some other subcommand was used"),
        }
    }

    if let Some(matches) = matches.subcommand_matches("profile") {
        match matches.subcommand_name() {
            Some("list") => {
                let ids = h.profile_list()?;
                if ids.len() != 0 {
                    println!("Profiles's in memory:");
                    for id in ids.iter() {
                        println!(
                            "   - {} ({})",
                            id,
                            profile_util::profile_to_colored_name(*id)
                        );
                    }
                } else {
                    println!("No macro's in memory.");
                }
            }
            Some("count") => {
                let count = h.profile_count()?;
                println!("{}", count);
            }
            Some("current") => {
                let id = h.profile_get_current()?;
                println!(
                    "Current profile: {} ({})",
                    id,
                    profile_util::profile_to_colored_name(id)
                );
            }
            Some("activate") => {
                let submatches = matches.subcommand_matches("activate").unwrap();
                let profile_id = get_profile_id(submatches)?;
                let _ = h.profile_set_current(profile_id)?;
                let id = h.profile_get_current()?;
                println!(
                    "Current profile: {} ({})",
                    id,
                    profile_util::profile_to_colored_name(id)
                );
            }
            Some("create") => {
                let submatches = matches.subcommand_matches("create").unwrap();
                let profile_id = get_profile_id(submatches)?;
                h.profile_create(profile_id)?;
                println!(
                    "Profile {} ({}) created.",
                    profile_id,
                    profile_util::profile_to_colored_name(profile_id)
                );
            }
            Some("del") => {
                let submatches = matches.subcommand_matches("del").unwrap();
                let profile_id = get_profile_id(submatches)?;
                let ids = h.profile_list()?;
                for id in ids.iter() {
                    if *id == profile_id {
                        // found the thing, remove it.
                        h.profile_delete(profile_id)?;
                        println!(
                            "Profile {} ({}) removed.",
                            profile_id,
                            profile_util::profile_to_colored_name(*id)
                        );
                        return Ok(());
                    }
                }
                println!(
                    "The device doesn't report profile {} is present.",
                    profile_id
                );
            }
            _ => println!("Some other subcommand was used"),
        }
    }

    if let Some(matches) = matches.subcommand_matches("mapping") {
        match matches.subcommand_name() {
            Some("load") => {
                let submatches = matches.subcommand_matches("load").unwrap();
                let file = get_value::<String>(submatches, "file")?;
                let profile = get_profile_id(submatches)?;
                println!("println!  {}, {}", file, profile);
                let mappings = huntsman::configuration::load_mappings(&file)
                    .map_err(|x| format!("{:?}", x))?;
                println!("{:?}", mappings);
                for m in mappings.iter() {
                    // build the actual configuration.
                    h.set_mapping(profile, m.key, m.mapping)?;
                }
            }
            Some("retrieve") => {
                let submatches = matches.subcommand_matches("retrieve").unwrap();
                let hypershift = submatches.occurrences_of("hypershift") == 1;
                let show_all = submatches.occurrences_of("showall") == 1;
                let mut profiles: Vec<u8> = vec![];

                if let Some(v_in) = submatches.value_of("profile_id") {
                    if v_in == "all" {
                        profiles.extend(&[1, 2, 3, 4, 5]);
                    } else {
                        let profile = get_profile_id(submatches)?;
                        profiles.push(profile);
                    }
                }
                println!();

                for profile in profiles.iter() {
                    println!(
                        "Profile {} ({}) mappings:",
                        profile,
                        profile_util::profile_to_colored_name(*profile)
                    );
                    for k in huntsman::configuration::at101_keys().iter() {
                        let key = commands::mappings::Key {
                            id: *k,
                            hypershift: hypershift,
                        };
                        let x = h.get_mapping(*profile, key)?;
                        let default_map = huntsman::configuration::get_default_keymap(&key);
                        if show_all || (default_map != x.mapping) {
                            let hypershift = if key.hypershift { ":hypershift" } else { "" };
                            let mapping: String;
                            match x.mapping {
                                commands::mappings::KeyMapping::Key(v) => {
                                    let key_name = if v.id != 0 {
                                        huntsman::configuration::keyboard_hid_to_key_name(v.id)?
                                    } else {
                                        ""
                                    };
                                    mapping = format!("{:?} ({})", v, key_name);
                                }
                                _ => mapping = format!("{:?}", x.mapping),
                            }
                            println!(
                                "  {: >30}{} {: >3} -> {}",
                                huntsman::configuration::at101_to_key_name(key.id)?,
                                hypershift,
                                key.id,
                                mapping
                            );
                        }
                    }
                }
            }
            Some("set") => {
                let submatches = matches.subcommand_matches("set").unwrap();
                let hypershift = submatches.occurrences_of("hypershift") == 1;
                let profile = get_profile_id(submatches)?;
                let key_string = get_value::<String>(submatches, "key")?;
                let mapping = get_value::<String>(submatches, "mapping")?;
                let key_id = huntsman::configuration::key_name_to_at101(key_string.as_str())?;
                let key = commands::mappings::Key {
                    id: key_id,
                    hypershift: hypershift,
                };
                let mapping = huntsman::configuration::read_mapping(mapping.as_str(), &key)?;
                h.set_mapping(profile, key, mapping)?;
                println!("Set key: {:?}, to: {:?}", key_string, mapping);
            }
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
