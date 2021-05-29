use library::Inspectable;

#[macro_use]
extern crate memoffset;

#[derive(Inspectable, Debug, Default, Copy, Clone)]
struct StructWithFloat {
    float_inside: f32,
}

#[derive(Inspectable, Debug, Default, Copy, Clone)]
#[repr(C)]
struct Pancakes {
    first_char: u8,
    an_uint: u32,
    a_float: f32,
    array_three_chars: [i8; 3],
    struct_z: StructWithFloat,
    array_with_three_structs: [StructWithFloat; 3],
}


#[test]
fn test_starts() {
    let mut stack: Pancakes = Default::default();
    let bound = stack.fields();

    assert_eq!(offset_of!(Pancakes, first_char), bound.children[0].start);
    assert_eq!(offset_of!(Pancakes, an_uint), bound.children[1].start);
    assert_eq!(offset_of!(Pancakes, a_float), bound.children[2].start);
    assert_eq!(offset_of!(Pancakes, array_three_chars), bound.children[3].start);
    assert_eq!(offset_of!(Pancakes, struct_z), bound.children[4].start);
    assert_eq!(offset_of!(Pancakes, array_with_three_structs), bound.children[5].start);
}

// To check our offsets in our tree, we want to be able to convert between arbitrary structs and their bytes.
// The whole endeavour of this work is such that we can this safely from the primitives... but for tests.. we can be
// unsafe. https://doc.rust-lang.org/std/mem/fn.transmute.html#alternatives .
#[allow(dead_code)]
fn struct_to_bytes<T: Sized>(v: &T) -> &[u8] {
    unsafe
    {
        let rawptr = v as *const T;
        let byte_ptr = rawptr as *const u8; // the reinterpret_cast
        // return a bounded slice of bytes for inspection.
        return std::slice::from_raw_parts(byte_ptr, std::mem::size_of::<T>());
    }
}


// And a mutable flavour...
#[allow(dead_code)]
fn struct_to_bytes_mut<T: Sized>(v: &mut T) -> &mut [u8] {
    unsafe
    {
        let rawptr = v as *mut T;
        let byte_ptr = rawptr as *mut u8; // the reinterpret_cast
        return std::slice::from_raw_parts_mut(byte_ptr, std::mem::size_of::<T>());
    }
}

#[test]
fn sdfsdf() {
    let mut to_be_modified: Pancakes = Default::default();
    let char_value: u8 = 100;
    let int_value = 0xDEADBEEFu32;
    let float_value: f32 = -1.0f32/3.0;
    let char_array_value: [i8; 3] = [-120, 0x55, 20];
    let expected_result: Pancakes = Pancakes{first_char: char_value, an_uint: int_value, a_float: float_value,array_three_chars: char_array_value,  ..Default::default()};


    {
        // This is pretty yucky, we need an extra instance just to obtain the field indices... :/ 
        let mut unused_mutable_instance: Pancakes = Default::default();
        let for_lookup = unused_mutable_instance.fields();

        let raw_bytes = struct_to_bytes_mut(&mut to_be_modified);
        assert_eq!(for_lookup.children[0].name.as_ref().expect("Should have a name"), "first_char");
        raw_bytes[for_lookup.children[0].start] = char_value;  // first byte.
        // 3 bytes padding.

        // And this will only work if the host is little endian as well...
        let int_bytes = int_value.to_le_bytes();
        assert_eq!(for_lookup.children[1].name.as_ref().expect("Should have a name"), "an_uint");
        for i in 0..for_lookup.children[1].length
        {
            raw_bytes[for_lookup.children[1].start + i] = int_bytes[i];
        }

        let float_bytes = float_value.to_le_bytes();
        assert_eq!(for_lookup.children[02].name.as_ref().expect("Should have a name"), "a_float");
        for i in 0..for_lookup.children[2].length
        {
            raw_bytes[for_lookup.children[2].start + i] = float_bytes[i];
        }

        // Now we get to the realm of nesting... 
        let array_offset = for_lookup.children[3].start;
        assert_eq!(for_lookup.children[3].name.as_ref().expect("Should have a name"), "array_three_chars");
        for i in 0..for_lookup.children[3].children.len()
        {
            raw_bytes[array_offset + for_lookup.children[3].children[i].start] = char_array_value[i].to_le_bytes()[0];
        }

        
    }
    println!("expected_result: {:?}", expected_result);
    println!("to_be_modified : {:?}", to_be_modified);
    assert_eq!(struct_to_bytes(&expected_result), struct_to_bytes(&to_be_modified));
}

