extern crate library_macro;

pub use library_macro::Inspectable;

// Don't really know how to forward these to dependants...
// #[macro_use]
// extern crate memoffset;

// Enum to hold the mutable scalar types in Rust; https://doc.rust-lang.org/rust-by-example/primitives.html#scalar-types
#[derive(Debug)]
pub enum MutRef<'a> {
    I8(&'a mut i8),
    I16(&'a mut i16),
    I32(&'a mut i32),
    I64(&'a mut i64),
    I128(&'a mut i128),

    U8(&'a mut u8),
    U16(&'a mut u16),
    U32(&'a mut u32),
    U64(&'a mut u64),
    U128(&'a mut u128),

    F32(&'a mut f32),
    F64(&'a mut f64),

    Bool(&'a mut bool),

    None,
}

// Struct to represent a field in a struct.
#[derive(Debug)]
pub struct MutableField<'a> {
    pub value: MutRef<'a>,

    pub start: usize,
    pub length: usize,
    pub type_name: &'static str,
    pub type_id: std::any::TypeId,
    pub name: Option<String>,
    pub children: Vec<MutableField<'a>>,
}

#[derive(Debug)]
pub struct Field {
    pub start: usize,
    pub length: usize,
    pub type_name: &'static str,
    pub type_id: std::any::TypeId,
    pub name: Option<String>,
    pub children: Vec<Field>,
}


pub trait Inspectable {
    fn fields_as_mut<'a>(&'a mut self) -> MutableField<'a>;
    fn fields() -> Field where Self: Sized;   // We need to be sized anyway for all this struct stuff.
}

//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name

//https://doc.rust-lang.org/rust-by-example/macros/designators.html
macro_rules! make_inspectable {
    ($a:ty, $v: path) => {
        impl Inspectable for $a {
            fn fields_as_mut<'a>(&'a mut self) -> MutableField {
                MutableField {
                    value: $v(self),
                    start: 0,
                    length: std::mem::size_of::<$a>(),
                    type_name: std::any::type_name::<$a>(),
                    type_id: std::any::TypeId::of::<$a>(),
                    name: None,
                    children: vec![],
                }
            }

            fn fields() -> Field where Self: Sized {
                Field {
                    start: 0,
                    length: std::mem::size_of::<$a>(),
                    type_name: std::any::type_name::<$a>(),
                    type_id: std::any::TypeId::of::<$a>(),
                    name: None,
                    children: vec![],
                }
            }
        }
    };
}

make_inspectable!(i8, MutRef::I8);
make_inspectable!(i16, MutRef::I16);
make_inspectable!(i32, MutRef::I32);
make_inspectable!(i64, MutRef::I64);
make_inspectable!(i128, MutRef::I128);

make_inspectable!(u8, MutRef::U8);
make_inspectable!(u16, MutRef::U16);
make_inspectable!(u32, MutRef::U32);
make_inspectable!(u64, MutRef::U64);
make_inspectable!(u128, MutRef::U128);

make_inspectable!(f32, MutRef::F32);
make_inspectable!(f64, MutRef::F64);

make_inspectable!(bool, MutRef::Bool);
