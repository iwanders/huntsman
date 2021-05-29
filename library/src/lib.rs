#[derive(Debug)]
pub enum MutRef<'a> {
    U8(&'a mut u8),
    F32(&'a mut f32),
    None,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub value: MutRef<'a>,
    pub start: usize,
    pub length: usize,
    pub type_name: String,
    pub type_id: std::any::TypeId,
    pub name: Option<String>,
    pub children: Vec<Field<'a>>,
}

pub trait Inspectable {
    fn hello_macro() -> () {}
    fn fields<'a>(&'a mut self) -> Field<'a>;
}

//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
// println!("f32 as wizard: {}", <f32 as Wizard>::fly());
impl Inspectable for f32 {
    fn fields<'a>(&'a mut self) -> Field {
        Field {
            value: MutRef::F32(self),
            start: 0,
            length: std::mem::size_of::<f32>(),
            type_name: "f32".to_string(),
            type_id: std::any::TypeId::of::<f32>(),
            name: None,
            children: vec![],
        }
    }
}

impl Inspectable for u8 {
    fn fields<'a>(&'a mut self) -> Field {
        Field {
            value: MutRef::U8(self),
            start: 0,
            length: std::mem::size_of::<u8>(),
            type_name: "u8".to_string(),
            type_id: std::any::TypeId::of::<u8>(),
            name: None,
            children: vec![],
        }
    }
}
