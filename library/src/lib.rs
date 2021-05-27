
#[derive(Clone, Default, Debug)]
pub struct HelloField
{
    pub start: usize,
    pub length: usize,
    pub unit: String,
    pub name: Option<String>,
    pub children: Vec<HelloField>,
}

pub trait HelloMacro {
    fn hello_macro() -> () {
    }
    fn fields() -> HelloField;
}

impl HelloMacro for f32 {
    fn fields() -> HelloField
    {
        HelloField{start: 0, length: std::mem::size_of::<f32>(), unit: "f32".to_string(), name: None, children: vec!()}
    }
}



//~ pub const fn size_of<T>() -> Vec<HelloField> {
    //~ intrinsics::size_of::<T>()
//~ }
