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

pub trait Inspectable {
    fn type_name(&self) -> &'static str {
        "no type"
    }
    // fn name(&self) -> Option<&'static str>;
    // fn element_type(&self) -> ElementType;

    // The runtime fields;
    fn offset(&self) -> usize;
    fn length(&self) -> usize;

    fn as_any(&self) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn elements(&self) -> Vec<Box<dyn Inspectable>> {
        vec![]
    }

    // The static fields:
    fn fields() -> &'static [&'static dyn Inspectable]
    where
        Self: Sized,
    {
        return &[];
    }

    // fn clone_box(&self) -> Box<dyn Inspectable>;
}

pub trait Wireable {
    // Convert from and two bytes.
}

#[derive(Default, Clone, Debug)]
struct PrimitiveHelper {
    type_name: &'static str,
    start: usize,
    length: usize,
}

impl Inspectable for PrimitiveHelper {
    fn type_name(&self) -> &'static str {
        self.type_name
    }
    fn offset(&self) -> usize {
        self.start
    }
    fn length(&self) -> usize {
        self.length
    }

    fn fields() -> &'static [&'static dyn Inspectable]
    where
        Self: Sized,
    {
        const A: PrimitiveHelper = PrimitiveHelper {
            start: 0,
            length: 0,
            type_name: "primitive_helper",
        };
        return &[&A];
    }
}

impl std::fmt::Debug for dyn Inspectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut opener = f.debug_struct(self.type_name());
        for k in self.elements() {
            opener.field(
                k.type_name(),
                &format_args!("{}  #{}@{}", k.type_name(), k.length(), k.offset()),
            );
        }
        opener.finish()
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
struct Z {
    f: u8,
}

impl Inspectable for u8 {
    fn type_name(&self) -> &'static str {
        "u8"
    }
    fn offset(&self) -> usize {
        0
    }
    fn length(&self) -> usize {
        std::mem::size_of::<u8>()
    }
}

impl Inspectable for u16 {
    fn type_name(&self) -> &'static str {
        "u16"
    }
    fn offset(&self) -> usize {
        0
    }
    fn length(&self) -> usize {
        std::mem::size_of::<u16>()
    }
}
impl Inspectable for u32 {
    fn type_name(&self) -> &'static str {
        "u32"
    }
    fn offset(&self) -> usize {
        0
    }
    fn length(&self) -> usize {
        std::mem::size_of::<u32>()
    }
}

impl Inspectable for Z {
    fn type_name(&self) -> &'static str {
        "ZZZZ"
    }
    fn offset(&self) -> usize {
        0
    }
    fn length(&self) -> usize {
        std::mem::size_of::<u8>()
    }

    fn elements(&self) -> Vec<Box<dyn Inspectable>> {
        vec![Box::new(self.f)]
    }
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C, packed)]
struct Pancakes {
    first_char: u16,
    an_uint: u32,
    x: Z,
}

impl Inspectable for Pancakes {
    fn offset(&self) -> usize {
        0
    }
    fn length(&self) -> usize {
        std::mem::size_of::<Pancakes>()
    }

    fn elements(&self) -> Vec<Box<dyn Inspectable>> {
        vec![
            Box::new(self.first_char),
            Box::new(self.an_uint),
            Box::new(self.x),
        ]
    }

    fn fields() -> &'static [&'static dyn Inspectable]
    where
        Self: Sized,
    {
        // offsetoff in const context requires nightly, hardcoded here for now.
        const A: PrimitiveHelper = PrimitiveHelper {
            start: 0,
            length: std::mem::size_of::<u16>(),
            type_name: "first_char",
        };
        const B: PrimitiveHelper = PrimitiveHelper {
            start: 2,
            length: std::mem::size_of::<u32>(),
            type_name: "an_uint",
        };
        const C: Z = Z { f: 0 }; // :|
        return &[&A, &B, &C];
    }
}

#[test]
fn test_starts() {
    let stack: Pancakes = Default::default();

    println!("Offset: {}", stack.offset());
    println!("length: {}", stack.length());
    // println!("length: {:?}", stack.element());
    println!("Inspectable fields: {:?}", (&stack as &dyn Inspectable));
    println!("Pancakes fields: {:?}", Pancakes::fields());
    // println!("z: {:?}", z.elements());
    // let bound = stack.fields_as_mut();

    // assert_eq!(
    // offset_of!(Pancakes, first_char),
    // bound.elements[0].info.start
    // );
    // assert_eq!(offset_of!(Pancakes, an_uint), bound.elements[1].info.start);
}
