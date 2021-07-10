# Huntsman

This [Rust][rust] workspace allows interacting with the [Razer Huntsman Elite][kbd] keyboard. 

Not all features of the keyboard are supported, neither do I intend to make this project feature
complete, it's mostly my means of learning a bit of Rust. Currently supported functionality is
actually fairly feature complete:

- Retrieving the serial number.
- The majority of the LED effects.
- Setting overall brightness.
- Setting a custom frame to the LEDs.
- Remapping keys (to keys, mouse, macro's, various HID pages, profile cycle, hypershift support).
- Hardware macro load and delete from yaml files.
- Profile creation and deletion.
- Switching profiles.

## Architecture

Here's a quick breakdown of the various components in the workspace.

The [`struct_helper`](/struct_helper) and [`struct_helper_derive`](/struct_helper_derive) crates
provide a derive macro that implements `StructHelper` for a particular struct. This allows iterating
over the fields in the struct. It also implements a safe way to serialize a particular struct to
bytes.

The [`huntsman`](/huntsman) crate provides a convenient `Huntsman` object to interact with the
keyboard and provides methods to manipulate it. It also provides a command line utility to interact
with the keyboard `cargo run --bin huntsman -- --help`. 

The [`huntsman::commands`](/huntsman/src/commands.rs) module holds the command messages and data
structuresthat can be serialized to the wire format. The
[`commands::mappings`](/huntsman/src/commands/mappings.rs) file holds data structures necessary to remap
key functionality from their default to the other supported functionality, such as other keys,
mouse clicks or repeating a keystroke on an interval.


The [`huntsman_dissector`](/huntsman_dissector) uses the `StructHelper` derived structs from the
[`huntsman::commands`](/huntsman/src/commands/) crate, together with the [`wireshark_dissector_rs`](/wireshark_dissector_rs)
crate to make a Wireshark dissector that dissects using the structs and fields that
are defined in the `huntsman` crate. This helps significanly with reverse engineering.

The [`torch`](/torch) crate contains a binary that can update the custom frame display on the
keyboard using various effects. These effect operations can be composed into a tree to make complex
effects. See the `cfg` folder in the `torch` crate.

The [`usb_hut`](/usb_hut) contains the usb HID usage tables used for the key mappings and macros.

## Usage

Install the `udev` rule found in the [`misc/udev/`](/misc/udev/) folder to `/etc/udev/rules.d` to
ensure you can use HID to interact with the keyboard without requiring elevated privileges.

### `./huntsman`
After building the workspace you can run the `huntsman` binary (`cargo run --bin huntsman -- --help`).

#### On macro's
The device doesn't care about whether or not the macro metadata (uuid, name, etc) is present.

When a key is mapped to a macro, and the macro is removed, the mapping remains. Recreating the macro
with the id that was used for the mapping makes the mapping function again using the new macro payload.
So one can modify / change / load updated macro's without having to set the mapping again.

When using the official software the keyboard ocasionally (often)  disconnects, this seems to be a bug
in the keyboard hardware as it also seems to occur from time to time when setting macro's without
the official software.

The official software always serializes delays in mouse move events using the four byte wide delay.
This is unnecessary, it also seems to send very small delta's, often only -1 or 1.

There's an example macro in the [`huntsman/cfg/`](/huntsman/cfg/) directory.

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