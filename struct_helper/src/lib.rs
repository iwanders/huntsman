//! This crate allows struct conversion from and to bytes, without using a transmute or pointer
//! cast. It also makes the struct's fields and any subfields inspectable and allows iteration over
//! the fields.
//!
//! At the time of writing, there's no safe way to get a field offset in Rust, to perform this step
//! the [`memoffset`] crate is used.

extern crate struct_helper_derive;

pub use memoffset::*;
/// fooo
pub use struct_helper_derive::*;

/*
This architecture can only deal with leafs of primitives, if someone puts something in their struct and wants to implement
their own StructHelper, or to_le_bytes function for that type, that doesn't work because it can't be captured in the (Mut)Ref
that's defined in this crate.

The whole reference situation is ugly atm, but it's useful as it allows building the struct by reading the primitives
from bytes, or writing bytes from the primitives.
*/

// use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// Enum to denote the element type for this field.
pub enum ElementType {
    /// It's a path, this can be a primitive type, or any other named type.
    Path,

    /// It's an array,
    Array,

    /// It's a scalar, a primitive type
    Scalar,

    /// It's something else, root of an derived struct for example.
    Other,
}

#[derive(Debug, Clone)]
/// Info struct to hold common information for fields.
pub struct Info {
    /// The start location of this field from its parent.
    pub start: usize,

    /// The length of this field.
    pub length: usize,

    /// A string representation of the type this field is.
    pub type_name: &'static str,

    /// A [`std::any::TypeId`] instance for the type of this field.
    pub type_id: std::any::TypeId,

    /// If this field has a name, the name of the field, otherwise `None`.
    pub name: Option<&'static str>,

    /// The type of field this element is.
    pub element_type: ElementType,

    /// A hashmap that can contain arbitrary annotations to fields.
    // This feels 100% over the top, we'll have 0 to 1 keys at most. But this is the most flexible, allowing free-form
    // annotations to be completely specified by the user.
    pub attrs: std::collections::HashMap<&'static str, &'static str>,
}

#[derive(Debug, Clone)]
/// Struct to provide information about a field, without any reference to it.
pub struct Field {
    pub info: Info,
    pub children: Vec<Field>,
}

impl Field {
    /// Retrieve a particular field by name.
    pub fn find(&self, name: &str) -> Option<Field> {
        for i in 0..self.children.len() {
            match self.children[i].info.name {
                Some(n) => {
                    if n == name {
                        return Some(self.children[i].clone());
                    }
                }
                None => {}
            }
        }
        return None;
    }
}


/// Main trait that provides inspection functions to objects that implement this trait, or as a
/// static method to the type itself.
pub trait StructHelper {

    /// Retrieve the fields without references.
    fn fields() -> Field where Self: Sized;
    // We need to be sized anyway for all this struct stuff.

    /// Convert this object into bytes at the destination buffer.
    fn to_le_bytes(&self, dest: &mut [u8]) -> Result<(), String>;

    // from_le_bytes(src: &[u8]) -> Result<Self, String> is a bit more involved if we don't want to assume default
    // constructability... Need to tackle it on the derive side mostly... if we assume default constructability, it
    // becomes a lot easier and we can just grab the mutable field tree and recurse.
    /// Create an object from a byte buffer, this requires the type to be default constructible.
    fn from_le_bytes(src: &[u8]) -> Result<Self, String> where Self: Sized + Default;
}

//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name

//https://doc.rust-lang.org/rust-by-example/macros/designators.html


/// Helper macro to create the implementations for the primitive scalar types.
macro_rules! make_inspectable {
    ($a:ty) => {
        impl StructHelper for $a {
            fn fields() -> Field
            where
                Self: Sized,
            {
                Field {
                    info: Info {
                        start: 0,
                        length: std::mem::size_of::<$a>(),
                        type_name: std::any::type_name::<$a>(),
                        type_id: std::any::TypeId::of::<$a>(),
                        name: None,
                        element_type: ElementType::Scalar,
                        attrs: std::collections::HashMap::new(),
                    },
                    children: vec![],
                }
            }

            fn to_le_bytes(&self, dest: &mut [u8]) -> Result<(), String>
            {
                let bytes = (*self as $a).to_le_bytes();
                if bytes.len() != dest.len()
                {
                    return Err(format!("Type is {} long, doesn't fit into {} provided.", bytes.len(), dest.len()));
                }
                for i in 0..bytes.len()
                {
                    dest[i] = bytes[i];
                }
                Ok(())
            }

            fn from_le_bytes(src: &[u8]) -> Result<Self, String> where Self: Sized + Default
            {
                let dummy: $a = Default::default();
                let mut bytes = dummy.to_le_bytes(); // just to create an appropriately sized array easily.
                if bytes.len() != src.len()
                {
                    return Err(format!("Type is {} long, doesn't fit into {} provided.", bytes.len(), src.len()));
                }
                // Now, we can read the bytes.
                for i in 0..bytes.len()
                {
                    bytes[i] = src[i];
                }
                // and perform the real read.
                Ok(<$a>::from_le_bytes(bytes))
            }
        }
    };
}

make_inspectable!(i8);
make_inspectable!(i16);
make_inspectable!(i32);
make_inspectable!(i64);
make_inspectable!(i128);

make_inspectable!(u8);
make_inspectable!(u16);
make_inspectable!(u32);
make_inspectable!(u64);
make_inspectable!(u128);

make_inspectable!(f32);
make_inspectable!(f64);

// make_inspectable!(bool);

