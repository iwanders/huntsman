use struct_helper::*;

#[derive(StructHelper, Debug, Default, Copy, Clone)]
struct Z {
    f: u8,
}

#[derive(StructHelper, Debug, Default, Copy, Clone)]
#[repr(C, packed)]
struct Pancakes {
    first_char: u16,
    an_uint: u32,
    x: Z,
}

// https://github.com/rust-lang/rust/issues/46043
// Probably means the entire current setup with the references to primitives is ehm, busted.
// We do want to keep the functionality to read from a byte array though...

#[test]
fn test_starts() {
    let bound = Pancakes::fields();

    assert_eq!(
        offset_of!(Pancakes, first_char),
        bound.children[0].info.start
    );
    assert_eq!(offset_of!(Pancakes, an_uint), bound.children[1].info.start);
}
