extern crate library;
extern crate library_macro;

use library::HelloField;
use library::HelloMacro;
use library_macro::route;
use library_macro::HelloMacro;

#[derive(HelloMacro)]
struct StructWithFloat {
    float_inside: f32,
}

#[derive(HelloMacro)]
#[repr(C)]
struct Pancakes {
    #[hello("foo")]
    a_float: f32,
    array_three_char: [u8; 3],
    struct_Z: StructWithFloat,
}

#[library_macro::route]
fn foo() {}

#[macro_use]
extern crate memoffset;

fn main() {
    Pancakes::hello_macro();
    println!("{:?}", Pancakes::fields()); // [HelloField { start: 0, length: 4, unit: "f32", name: "x" }, HelloField { start: 8, length: 4, unit: "Z", name: "s" }]

    println!("Offset: {:?}", offset_of!(Pancakes, array_three_char));

    pub fn printer(f: &library::HelloField, indent: usize) {
        let mut ind: String = String::new();
        for i in 0..indent {
            ind += " ";
        }

        println!("{}name: {:?}", ind, f.name);
        println!("{} start: {:?}", ind, f.start);
        println!("{} length: {:?}", ind, f.length);
        println!("{} unit: {:?}", ind, f.unit);
        for c in &f.children {
            printer(&c, indent + 4);
        }
    }
    printer(&Pancakes::fields(), 0)
}
