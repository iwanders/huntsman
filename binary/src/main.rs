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
struct Pancakes
{
    #[hello("foo")]
    x: f32,
    arr: [u32; 3],
    s: Z,
}

#[library_macro::route]
fn foo()
{
}


fn main()
{
    Pancakes::hello_macro();
    println!("{:?}", Pancakes::fields());
}

