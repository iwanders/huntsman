struct_helper
=============

This crate helps introspect structs, providing information about the fields a struct hold, their respective offset from
the start and their length. Besides this structs and their fields can be annotated with arbitrary information.

It also add support to read a struct from a byte vector and write the structs' contents to a byte vector. This is done
in a way that's identical to a 'reinterpret_cast' to bytes. In rust there's no unsafe way to do this, this crate
attempts to solve that by recursively walking through the struct, keeping track of the offsets until we reach a
primitive scalar value. Then on these primitive scalars the `.to_le_bytes()`, or `from_le_bytes()` methods are invoked.

To keep track of the offsets for struct sub fields, the [memoffset](https://crates.io/crates/memoffset) crate is used,
which unfortunately can only be done (currently) through `unsafe` operations.
