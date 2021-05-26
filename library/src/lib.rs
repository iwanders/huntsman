
#[derive(Clone, Default, Debug)]
pub struct HelloField
{
    pub start: usize,
    pub length: usize,
    pub unit: String,
    pub name: String,
    //~ pub children: Vec<HelloField>;
}

pub trait HelloMacro {
    fn hello_macro();
    fn fields() -> Vec<HelloField>;
}

//~ pub const fn size_of<T>() -> Vec<HelloField> {
    //~ intrinsics::size_of::<T>()
//~ }
