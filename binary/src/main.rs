extern crate library;
extern crate library_macro;

use library::HelloField;
use library::HelloMacro;
use library_macro::route;
use library_macro::HelloMacro;

#[derive(HelloMacro, Debug)]
struct StructWithFloat {
    float_inside: f32,
}

#[derive(HelloMacro, Debug)]
#[repr(C)]
struct Pancakes {
    // #[hello("foo")]
    first_char: u8,
    a_float: f32,
    array_three_chars: [u8; 3],
    struct_z: StructWithFloat,
}

#[library_macro::route]
fn foo() {}

#[macro_use]
extern crate memoffset;

fn main() {
    // Pancakes::hello_macro();
    let mut stack: Pancakes = Pancakes{first_char: 3u8, a_float: 3.3, array_three_chars: [0, 0, 0], struct_z: StructWithFloat{float_inside: 8.8}};
    // println!("{:?}", stack.fields()); // [HelloField { start: 0, length: 4, unit: "f32", name: "x" }, HelloField { start: 8, length: 4, unit: "Z", name: "s" }]

    // println!("Offset: {:?}", offset_of!(Pancakes, array_three_chars));

    pub fn printer(f: &library::HelloField, indent: usize) {
        let mut ind: String = String::new();
        for i in 0..indent {
            ind += " ";
        }

        println!("{}name: {:?}", ind, f.name);
        println!("{} value: {:?}", ind, f.value);
        println!("{} start: {:?}", ind, f.start);
        println!("{} length: {:?}", ind, f.length);
        println!("{} type_name: {:?}", ind, f.type_name);
        println!("{} type_id: {:?}", ind, f.type_id);
        for c in &f.children {
            printer(&c, indent + 4);
        }
    }


    println!("{:?}", stack);
    let mut bound = stack.fields();
    printer(&bound, 0);

    printer(&bound.children[0], 0);
    match (&mut bound.children[0].children[0].value)
    {
        library::PrimitiveBind::U8(z) => 
        {
            **z = 127;
        },
        _ => {}
    }
    println!("{:?}", stack);

    stack.first_char = 10;
    println!("{:?}", stack);

    // &bound.children[0].assign_u8(127);

    let mut mu8: u8 = 3;

    let f = HelloField {
        value: library::PrimitiveBind::U8(&mut mu8),
        start: 0,
        length: std::mem::size_of::<u8>(),
        type_name: "u8".to_string(),
        type_id: std::any::TypeId::of::<u8>(),
        name: None,
        children: vec![],
    };

    // mu8 = 10;
    // println!("Mu8: {}", mu8);
    match f.value
    {
        library::PrimitiveBind::U8(mut v) => {
            println!("V: {}", v);
            *v = 123;
            println!("V: {}", v);
        }
        _ =>{}
    }
    println!("Mu8: {}", mu8);
    

}
