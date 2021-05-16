extern crate library_macro;
extern crate library;

use library_macro::HelloMacro;
use library::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main()
{
    Pancakes::hello_macro();
}