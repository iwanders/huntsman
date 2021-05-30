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
    _x: [u8; 3],  // This is ignored in the Inspectable entry
    an_uint: u32,
    a_float: f32,
    array_three_chars: [i8; 3],
    struct_z: StructWithFloat,
    array_with_three_structs: [StructWithFloat; 3],
}

#[test]
fn test_starts() {
    let mut stack: Pancakes = Default::default();
    let bound = stack.fields_as_mut();

    assert_eq!(
        offset_of!(Pancakes, first_char),
        bound.children[0].info.start
    );
    assert_eq!(offset_of!(Pancakes, an_uint), bound.children[1].info.start);
    assert_eq!(offset_of!(Pancakes, a_float), bound.children[2].info.start);
    assert_eq!(
        offset_of!(Pancakes, array_three_chars),
        bound.children[3].info.start
    );
    assert_eq!(offset_of!(Pancakes, struct_z), bound.children[4].info.start);
    assert_eq!(
        offset_of!(Pancakes, array_with_three_structs),
        bound.children[5].info.start
    );
}

// To check our offsets in our tree, we want to be able to convert between arbitrary structs and their bytes.
// The whole endeavour of this work is such that we can this safely from the primitives... but for tests.. we can be
// unsafe. https://doc.rust-lang.org/std/mem/fn.transmute.html#alternatives .
#[allow(dead_code)]
fn struct_to_bytes<T: Sized>(v: &T) -> &[u8] {
    unsafe {
        let rawptr = v as *const T;
        let byte_ptr = rawptr as *const u8; // the reinterpret_cast
                                            // return a bounded slice of bytes for inspection.
        return std::slice::from_raw_parts(byte_ptr, std::mem::size_of::<T>());
    }
}

// And a mutable flavour...
#[allow(dead_code)]
fn struct_to_bytes_mut<T: Sized>(v: &mut T) -> &mut [u8] {
    unsafe {
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
    let float_value: f32 = -1.0f32 / 3.0;
    let float_z_value: f32 = 100f32 / 3.0;
    let char_array_value: [i8; 3] = [-120, 0x55, 20];

    let float_1_value: f32 = 1.0f32 / 3.0;
    let float_2_value: f32 = 254468546546.0f32 / 3.0;
    let float_3_value: f32 = 3156416.0f32 / 3.0;
    let expected_result: Pancakes = Pancakes {
        first_char: char_value,
        _x: [0; 3],
        an_uint: int_value,
        a_float: float_value,
        array_three_chars: char_array_value,
        struct_z: StructWithFloat {
            float_inside: float_z_value,
        },
        array_with_three_structs: [
            StructWithFloat {
                float_inside: float_1_value,
            },
            StructWithFloat {
                float_inside: float_2_value,
            },
            StructWithFloat {
                float_inside: float_3_value,
            },
        ],
    };

    {
        let for_lookup = Pancakes::fields();

        let raw_bytes = struct_to_bytes_mut(&mut to_be_modified);
        assert_eq!(
            for_lookup.children[0]
                .info
                .name
                .expect("Should have a name"),
            "first_char"
        );
        raw_bytes[for_lookup.children[0].info.start] = char_value; // first byte.
                                                                   // 3 bytes padding.

        // And this will only work if the host is little endian as well...
        let int_bytes = int_value.to_le_bytes();
        assert_eq!(
            for_lookup.children[1]
                .info
                .name
                .expect("Should have a name"),
            "an_uint"
        );
        for i in 0..for_lookup.children[1].info.length {
            raw_bytes[for_lookup.children[1].info.start + i] = int_bytes[i];
        }

        let float_bytes = float_value.to_le_bytes();
        assert_eq!(
            for_lookup.children[2]
                .info
                .name
                .expect("Should have a name"),
            "a_float"
        );
        for i in 0..for_lookup.children[2].info.length {
            raw_bytes[for_lookup.children[2].info.start + i] = float_bytes[i];
        }

        // Now we get to the realm of nesting...
        let array_offset = for_lookup.children[3].info.start;
        assert_eq!(
            for_lookup.children[3]
                .info
                .name
                .expect("Should have a name"),
            "array_three_chars"
        );
        for i in 0..for_lookup.children[3].children.len() {
            raw_bytes[array_offset + for_lookup.children[3].children[i].info.start] =
                char_array_value[i].to_le_bytes()[0];
        }

        let float_z_bytes = float_z_value.to_le_bytes();
        assert_eq!(
            for_lookup.children[4]
                .info
                .name
                .expect("Should have a name"),
            "struct_z"
        );
        for i in 0..for_lookup.children[4].info.length {
            raw_bytes[for_lookup.children[4].info.start + i] = float_z_bytes[i];
        }

        assert_eq!(
            for_lookup.children[5]
                .info
                .name
                .expect("Should have a name"),
            "array_with_three_structs"
        );
        let float_array = [float_1_value, float_2_value, float_3_value];
        for i in 0..for_lookup.children[5].children.len() {
            let b = float_array[i].to_le_bytes();
            for j in 0..for_lookup.children[5].children[i].info.length {
                raw_bytes[for_lookup.children[5].info.start
                    + for_lookup.children[5].children[i].info.start
                    + j] = b[j];
            }
        }
    }
    println!("expected_result: {:?}", expected_result);
    println!("to_be_modified : {:?}", to_be_modified);
    assert_eq!(
        struct_to_bytes(&expected_result),
        struct_to_bytes(&to_be_modified)
    );

    // Also check the to_le_bytes operation for the pancake struct.
    {
        let mut arr: [u8; std::mem::size_of::<Pancakes>()] = [0; std::mem::size_of::<Pancakes>()];
        expected_result
            .to_le_bytes(&mut arr)
            .expect("Should succeed");

        // The expected result byte array should be identical to the array we just wrote.
        assert_eq!(struct_to_bytes(&expected_result), arr);
    }

    // If we do the same trick on the fieldref it should also work.
    {
        let mut arr: [u8; std::mem::size_of::<Pancakes>()] = [0; std::mem::size_of::<Pancakes>()];
        expected_result
            .fields_as_ref()
            .to_le_bytes(&mut arr)
            .expect("Should succeed");
        println!("arr: {:?}", arr);

        // The expected result byte array should be identical to the array we just wrote.
        assert_eq!(struct_to_bytes(&expected_result), arr);

        // And check whether we can read from the array.
        let read_back = Pancakes::from_le_bytes(&arr).expect("Should succeed");
        assert_eq!(struct_to_bytes(&expected_result), struct_to_bytes(&read_back));
        // and this fails because of padding not being zerod :(
    }
}

#[test]
fn test_to_le_bytes() {
    let z: StructWithFloat = StructWithFloat {
        float_inside: 3.333,
    };
    let x = z.clone();
    let mut arr: [u8; std::mem::size_of::<StructWithFloat>()] =
        [0; std::mem::size_of::<StructWithFloat>()];
    z.to_le_bytes(&mut arr).expect("Should succeed");

    assert_eq!(struct_to_bytes(&x), arr);
}
