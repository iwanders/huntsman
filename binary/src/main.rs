extern crate library_macro;
extern crate library;

use library_macro::HelloMacro;
use library_macro::route;
use library::HelloMacro;
use library::HelloField;

struct Z
{
    z: f32,
}

#[derive(HelloMacro)]
#[repr(C)]
struct Pancakes
{
    #[hello("foo")]
    x: f32,
    arr: [u8; 3],
    s: Z,
}

#[library_macro::route]
fn foo()
{
}


#[macro_use]
extern crate memoffset;

fn main()
{
    Pancakes::hello_macro();
    println!("{:?}", Pancakes::fields());

    println!("Offset: {:?}", offset_of!(Pancakes, s));
}

