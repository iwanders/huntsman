extern crate library_macro;
extern crate library;

use library_macro::HelloMacro;
use library_macro::route;
use library::HelloMacro;

struct Z
{
    z: f32,
}

#[derive(HelloMacro)]
struct Pancakes
{
    x: f32,
    arr: [u32; 3],
    s: Z,
}

fn main()
{
    Pancakes::hello_macro();
}

#[library_macro::route]
fn foo()
{
}