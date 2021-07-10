// quick helpers to make reading the profile list easier and allow conversion.

use crate::commands::ProfileId;
pub fn profile_to_colored_name(id: ProfileId) -> &'static str {
    match id {
        1 => "boot_or_current_not_sure",
        2 => concat!("\u{001b}[31m", "red", "\u{001b}[0m"),
        3 => concat!("\u{001b}[32m", "green", "\u{001b}[0m"),
        4 => concat!("\u{001b}[34m", "blue", "\u{001b}[0m"),
        5 => concat!("\u{001b}[36m", "cyan", "\u{001b}[0m"),
        _ => "Unknown profile id",
    }
}
