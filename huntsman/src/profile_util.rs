// quick helpers to make reading the profile list easier and allow conversion.

use crate::commands::ProfileId;

/// Create ansi colorized profile string.
pub fn profile_to_colored_name(id: ProfileId) -> &'static str {
    match id {
        1 => concat!("\u{001b}[37m", "white", "\u{001b}[0m"),
        2 => concat!("\u{001b}[31m", "red", "\u{001b}[0m"),
        3 => concat!("\u{001b}[32m", "green", "\u{001b}[0m"),
        4 => concat!("\u{001b}[34m", "blue", "\u{001b}[0m"),
        5 => concat!("\u{001b}[36m", "cyan", "\u{001b}[0m"),
        _ => "Unknown profile id",
    }
}

/// Convert a profile id as string (number or color) to a profile id, if parsing fails, it returns
/// profile 1 (white).
pub fn str_to_profile_id(input: &str) -> ProfileId {
    let res = match input.to_lowercase().as_str() {
        "red" => 2,
        "green" => 3,
        "blue" => 4,
        "cyan" => 5,
        maybe_number => {
            if let Ok(v) = maybe_number.parse::<u8>() {
                v
            } else {
                1 // didn't parse as a number, use sane fallback.
            }
        }
    };
    res.clamp(1, 5)
}
