// use struct_helper::*;

pub use memoffset::*;

// https://github.com/rust-lang/rust/issues/46043
// Probably means the entire current setup with the references to primitives is ehm, busted.
// We do want to keep the functionality to read from a byte array though...

// https://doc.rust-lang.org/reference/items/traits.html#object-safety

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


pub trait Inspectable
{
    fn type_name(&self) -> &'static str
    {
        "no type"
    }
    // fn name(&self) -> Option<&'static str>;
    // fn element_type(&self) -> ElementType;

    // The runtime fields;
    fn offset(&self) -> usize;
    fn length(&self) -> usize;

    // fn from_le_bytes(&mut self, src: &[u8]) -> Result<usize, String>;
    // fn to_le_bytes(&self, dest: &mut [u8]) -> Result<usize, String>
    // {
        // Ok(0)
    // }

    fn elements(&self) -> Vec<Box<dyn Inspectable>>;


    // The static fields:
    fn fields() -> &'static [&'static  dyn Inspectable] where Self:Sized
    {
        return &[];
    }
    fn foo() -> Self where Self: Sized;
}

#[derive(Default, Clone, Debug)]
struct PrimitiveHelper
{
    type_name: &'static str,
    start: usize,
    length: usize,
}

impl Inspectable for PrimitiveHelper
{
    fn type_name(&self) -> &'static str
    {
        self.type_name
    }
    fn offset(&self) -> usize
    {
        self.start
    }
    fn length(&self) -> usize
    {
        self.length
    }
    fn elements(&self) -> Vec<Box<dyn Inspectable>>
    {
        vec!()
    }


    fn foo() -> Self
    {
        PrimitiveHelper{start: 0, length: 0, ..Default::default()}
    }

    fn fields() -> &'static [&'static  dyn Inspectable] where Self:Sized
    {
        const A: PrimitiveHelper = PrimitiveHelper{start: 0, length: 0, type_name: "primitive_helper"};
        return &[&A];
    }
}

impl std::fmt::Debug for dyn Inspectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut opener = f.debug_struct(self.type_name());
        for k in self.elements().iter()
        {
            opener.field("f", &format_args!("#{}@{}", k.length(), k.offset()));
        }
        opener.finish()
    }
}



#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
struct Z {
    f: u8,
}


impl Inspectable for Z
{
    fn type_name(&self) -> &'static str
    {
        "ZZZZ"
    }
    fn offset(&self) -> usize
    {
        0
    }
    fn length(&self) -> usize
    {
        std::mem::size_of::<u8>()
    }
    fn elements(&self) -> Vec<Box<dyn Inspectable>>
    {
        vec!()
    }

    fn foo() -> Self
    {
        Z{..Default::default()}
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C, packed)]
struct Pancakes {
    first_char: u16,
    an_uint: u32,
    x: Z,
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
    fn elements(&self) -> Vec<Box<dyn Inspectable>>
    {
        vec!(
            Box::new(PrimitiveHelper{start: offset_of!(Pancakes, first_char), length: std::mem::size_of::<u16>(), type_name: "instantiated first_char", ..Default::default()}),
            Box::new(PrimitiveHelper{start: offset_of!(Pancakes, an_uint), length: std::mem::size_of::<u32>(), type_name: "instantiated an_uint", ..Default::default()}),
            Box::new(self.x),  // this copies self.x, which is kinda mehh...
            
        )
    }

    fn fields() -> &'static [&'static  dyn Inspectable] where Self:Sized
    {
        // offsetoff in const context requires nightly, hardcoded here for now.
        const A: PrimitiveHelper = PrimitiveHelper{start: 0, length: std::mem::size_of::<u16>(), type_name: "first_char"};
        const B: PrimitiveHelper = PrimitiveHelper{start: 2, length: std::mem::size_of::<u32>(), type_name: "an_uint"};
        const C: Z = Z{f:0}; // :|
        return &[&A, &B, &C];
    }


    fn foo() -> Self
    {
        Pancakes{..Default::default()}
    }
}


#[test]
fn test_starts() {
    let mut stack: Pancakes = Default::default();

    let z : Box<dyn Inspectable> = Box::new(stack);



    println!("Offset: {}", stack.offset());
    println!("length: {}", stack.length());
    println!("length: {:?}", stack.elements());
    println!("Pancakes fields: {:?}", Pancakes::fields());
    println!("z: {:?}", z.elements());
    // let bound = stack.fields_as_mut();

    // assert_eq!(
        // offset_of!(Pancakes, first_char),
        // bound.elements[0].info.start
    // );
    // assert_eq!(offset_of!(Pancakes, an_uint), bound.elements[1].info.start);
}
