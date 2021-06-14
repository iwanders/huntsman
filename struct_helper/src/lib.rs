//! This crate allows struct conversion from and to bytes, without using a transmute or pointer
//! cast. It also makes the struct's fields and any subfields inspectable and allows iteration over
//! the fields.
//!
//! At the time of writing, there's no safe way to get a field offset in Rust, to perform this step
//! the [`memoffset`] crate is used.

// the attrs() system is kinda ugly... we should make that prettier.

extern crate struct_helper_derive;

pub use memoffset::*;
pub use struct_helper_derive::*;

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

/// Trait to allow conversion to Bytes.
pub trait ToBytes {
    /// The main worker method, returns a vector of bytes.
    fn to_bytes(&self, endianness: Endianness) -> Result<Vec<u8>, String>;

    /// Convert this object into bytes as little endian.
    fn to_le_bytes(&self) -> Result<Vec<u8>, String> {
        self.to_bytes(Endianness::Little)
    }

    /// Convert this object into bytes as big endian.
    fn to_be_bytes(&self) -> Result<Vec<u8>, String> {
        self.to_bytes(Endianness::Big)
    }
}

/// Trait to allow bytes to be converted into the type.
pub trait FromBytes {
    /// Main worker primitive, reads into self, returns the amount of bytes it read from src.
    fn from_bytes(&mut self, src: &[u8], endianness: Endianness) -> Result<usize, String>;

    /// Return an instantiated type from the provided buffer.
    fn from_le_bytes(src: &[u8]) -> Result<Self, String>
    where
        Self: Sized + Default,
    {
        let mut tmp: Self = Default::default();
        tmp.from_bytes(src, Endianness::Little)?;
        Ok(tmp)
    }

    /// Return an instantiated type from the provided buffer.
    fn from_be_bytes(src: &[u8]) -> Result<Self, String>
    where
        Self: Sized + Default,
    {
        let mut tmp: Self = Default::default();
        tmp.from_bytes(src, Endianness::Big)?;
        Ok(tmp)
    }
}

/// Trait to make something inspectible.
pub trait Inspectable: std::fmt::Debug {
    /// Makes a clone in a box from the current Inspectable object.
    fn clone_box(&self) -> Box<dyn Inspectable>;

    /// Return an inspectable that represents this type fully. So the returned value's
    /// elements() methods should be identical to called the returns [`Inspectable::elements()`].
    fn inspect() -> Box<dyn Inspectable>
    where
        Self: Sized,
    {
        panic!("Doesn't implement the static constructor");
    }

    /// Return a list of inspectables that hold represent the fields in this type. Or, if returns
    /// less in an instance, this should still return all possible fields.
    fn fields() -> Vec<Box<dyn Inspectable>>
    where
        Self: Sized;

    /// The start offset relative to the parent.
    fn start(&self) -> usize {
        0
    }

    /// Method to set the start, may be necessary when building up an Inspectable.
    fn set_start(&mut self, _start: usize) {
        panic!("Can't set start, it is not implemented for this Inspectable type.");
    }

    /// The length of this element.
    fn length(&self) -> usize {
        0
    }

    /// Method to set the length, may be necessary when building up an Inspectable.
    fn set_length(&mut self, _length: usize) {
        panic!("Can't set start, it is not implemented for this Inspectable type.");
    }

    /// The type_name, "u8", "u16", "MyStruct"...
    fn type_name(&self) -> &'static str;

    /// The name, if it has one (so name of the field if inside a struct).
    fn name(&self) -> Option<String> {
        None
    }

    /// Returns the elements/fields this instance has.
    fn elements(&self) -> Vec<Box<dyn Inspectable>> {
        vec![]
    }

    /// Updates the elements with the provided vector.
    fn set_elements(&mut self, _elements: Vec<Box<dyn Inspectable>>)
    {
        panic!("Can't set elements on this Inspectable type");
    }

    /// Helper function to retrieve the element with a certain name. None if not found.
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
/// Struct that the minimal Inspectable requirements, used by the derive() macro.
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

    /// The fields or children of this inspectable.
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
        panic!("One should never use the StaticInspectable as direct type.");
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

    fn set_length(&mut self, length: usize) {
        self.length = length;
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

    fn elements(&self) -> Vec<Box<dyn Inspectable>> {
        self.elements.iter().map(|x| x.clone_box()).collect() // yuck.
    }
    fn set_elements(&mut self, elements: Vec<Box<dyn Inspectable>>)
    {
        self.elements = elements;
    }


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

make_inspectable!(bool);

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

impl ToBytes for bool {
    fn to_bytes(&self, _endianness: Endianness) -> Result<Vec<u8>, String> {
        if *self {
            return Ok(vec![1]);
        }
        Ok(vec![0])
    }
}
impl FromBytes for bool {
    fn from_bytes(&mut self, src: &[u8], _endianness: Endianness) -> Result<usize, String>
    where
        Self: Sized + Default,
    {
        let len = 1;
        if len > src.len() {
            return Err(format!(
                "Type is {} long, doesn't fit into {} provided.",
                len,
                src.len()
            ));
        }
        *self = src[0] != 0;
        Ok(1)
    }
}
