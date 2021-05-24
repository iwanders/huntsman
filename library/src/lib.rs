
#[derive(Clone, Default, Debug)]
pub struct HelloField
{
    pub start: usize,
    pub length: usize,
    pub name: String,
}

pub trait HelloMacro {
    fn hello_macro();
    fn fields() -> Vec<HelloField>;
}
