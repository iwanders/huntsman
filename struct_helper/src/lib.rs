//! This crate allows struct conversion from and to bytes, without using a transmute or pointer
//! cast. It also makes the struct's fields and any subfields inspectable and allows iteration over
//! the fields.
//!
//! At the time of writing, there's no safe way to get a field offset in Rust, to perform this step
//! the [`memoffset`] crate is used.

extern crate struct_helper_derive;

pub use memoffset::*;
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

impl Default for ElementType {
    fn default() -> ElementType {
        ElementType::Other
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

pub trait ToBytes {
    fn to_bytes(&self, endianness: Endianness) -> Result<Vec<u8>, String>;

    fn length_as_bytes(&self) -> Result<usize, String> {
        let r = self.to_bytes(Endianness::Little)?;
        Ok(r.len())
    }

    /// Convert this object into bytes at the destination buffer.
    fn to_le_bytes(&self) -> Result<Vec<u8>, String> {
        self.to_bytes(Endianness::Little)
    }

    /// Convert this object into bytes at the destination buffer.
    fn to_be_bytes(&self) -> Result<Vec<u8>, String> {
        self.to_bytes(Endianness::Big)
    }
}

pub trait FromBytes {
    fn from_bytes(&mut self, src: &[u8], endianness: Endianness) -> Result<usize, String>
    where
        Self: Sized;

    // from_le_bytes(src: &[u8]) -> Result<Self, String> is a bit more involved if we don't want to assume default
    // constructability... Need to tackle it on the derive side mostly... if we assume default constructability, it
    // becomes a lot easier and we can just grab the mutable field tree and recurse.
    /// Create an object from a byte buffer, this requires the type to be default constructible.
    fn from_le_bytes(src: &[u8]) -> Result<Self, String>
    where
        Self: Sized + Default,
    {
        let mut tmp: Self = Default::default();
        tmp.from_bytes(src, Endianness::Little)?;
        Ok(tmp)
    }

    /// Create an object from a byte buffer, this requires the type to be default constructible.
    fn from_be_bytes(src: &[u8]) -> Result<Self, String>
    where
        Self: Sized + Default,
    {
        let mut tmp: Self = Default::default();
        tmp.from_bytes(src, Endianness::Big)?;
        Ok(tmp)
    }

}

pub trait Inspectable: std::fmt::Debug {
    fn clone_box(&self) -> Box<dyn Inspectable>;
    fn fields() -> Vec<Box<dyn Inspectable>>
    where
        Self: Sized;

    fn inspect() -> Box<dyn Inspectable>
    where
        Self: Sized,
    {
        panic!("Doesn't implement the static constructor");
    }

    /// The start offset relative to the parent.
    fn start(&self) -> usize {
        0
    }

    fn set_start(&mut self, _start: usize) {
        panic!("Can't set start");
    }

    /// The length of this element.
    fn length(&self) -> usize {
        0
    }

    /// The type name, "u8", "u16", "MyStruct"...
    fn type_name(&self) -> &'static str;

    /// The name, if it has one (so name of the field if inside a struct).
    fn name(&self) -> Option<String> {
        None
    }

    // as_any?

    /// Returns the elements this instance has.
    fn elements(&self) -> Vec<Box<dyn Inspectable>> {
        vec![]
    }

    fn get(&self, search_name: &str) -> Option<Box<dyn Inspectable>> {
        let children = self.elements();
        for child in children.iter() {
            if let Some(name) = child.name() {
                if name == search_name {
                    return Some(child.clone_box());
                }
            }
        }
        None
    }

    /// Returns the fields this thing could return.
    // fn clone_box(&self) -> Box<dyn Information>
    // fn fields()() -> Vec<Box<dyn Information>> where Self:Sized
    // {
    // vec!()
    // }

    // fn inspect(&self) -> Box<dyn Inspectable>;

    // Below here are things from the old implementation, they may be redundant now.

    /// The type of this thing, may be redundant now?
    fn element_type(&self) -> ElementType {
        ElementType::Other
    }

    /// A hashmap that can contain arbitrary annotations to fields.
    fn attrs(&self) -> std::collections::HashMap<&'static str, &'static str> {
        std::collections::HashMap::new()
    }
}

impl Clone for Box<dyn Inspectable> {
    fn clone(&self) -> Box<dyn Inspectable> {
        self.clone_box()
    }
}

//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name

//https://doc.rust-lang.org/rust-by-example/macros/designators.html
#[derive(Default, Debug)]
pub struct SimpleInspectable {
    /// The start location of this field from its parent.
    pub start: usize,

    /// The length of this field.
    pub length: usize,

    /// A string representation of the type this field is.
    pub type_name: &'static str,

    /// If this field has a name, the name of the field, otherwise `None`.
    pub name: Option<String>,

    /// The type of field this element is.
    pub element_type: ElementType,

    /// A hashmap that can contain arbitrary annotations to fields.
    // This feels 100% over the top, we'll have 0 to 1 keys at most. But this is the most flexible, allowing free-form
    // annotations to be completely specified by the user.
    pub attrs: std::collections::HashMap<&'static str, &'static str>,

    pub elements: Vec<Box<dyn Inspectable>>,
}
impl Clone for SimpleInspectable {
    fn clone(&self) -> SimpleInspectable {
        SimpleInspectable {
            start: self.start,
            length: self.length,
            type_name: self.type_name,
            name: self.name.clone(),
            element_type: self.element_type,
            attrs: self.attrs.clone(),
            elements: self.elements.iter().map(|x| x.clone()).collect(),
        }
    }
}

impl Inspectable for SimpleInspectable {
    fn fields() -> Vec<Box<dyn Inspectable>>
    where
        Self: Sized,
    {
        panic!("One should never use the StaticInspectable...");
    }

    fn clone_box(&self) -> Box<dyn Inspectable> {
        Box::new(self.clone())
    }

    fn start(&self) -> usize {
        self.start
    }

    fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    fn length(&self) -> usize {
        self.length
    }
    fn type_name(&self) -> &'static str {
        self.type_name
    }

    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    fn element_type(&self) -> ElementType {
        self.element_type
    }

    /// Returns the elements this instance has.
    fn elements(&self) -> Vec<Box<dyn Inspectable>> {
        self.elements.iter().map(|x| x.clone_box()).collect() // yuck.
    }

    /// A hashmap that can contain arbitrary annotations to fields.
    fn attrs(&self) -> std::collections::HashMap<&'static str, &'static str> {
        self.attrs.clone()
    }
}

/// Helper macro to create the implementations for the primitive scalar types.
macro_rules! make_inspectable {
    ($a:ty) => {
        impl Inspectable for $a {
            /// The start offset relative to the parent.
            fn start(&self) -> usize {
                0
            }

            /// The length of this element.
            fn length(&self) -> usize {
                std::mem::size_of::<$a>()
            }

            /// The type name, "u8", "u16", "MyStruct"...
            fn type_name(&self) -> &'static str {
                std::any::type_name::<$a>()
            }

            fn element_type(&self) -> ElementType {
                ElementType::Scalar
            }

            fn fields() -> Vec<Box<dyn Inspectable>> {
                vec![]
            }

            fn clone_box(&self) -> Box<dyn Inspectable> {
                Box::new(self.clone())
            }

            fn inspect() -> Box<dyn Inspectable>
            where
                Self: Sized,
            {
                Box::new(SimpleInspectable {
                    start: 0,
                    length: std::mem::size_of::<$a>(),
                    type_name: std::any::type_name::<$a>(),
                    element_type: ElementType::Scalar,
                    ..Default::default()
                })
            }
        }
    };
}

macro_rules! make_wireable {
    ($a:ty) => {
        impl ToBytes for $a {
            fn to_bytes(&self, endianness: Endianness) -> Result<Vec<u8>, String> {
                // Why isn't this match the same as the if below? Because of the footgun if the label isn't known.
                // match endianness
                // {
                // Little => {bytes = (*self as $a).to_le_bytes()},
                // Big => {bytes = (*self as $a).to_be_bytes()},
                // };
                if endianness == Endianness::Big {
                    Ok((*self as $a).to_be_bytes().to_vec())
                } else {
                    Ok((*self as $a).to_le_bytes().to_vec())
                }
            }
        }
        impl FromBytes for $a {
            fn from_bytes(&mut self, src: &[u8], endianness: Endianness) -> Result<usize, String>
            where
                Self: Sized + Default,
            {
                use std::convert::TryInto;
                let len = std::mem::size_of::<$a>();
                if len > src.len() {
                    return Err(format!(
                        "Type is {} long, doesn't fit into {} provided.",
                        len,
                        src.len()
                    ));
                }
                let (value_bytes, _rest) = src.split_at(len);
                

                // Why... isn't this match the same as the if below!?
                // match endianness
                // {
                // Little => Ok(<$a>::from_le_bytes(bytes)),
                // Big => Ok(<$a>::from_be_bytes(bytes)),
                // }
                if endianness == Endianness::Big {
                    *self = <$a>::from_be_bytes(value_bytes.try_into().unwrap());
                    return Ok(len);
                } else {
                    *self = <$a>::from_le_bytes(value_bytes.try_into().unwrap());
                    return Ok(len);
                }
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

make_wireable!(i8);
make_wireable!(i16);
make_wireable!(i32);
make_wireable!(i64);
make_wireable!(i128);

make_wireable!(u8);
make_wireable!(u16);
make_wireable!(u32);
make_wireable!(u64);
make_wireable!(u128);

make_wireable!(f32);
make_wireable!(f64);
