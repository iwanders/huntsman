#[derive(Clone, Debug)]
pub struct HelloField {
    pub start: usize,
    pub length: usize,
    pub type_name: String,
    pub type_id: std::any::TypeId,
    pub name: Option<String>,
    pub children: Vec<HelloField>,
}

pub trait HelloMacro {
    fn hello_macro() -> () {}
    fn fields() -> HelloField;
}


//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
// println!("f32 as wizard: {}", <f32 as Wizard>::fly());
impl HelloMacro for f32 {
    fn fields() -> HelloField {
        HelloField {
            start: 0,
            length: std::mem::size_of::<f32>(),
            type_name: "f32".to_string(),
            type_id: std::any::TypeId::of::<f32>(),
            name: None,
            children: vec![],
        }
    }
}
impl HelloMacro for u8 {
    fn fields() -> HelloField {
        HelloField {
            start: 0,
            length: std::mem::size_of::<u8>(),
            type_name: "u8".to_string(),
            type_id: std::any::TypeId::of::<u8>(),
            name: None,
            children: vec![],
        }
    }
}
