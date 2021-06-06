# Huntsman

This [Rust][rust] workspace allows interacting with the [Razer Huntsman Elite][kbd] keyboard. 

Not all features of the keyboard are supported, neither do I intend to make this project feature
complete, it's mostly my means of learning a bit of Rust.

## Architecture

Here's a quick breakdown of the various components in the workspace.

The [`struct_helper`](/struct_helper) and [`struct_helper_derive`](/struct_helper_derive) crates
provide a derive macro that implements `StructHelper` for a particular struct. This allows iterating
over the fields in the struct. It also implements a safe way to serialize a particular struct to
bytes.

The [`huntsman_comm`](/huntsman_comm) holds the `Command` trait, which is something we can send to
the keyboard. This crate also contains a module called `wire`, that holds struct definitions for
each command as it is sent over the USB bus. These structs all derive from `struct_helper`.
Main purpose is to provide convenient methods to create the instructions we can send to the keyboard
, lots of fields are unknown or not 100% certain as we have to determine the communication protocol
from reverse engineering USB packet captures.

The [`huntsman`](/huntsman) crate provides a convenient `Huntsman` object to interact with the
keyboard and provides methods to manipulate it. It also provides a command line utility to interact
with the keyboard.

The [`huntsman_dissector`](/huntsman_dissector) uses the `StructHelper` derived structs from the
[`huntsman_comm`](/huntsman_comm) crate, together with the [`wireshark_dissector_rs`](/wireshark_dissector_rs)
crate to make a Wireshark dissector that dissects using the structs and fields that
are defined in the `huntsman_comm` crate. This helps significanly with reverse engineering.

## Usage

Install the `udev` rule found in the [`misc/udev/`](/misc/udev/) folder to `/etc/udev/rules.d` to
ensure you can use HID to interact with the keyboard without requiring elevated privileges.

### `./huntsman`
After building the workspace you can run the `huntsman` binary (`cargo run --bin huntsman -- --help`).

### Wireshark dissector
Modify the values at the bottom of the `lib.rs` file to match your Wireshark version. Then symlink 
the built library in `target/debug/libhuntsman_dissector.so` to the Wireshark plugin folder in your
home dir, for Wireshark 2.6 that would be `~/.local/lib/wireshark/plugins/2.6/epan/libhuntsman_dissector.so`.

Read the [Wireshark wiki page on USB capture](https://wiki.wireshark.org/CaptureSetup/USB). On Linux that usually means you have to run:
```sh
modprobe usbmon
sudo setfacl -m u:$USER:r /dev/usbmon*
```
Wireshark provides a commandline utility to perform dissections, called `tshark`, this can make
it much easier to perform bulk dissections and analyse patterns, for example the following will
show command, length and payload for all messages except for the `SetLedState` (`0x0f03`) messages; 

```bash
for file in *.pcapng;do
  tshark -n -r "$file" -Y "!(huntsman.Command.cmd.u16.cmd == 0x0f03 ||  !huntsman.Command.cmd.u16.cmd)" -e huntsman.Command.cmd.u16.cmd -e huntsman.Command.len -e huntsman.payload -Tfields
done
```




## License
The [`wireshark_dissector_rs`][dissector_rs] and [`huntsman_dissector`](/huntsman_dissector) are `GPL-2.0-or-later`
because they link against Wireshark to build the dissector. The other crates are `MIT OR Apache-2.0`, that seems to be the common choice in the Rust community. 



[rust]: https://www.rust-lang.org/
[kbd]: https://www.razer.com/pc/gaming-keyboards/huntsman-family
[dissector_rs]: https://github.com/iwanders/wireshark_dissector_rs