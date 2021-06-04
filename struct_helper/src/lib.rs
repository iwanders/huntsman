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

// Enum to hold the mutable scalar types in Rust; https://doc.rust-lang.org/rust-by-example/primitives.html#scalar-types
#[derive(Debug)]
/// Enum to hold mutable scalar value types.
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

#[derive(Debug)]
/// Enum to hold immutable scalar value types.
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

/// Helper macro to expand types into a match case.
macro_rules! expand_cases_to_le_bytes {
    ($input:ident, $dest:ident, $( $y:path ),*) => (
        match ($input) {
                $($y(d) => {
                    let bytes = d.to_le_bytes();
                    if bytes.len() != $dest.len()
                    {
                        return Err(format!("Type is {} long, doesn't fit into {} provided.", bytes.len(), $dest.len()));
                    }
                    for i in 0..bytes.len()
                    {
                        $dest[i] = bytes[i];
                    }
                },)+
                _ => {return Err(format!("Reached unhandled for conversion."))},
        }
    )
}

/// Worker function to convert a [`Ref`] to bytes.
fn ref_to_le_bytes(value: &Ref, dest: &mut [u8]) -> Result<(), String> {
    // This is very ugly :(
    expand_cases_to_le_bytes!(
        value,
        dest,
        Ref::I8,
        Ref::I16,
        Ref::I32,
        Ref::I64,
        Ref::I128,
        Ref::U8,
        Ref::U16,
        Ref::U32,
        Ref::U64,
        Ref::U128,
        Ref::F32,
        Ref::F64
    );
    // Ref::Bool is missing here as it doesn't have to_le_bytes :/
    Ok(())
}

/// Helper macro to expand types into a match case.
macro_rules! expand_cases_from_le_bytes {
    ($value:ident, $src:ident, $( ($y:path, $e:ty) ),*) => (
        match ($value) {
                $($y(d) => {
                    // do some smart to assign into d.
                    // d = f32::from_le_bytes(bytes: [u8; 4])
                    let mut bytes = d.to_le_bytes(); // just to create an appropriately sized array easily.
                    if bytes.len() != $src.len()
                    {
                        return Err(format!("Type is {} long, doesn't fit into {} provided.", bytes.len(), $src.len()));
                    }
                    // Now, we can read the bytes.
                    for i in 0..bytes.len()
                    {
                        bytes[i] = $src[i];
                    }
                    // and perform the real read.
                    **d = <$e>::from_le_bytes(bytes);
                },)+
                _ => {return Err(format!("Reached unhandled for conversion."))},
        }
    )
}

/// Worker function to read into a [`MutRef`] from bytes.
fn mut_from_le_bytes(value: &mut MutRef, src: &[u8]) -> Result<(), String> {
    expand_cases_from_le_bytes!(
        value,
        src,
        (MutRef::I8, i8), // just because we miss decltype :(
        (MutRef::I16, i16),
        (MutRef::I32, i32),
        (MutRef::I64, i64),
        (MutRef::I128, i128),
        (MutRef::U8, u8),
        (MutRef::U16, u16),
        (MutRef::U32, u32),
        (MutRef::U64, u64),
        (MutRef::U128, u128),
        (MutRef::F32, f32),
        (MutRef::F64, f64)
    );
    // Ref::Bool is missing here as it doesn't have from_le_bytes :/
    Ok(())
}

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

#[derive(Debug)]
/// Struct to provide information and a reference to a mutable field.
pub struct FieldMut<'a> {
    pub info: Info,
    pub value: MutRef<'a>,
    pub children: Vec<FieldMut<'a>>,
}

#[derive(Debug)]
/// Struct to provide information and a reference to a field.
pub struct FieldRef<'a> {
    pub info: Info,
    pub value: Ref<'a>,
    pub children: Vec<FieldRef<'a>>,
}

impl FieldRef<'_> {
    /// Converts whatever the field holds to bytes, if this fails because the buffer isn't large
    /// enough it provides a string error message.
    pub fn to_le_bytes(self, dest: &mut [u8]) -> Result<(), String>
    where
        Self: Sized,
    {
        return impl_to_le_bytes(&self, dest);
    }
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

/// Function that recurses from the provided field down, trying to convert each field to bytes in
/// the buffer.
fn impl_to_le_bytes(v: &FieldRef, dest: &mut [u8]) -> Result<(), String> {
    // Here we go... We inspect v, and then we do the magic thing and recurse, and out should come a
    // perfect struct :o
    if !v.children.is_empty() {
        for c in v.children.iter() {
            // recurse...
            impl_to_le_bytes(&c, &mut dest[c.info.start..(c.info.start + c.info.length)])?
        }
    } else {
        // We have reached a leaf... perform a final check
        if dest.len() != v.info.length {
            return Err(format!(
                "Field length doesn't match available buffer need `{}`, buffer: {}",
                v.info.length,
                dest.len()
            ));
        }
        // And then convert the wrapped reference appropriately.
        ref_to_le_bytes(&v.value, dest)?;
    }

    Ok(())
}

/// Function that recurses from the provided mutable field down, trying to conver bytes from the
/// buffer into the references of the mutable fields.
fn impl_from_le_bytes(v: &mut FieldMut, src: &[u8]) -> Result<(), String> {
    if !v.children.is_empty() {
        for i in 0..v.children.len() {
            // recurse...
            let start = v.children[i].info.start;
            let len = v.children[i].info.length;
            impl_from_le_bytes(&mut v.children[i], &src[start..(start + len)])?
        }
    } else {
        // We have reached a leaf... perform a final check
        if src.len() != v.info.length {
            return Err(format!(
                "Field length doesn't match available buffer need `{}`, buffer: {}",
                v.info.length,
                src.len()
            ));
        }
        // And then convert the wrapped reference appropriately.
        mut_from_le_bytes(&mut v.value, src)?;
    }

    Ok(())
}

/// Main trait that provides inspection functions to objects that implement this trait, or as a
/// static method to the type itself.
pub trait StructHelper {
    /// Retrieve the fields with mutable references.
    fn fields_as_mut<'a>(&'a mut self) -> FieldMut<'a>;

    /// Retrieve the fields with references.
    fn fields_as_ref<'a>(&'a self) -> FieldRef<'a>;

    /// Retrieve the fields without references.
    fn fields() -> Field
    where
        Self: Sized; // We need to be sized anyway for all this struct stuff.

    /// Convert this object into bytes at the destination buffer.
    fn to_le_bytes(self, dest: &mut [u8]) -> Result<(), String>
    where
        Self: Sized,
    {
        return impl_to_le_bytes(&self.fields_as_ref(), dest);
    }

    // from_le_bytes(src: &[u8]) -> Result<Self, String> is a bit more involved if we don't want to assume default
    // constructability... Need to tackle it on the derive side mostly... if we assume default constructability, it
    // becomes a lot easier and we can just grab the mutable field tree and recurse.
    /// Create an object from a byte buffer, this requires the type to be default constructible.
    fn from_le_bytes(src: &[u8]) -> Result<Self, String>
    where
        Self: Sized + Default,
    {
        let mut v: Self = Default::default();
        impl_from_le_bytes(&mut v.fields_as_mut(), src)?;
        Ok(v)
    }
}

//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name

//https://doc.rust-lang.org/rust-by-example/macros/designators.html

/// Helper macro to create the implementations for the primitive scalar types.
macro_rules! make_inspectable {
    ($a:ty, $as_mut: path, $as_ref: path) => {
        impl StructHelper for $a {
            fn fields_as_mut<'a>(&'a mut self) -> FieldMut {
                FieldMut {
                    info: Info {
                        start: 0,
                        length: std::mem::size_of::<$a>(),
                        type_name: std::any::type_name::<$a>(),
                        type_id: std::any::TypeId::of::<$a>(),
                        name: None,
                        element_type: ElementType::Scalar,
                        attrs: std::collections::HashMap::new(),
                    },
                    value: $as_mut(self),
                    children: vec![],
                }
            }

            fn fields_as_ref<'a>(&'a self) -> FieldRef {
                FieldRef {
                    info: Info {
                        start: 0,
                        length: std::mem::size_of::<$a>(),
                        type_name: std::any::type_name::<$a>(),
                        type_id: std::any::TypeId::of::<$a>(),
                        name: None,
                        element_type: ElementType::Scalar,
                        attrs: std::collections::HashMap::new(),
                    },
                    value: $as_ref(self),
                    children: vec![],
                }
            }

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
