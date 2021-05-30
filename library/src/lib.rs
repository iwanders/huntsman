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

// There's probably some elegant way to collapse this.
#[derive(Debug)]
pub enum Ref<'a> {
    I8(&'a i8),
    I16(&'a i16),
    I32(&'a i32),
    I64(&'a i64),
    I128(&'a i128),

    U8(&'a u8),
    U16(&'a u16),
    U32(&'a u32),
    U64(&'a u64),
    U128(&'a u128),

    F32(&'a f32),
    F64(&'a f64),

    Bool(&'a bool),

    None,
}

// use std::collections::HashMap;

#[derive(Debug)]
pub struct Info
{
    pub start: usize,
    pub length: usize,
    pub type_name: &'static str,
    pub type_id: std::any::TypeId,
    pub name: Option<&'static str>,
    // This feels 100% over the top, we'll have 0 to 1 keys at most. But this is the most flexible, allowing free-form
    // annotations to be completely specified by the user.
    pub attrs: std::collections::HashMap<&'static str, &'static str>,
}

// Struct to represent a field in a struct.
#[derive(Debug)]
pub struct FieldMut<'a> {
    pub info: Info,
    pub value: MutRef<'a>,
    pub children: Vec<FieldMut<'a>>,
}

#[derive(Debug)]
pub struct FieldRef<'a> {
    pub info: Info,
    pub value: Ref<'a>,
    pub children: Vec<FieldRef<'a>>,
}

#[derive(Debug)]
pub struct Field {
    pub info: Info,
    pub children: Vec<Field>,
}

pub trait Inspectable {
    fn fields_as_mut<'a>(&'a mut self) -> FieldMut<'a>;
    fn fields_as_ref<'a>(&'a self) -> FieldRef<'a>;
    fn fields() -> Field where Self: Sized;   // We need to be sized anyway for all this struct stuff.

}

//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name

//https://doc.rust-lang.org/rust-by-example/macros/designators.html
macro_rules! make_inspectable {
    ($a:ty, $as_mut: path, $as_ref: path) => {
        impl Inspectable for $a {
            fn fields_as_mut<'a>(&'a mut self) -> FieldMut {
                FieldMut {
                    info: Info{
                        start: 0,
                        length: std::mem::size_of::<$a>(),
                        type_name: std::any::type_name::<$a>(),
                        type_id: std::any::TypeId::of::<$a>(),
                        name: None,
                        attrs: std::collections::HashMap::new(),
                    },
                    value: $as_mut(self),
                    children: vec![],
                }
            }

            fn fields_as_ref<'a>(&'a self) -> FieldRef {
                FieldRef {
                    info: Info{
                        start: 0,
                        length: std::mem::size_of::<$a>(),
                        type_name: std::any::type_name::<$a>(),
                        type_id: std::any::TypeId::of::<$a>(),
                        name: None,
                        attrs: std::collections::HashMap::new(),
                    },
                    value: $as_ref(self),
                    children: vec![],
                }
            }

            fn fields() -> Field where Self: Sized {
                Field {
                    info: Info{
                        start: 0,
                        length: std::mem::size_of::<$a>(),
                        type_name: std::any::type_name::<$a>(),
                        type_id: std::any::TypeId::of::<$a>(),
                        name: None,
                        attrs: std::collections::HashMap::new(),
                    },
                    children: vec![],
                }
            }
        }
    };
}

make_inspectable!(i8, MutRef::I8, Ref::I8);
make_inspectable!(i16, MutRef::I16, Ref::I16);
make_inspectable!(i32, MutRef::I32, Ref::I32);
make_inspectable!(i64, MutRef::I64, Ref::I64);
make_inspectable!(i128, MutRef::I128, Ref::I128);

make_inspectable!(u8, MutRef::U8, Ref::U8);
make_inspectable!(u16, MutRef::U16, Ref::U16);
make_inspectable!(u32, MutRef::U32, Ref::U32);
make_inspectable!(u64, MutRef::U64, Ref::U64);
make_inspectable!(u128, MutRef::U128, Ref::U128);

make_inspectable!(f32, MutRef::F32, Ref::F32);
make_inspectable!(f64, MutRef::F64, Ref::F64);

make_inspectable!(bool, MutRef::Bool, Ref::Bool);
