// use struct_helper::*;

pub use memoffset::*;

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
struct Z {
    f: u8,
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C, packed)]
struct Pancakes {
    first_char: u16,
    an_uint: u32,
    x: Z,
}
// https://github.com/rust-lang/rust/issues/46043
// Probably means the entire current setup with the references to primitives is ehm, busted.
// We do want to keep the functionality to read from a byte array though...

// https://doc.rust-lang.org/reference/items/traits.html#object-safety

/*
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

*/

pub trait Convertible {
    fn to_le_bytes(self, dest: &mut [u8]) -> Result<(), String>;
    fn from_le_bytes(src: &[u8]) -> Result<Self, String> where Self: Sized;
}

pub trait Inspectable
{
    // fn type_name(&self) -> &'static str;
    // fn name(&self) -> Option<&'static str>;
    // fn element_type(&self) -> ElementType;

    fn offset(&self) -> usize;
    fn length(&self) -> usize;
    // fn from_le_bytes(&mut self, src: &[u8]) -> Result<usize, String>;
    // fn to_le_bytes(self, dest: &mut [u8]) -> Result<usize, String>;

    fn children(&self) -> Vec<Box<dyn Inspectable>>;
}

#[derive(Default, Clone)]
struct PrimitiveHelper
{
    start: usize,
    length: usize,
}
impl Inspectable for PrimitiveHelper
{
    fn offset(&self) -> usize
    {
        self.start
    }
    fn length(&self) -> usize
    {
        self.length
    }
    fn children(&self) -> Vec<Box<dyn Inspectable>>
    {
        vec!()
    }
}

impl std::fmt::Debug for dyn Inspectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut opener = f.debug_struct("Thing");
        for k in self.children().iter()
        {
            opener.field("f", &format_args!("#{}@{}", k.length(), k.offset()));
        }
        opener.finish()
    }
}


impl Inspectable for Pancakes
{
    fn offset(&self) -> usize
    {
        0
    }
    fn length(&self) -> usize
    {
        std::mem::size_of::<Pancakes>()
    }
    fn children(&self) -> Vec<Box<dyn Inspectable>>
    {
        vec!(
            Box::new(PrimitiveHelper{start: 0, length: 0, ..Default::default()}),
            Box::new(PrimitiveHelper{start: 1, length: 2, ..Default::default()}),
        )
    }
}


#[test]
fn test_starts() {
    let mut stack: Pancakes = Default::default();

    let z : Box<dyn Inspectable> = Box::new(stack);
    println!("Offset: {}", stack.offset());
    println!("length: {}", stack.length());
    println!("length: {:?}", stack.children());
    println!("z: {:?}", z);
    // let bound = stack.fields_as_mut();

    // assert_eq!(
        // offset_of!(Pancakes, first_char),
        // bound.children[0].info.start
    // );
    // assert_eq!(offset_of!(Pancakes, an_uint), bound.children[1].info.start);
}
