use struct_helper::*;

#[derive(StructHelper, Debug, Default, Copy, Clone)]
#[repr(C)] // Try this example with this line commented out and see how it changes!
struct Example {
    a_char: u8,
    a_short: i16,
    an_uint: u32,
    #[struct_helper(my_key = "Pi!")] // can add arbitrary key="string_value" pairs.
    a_float: f32,
}

fn main() -> Result<(), String> {
    let v: Example = Example {
        a_char: 130,
        a_short: -3120,
        an_uint: 0xDEADBEEF,
        a_float: std::f32::consts::PI,
    };

    let mut buffer: [u8; std::mem::size_of::<Example>()] = [0; std::mem::size_of::<Example>()];

    // Convert to bytes;
    v.to_le_bytes(&mut buffer)?;
    println!("v: {:?}", v);
    // v: Example { a_char: 130, a_short: -3120, an_uint: 3735928559, a_float: 3.1415927 }

    println!("buffer: {:?}", buffer);
    // buffer: [130, 0, 208, 243, 239, 190, 173, 222, 219, 15, 73, 64]

    buffer[0] = 3;

    // Convert from bytes back to a struct, returns a result holding the new struct.
    let x: Example = Example::from_le_bytes(&buffer)?;
    println!("x: {:?}", x);
    // x: Example { a_char: 3, a_short: -3120, an_uint: 3735928559, a_float: 3.1415927 }

    // Print a fancy ascii diagram showing the hexadecimal representation of the bytes, and which
    // bytes represent each field.
    println!(
        "\n    {}",
        buffer
            .iter()
            .map(|x| format!("{:0>2x}", x))
            .collect::<Vec<String>>()
            .join(" ")
    );
    for f in Example::fields().children.iter() {
        let info = &f.info;
        let s = 3; // each byte takes 3 characters in hex, with space to seperate them
        let start = f.info.start * s;
        let length = f.info.length * s - 1; // -1 to make the diagram pretty :)
        let remainder = (buffer.len() - f.info.start - f.info.length) * s;
        println!(
            "   {:>start$}|{:->length$}|{:remainder$} {name:}: {type_name: >3} {attrs:}",
            "",
            "",
            "",
            start = start,
            length = length,
            remainder = remainder,
            name = info.name.as_ref().unwrap(),
            type_name = info.type_name,
            attrs = if !info.attrs.is_empty() {
                format!(" {:?}", info.attrs)
            } else {
                "".to_string()
            }
        );
    }
    // It prints this:
    /*
        03 00 d0 f3 ef be ad de db 0f 49 40
       |--|                                  a_char: u8
             |-----|                         a_short: i16
                   |-----------|             an_uint: u32
                               |-----------| a_float: f32
    */

    Ok(())
}
