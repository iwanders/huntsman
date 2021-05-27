extern crate library_macro;
extern crate library;

use library_macro::HelloMacro;
use library_macro::route;
use library::HelloMacro;
use library::HelloField;

#[derive(HelloMacro)]
struct Z
{
    z: f32,
}

#[derive(HelloMacro)]
#[repr(C)]
struct Pancakes
{
    #[hello("foo")]
    a_float: f32,
    array_three_char: [u8; 3],
    struct_Z: Z,
}


#[library_macro::route]
fn foo()
{
}


#[macro_use]
extern crate memoffset;


//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
trait Pilot {
    fn fly() -> u32;
}

trait Wizard {
    fn fly() -> u32;
}

struct Human;

impl Pilot for Human {
    fn fly() -> u32 {
        println!("This is your captain speaking.");
        5
    }
}

impl Wizard for Human {
    fn fly() -> u32 {
        println!("Up!");
        1
    }
}

impl Wizard for u8 {
    fn fly() -> u32
    {
        println!("Up!");
        1
    }
}


impl Wizard for f32 {
    fn fly() -> u32 {
        println!("Up!");
        4
    }
}


fn main()
{
    Pancakes::hello_macro();
    println!("{:?}", Pancakes::fields());  // [HelloField { start: 0, length: 4, unit: "f32", name: "x" }, HelloField { start: 8, length: 4, unit: "Z", name: "s" }]

    println!("Offset: {:?}", offset_of!(Pancakes, array_three_char));

    <Human as Wizard>::fly();
    <Human as Pilot>::fly();
    println!("u8 as wizard: {}", <u8 as Wizard>::fly());
    println!("f32 as wizard: {}", <f32 as Wizard>::fly());


    pub fn printer(f: &library::HelloField, indent: usize)
    {
        let mut ind: String = String::new();
        for i in 0..indent
        {
            ind += " ";
        }

        println!("{}name: {:?}", ind, f.name);
        println!("{} start: {:?}", ind, f.start);
        println!("{} length: {:?}", ind, f.length);
        println!("{} unit: {:?}", ind, f.unit);
        for c in &f.children
        {
            printer(&c, indent +4);
        }
    }
    printer(&Pancakes::fields(), 0)
}

