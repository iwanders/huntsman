use struct_helper::*;

#[derive(Inspectable, FromBytes, ToBytes, Debug, Default, Copy, Clone)]
// #[repr(C)]
struct StructWithFloat {
    float_inside: f32,
}

#[derive(Inspectable, FromBytes, ToBytes, Debug, Default, Copy, Clone)]
#[repr(C)]
struct Pancakes {
    first_char: u8,
    _x: [u8; 3], // This is ignored in the Inspectable entry
    an_uint: u32,
    a_float: f32,
    // #[struct_helper(annotation = "foo")]
    array_three_chars: [i8; 3],
    struct_z: StructWithFloat,
    array_with_three_structs: [StructWithFloat; 3],
}

#[test]
fn test_starts() {
    let bound = Pancakes::fields();
    println!("{:?}", bound);

    assert_eq!(offset_of!(Pancakes, first_char), bound[0].start());
    assert_eq!(offset_of!(Pancakes, an_uint), bound[1].start());
    assert_eq!(offset_of!(Pancakes, a_float), bound[2].start());
    assert_eq!(offset_of!(Pancakes, array_three_chars), bound[3].start());
    assert_eq!(offset_of!(Pancakes, struct_z), bound[4].start());
    assert_eq!(
        offset_of!(Pancakes, array_with_three_structs),
        bound[5].start()
    );
}

// To check our offsets in our tree and conversions we want to be able to convert between arbitrary structs and their
// bytes. The whole endeavour of this work is such that we can this safely from the primitives... but for tests.. we
// can be, and we should to compare whether we matched the ground truth.
//
// When a struct has padding, the bytes in the padding can be populated with anything, which doesn't help when
// we convert a struct this way to compare against our hand written one which will have the padding equal to zeros.
//
// https://doc.rust-lang.org/std/mem/fn.transmute.html#alternatives
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
fn struct_to_bytes_mut<T: Sized>(v: &mut T) -> &mut [u8] {
    unsafe {
        let rawptr = v as *mut T;
        let byte_ptr = rawptr as *mut u8; // the reinterpret_cast
        return std::slice::from_raw_parts_mut(byte_ptr, std::mem::size_of::<T>());
    }
}

#[test]
fn test_roundtrips_ranges_and_most_things() {
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
            for_lookup[0].name().expect("Should have a name"),
            "first_char"
        );
        raw_bytes[for_lookup[0].start()] = char_value; // first byte.
                                                       // 3 bytes padding.

        // And this will only work if the host is little endian as well...
        let int_bytes = int_value.to_le_bytes();
        assert_eq!(for_lookup[1].name().expect("Should have a name"), "an_uint");
        for i in 0..for_lookup[1].length() {
            raw_bytes[for_lookup[1].start() + i] = int_bytes[i];
        }

        let float_bytes = float_value.to_le_bytes();
        assert_eq!(for_lookup[2].name().expect("Should have a name"), "a_float");
        for i in 0..for_lookup[2].length() {
            raw_bytes[for_lookup[2].start() + i] = float_bytes[i];
        }

        // Now we get to the realm of nesting...
        let array_offset = for_lookup[3].start();
        assert_eq!(
            for_lookup[3].name().expect("Should have a name"),
            "array_three_chars"
        );
        for i in 0..for_lookup[3].elements().len() {
            raw_bytes[array_offset + for_lookup[3].elements()[i].start()] =
                char_array_value[i].to_le_bytes()[0];
        }

        let float_z_bytes = float_z_value.to_le_bytes();
        assert_eq!(
            for_lookup[4].name().expect("Should have a name"),
            "struct_z"
        );
        for i in 0..for_lookup[4].length() {
            raw_bytes[for_lookup[4].start() + i] = float_z_bytes[i];
        }

        assert_eq!(
            for_lookup[5].name().expect("Should have a name"),
            "array_with_three_structs"
        );
        let float_array = [float_1_value, float_2_value, float_3_value];
        for i in 0..for_lookup[5].elements().len() {
            let b = float_array[i].to_le_bytes();
            for j in 0..for_lookup[5].elements()[i].length() {
                raw_bytes[for_lookup[5].start() + for_lookup[5].elements()[i].start() + j] = b[j];
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
        let arr = expected_result.to_le_bytes().expect("Should succeed");

        // The expected result byte array should be identical to the array we just wrote.
        assert_eq!(struct_to_bytes(&expected_result), arr);
    }

    // If we do a roundtrip, this should work;
    {
        let arr = expected_result.to_le_bytes().expect("Should succeed");
        println!("arr: {:?}", arr);

        // The expected result byte array should be identical to the array we just wrote.
        assert_eq!(struct_to_bytes(&expected_result), arr);

        // And check whether we can read from the array.
        let read_back = Pancakes::from_le_bytes(&arr).expect("Should succeed");
        assert_eq!(
            struct_to_bytes(&expected_result),
            struct_to_bytes(&read_back)
        );
        // and this fails because of padding not being zerod :(
    }
}

#[derive(Inspectable, FromBytes, ToBytes, Debug, Default, Copy, Clone)]
struct StructWithInteger {
    int: u32,
}

#[test]
fn test_little_endianness() {
    let v = StructWithInteger { int: 0x11223344 };
    let little_endian_expected: [u8; 4] = [68, 51, 34, 17]; // [i for i in (struct.pack("<I", 0x11223344))]
    let arr = v.to_le_bytes().expect("Should succeed");
    assert_eq!(&arr, &little_endian_expected);
    let w = StructWithInteger::from_le_bytes(&arr).expect("Should succeed");
    assert_eq!(w.int, v.int);
}

#[test]
fn test_big_endianness() {
    let v = StructWithInteger { int: 0x11223344 };
    let big_endian_expected: [u8; 4] = [17, 34, 51, 68]; // [i for i in (struct.pack(">I", 0x11223344))]
    let arr = v.to_be_bytes().expect("Should succeed");
    assert_eq!(&arr, &big_endian_expected);
    let w = StructWithInteger::from_be_bytes(&arr).expect("Should succeed");
    assert_eq!(w.int, v.int);
}

// Let us test a dynamic length type;
struct VariableLengthStruct {
    data: Vec<u8>,
}
impl ToBytes for VariableLengthStruct {
    fn to_bytes(&self, endianness: Endianness) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = Vec::new();
        for z in 0..self.data.len() {
            buff.extend(self.data[z].to_bytes(endianness)?)
        }
        Ok(buff)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TripleEnum {
    One,
    Two,
    Three,
}

#[derive(Debug, Eq, PartialEq)]
struct Thing {
    data: Vec<TripleEnum>,
}

impl FromBytes for Thing {
    fn from_bytes(src: &[u8], _endianness: Endianness) -> Result<Thing, String> {
        let mut tmp: Thing = Thing { data: vec![] };
        for z in 0..src.len() {
            let v = match src[z] {
                1 => TripleEnum::One,
                2 => TripleEnum::Two,
                3 => TripleEnum::Three,
                _ => {
                    return Err("Nope".to_string());
                }
            };
            tmp.data.push(v)
        }
        Ok(tmp)
    }
}

#[test]
fn test_variable_length() {
    // let mut arr: [u8; 20] = [0; 20];
    let t = VariableLengthStruct {
        data: vec![1, 2, 3, 4],
    };
    let r = t.to_le_bytes().expect("Should succeed");
    println!("{:?}", r);
    assert_eq!(r[0], 1);

    assert_eq!(Thing::from_bytes(&r[..], Endianness::Little).is_err(), true);
    let one_less = &r[0..3];
    let res = Thing::from_bytes(&one_less, Endianness::Little).expect("Should be ok now");
    let expected = Thing {
        data: vec![TripleEnum::One, TripleEnum::Two, TripleEnum::Three],
    };
    assert_eq!(expected, res);
}
