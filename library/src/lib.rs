#[derive(Debug)]
pub enum PrimitiveBind<'a>
{
    U8(&'a mut u8),
    F32(&'a  mut  f32),
    None,
}

#[derive(Debug)]
pub struct HelloField<'a> {
    pub value: PrimitiveBind<'a>,
    pub start: usize,
    pub length: usize,
    pub type_name: String,
    pub type_id: std::any::TypeId,
    pub name: Option<String>,
    pub children: Vec<HelloField<'a>>,
}

pub trait HelloMacro {
    fn hello_macro() -> () {}
    fn fields<'a>( &'a mut self) -> HelloField<'a>;

}


//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
// println!("f32 as wizard: {}", <f32 as Wizard>::fly());
impl HelloMacro for f32 {
    fn fields<'a>(&'a mut self) -> HelloField {
        HelloField {
            value: PrimitiveBind::F32(self),
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
    fn fields<'a>(&'a mut self) -> HelloField {
        HelloField {
            value: PrimitiveBind::U8(self),
            start: 0,
            length: std::mem::size_of::<u8>(),
            type_name: "u8".to_string(),
            type_id: std::any::TypeId::of::<u8>(),
            name: None,
            children: vec![],
        }
    }
}
