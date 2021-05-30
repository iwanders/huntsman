extern crate library;
extern crate library_macro;

// use library::Field;
use library::Inspectable;
// use library_macro::Inspectable;

#[derive(Inspectable, Debug, Default)]
struct StructWithFloat {
    float_inside: f32,
}

#[derive(Inspectable, Debug, Default)]
#[repr(C)]
#[hello(prefix = "foo", zulu = 3, 3)]
struct Pancakes {
    first_char: u8,
    a_float: f32,
    array_three_chars: [u8; 3],
    struct_z: StructWithFloat,
    array_with_three_structs: [StructWithFloat; 3],
}

#[macro_use]
extern crate memoffset;

// #[derive(Inspectable, Debug)]
#[repr(C)]
#[allow(dead_code)]
enum Flour {
    FullGrain(u8),
    White(f32),
}

fn main() {
    return;
    // Pancakes::hello_macro();
    // let mut stack: Pancakes = Pancakes{first_char: 3u8, a_float: 3.3, array_three_chars: [0, 0, 0], struct_z: StructWithFloat{float_inside: 8.8}};
    let mut stack: Pancakes = Default::default();
    // println!("{:?}", stack.fields()); // [HelloField { start: 0, length: 4, unit: "f32", name: "x" }, HelloField { start: 8, length: 4, unit: "Z", name: "s" }]

    // println!("Offset: {:?}", offset_of!(Pancakes, array_three_chars));

    pub fn printer(f: &library::MutableField, indent: usize) {
        let mut ind: String = String::new();
        for _i in 0..indent {
            ind += " ";
        }

        println!("{}name: {:?}", ind, f.info.name);
        println!("{} value: {:?}", ind, f.value);
        println!("{} start: {:?}", ind, f.info.start);
        println!("{} length: {:?}", ind, f.info.length);
        println!("{} type_name: {:?}", ind, f.info.type_name);
        println!("{} type_id: {:?}", ind, f.info.type_id);
        for c in &f.children {
            printer(&c, indent + 4);
        }
    }

    println!("{:?}", stack);
    let mut bound = stack.fields_as_mut();
    printer(&bound, 0);

    printer(&bound.children[0], 0);
    match &mut bound.children[0].children[0].value {
        library::MutRef::U8(z) => {
            **z = 127;
        }
        _ => {}
    }

    match &mut bound.children[2].children[0].value {
        library::MutRef::U8(z) => {
            **z = 33;
        }
        _ => {}
    }

    match &mut bound.children[4].children[1].children[0].children[0].value {
        library::MutRef::F32(z) => {
            **z = 1337.3;
        }
        _ => {}
    }

    println!("{:?}", stack);

    stack.first_char = 10;
    println!("{:?}", stack);

    // &bound.children[0].assign_u8(127);

    let mut mu8: u8 = 3;

    let f = library::MutableField {
        value: library::MutRef::U8(&mut mu8),
        info: library::Info{
        start: 0,
        length: std::mem::size_of::<u8>(),
        type_name: "u8",
        type_id: std::any::TypeId::of::<u8>(),
        name: None,},
        children: vec![],
    };


    // mu8 = 10;
    // println!("Mu8: {}", mu8);
    match f.value {
        library::MutRef::U8(v) => {
            println!("V: {}", v);
            *v = 123;
            println!("V: {}", v);
        }
        _ => {}
    }
    println!("Mu8: {}", mu8);
}
