///! This actually implements the derive macro for the struct_helper.
// Sort of similar:
// https://github.com/dtolnay/syn/tree/master/examples/heapsize
extern crate proc_macro;
use quote::quote;
extern crate proc_macro2;
use syn;

// https://github.com/rust-lang/rust/issues/48956

// Enums treated as unions... would be cool if we could handle that elegantly...
// perhaps with a 'denoted by' field specifying which of the enums is active?
// https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html

// Outputs a tokenstream in the shape of [(&'static str, &'static str)], as well as the hashmap for
// the tokens.
fn process_str_attributes(
    list: &Vec<syn::Attribute>,
) -> (
    proc_macro2::TokenStream,
    std::collections::HashMap<String, String>,
) {
    let mut attribute_str_pairs: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for option in list.into_iter() {
        let option = option
            .parse_meta()
            .expect("The attribute list optional must be populated");
        match option {
            syn::Meta::List(z) => {
                for x in z.nested {
                    match x {
                        syn::NestedMeta::Meta(meta_thing) => {
                            match meta_thing {
                                syn::Meta::NameValue(meta_name_value) => {
                                    // We have something shapend like `foo = 3` or `foo = "bar"`
                                    // Check if we have a string entry.
                                    match meta_name_value.lit {
                                        syn::Lit::Str(str_lit) => {
                                            // Yes... cool, we can get the name of the path, and extract this literal.
                                            let attribute_name =
                                                meta_name_value.path.segments[0].ident.to_string();
                                            let value = str_lit.value();
                                            map.insert(attribute_name.clone(), value.clone());
                                            attribute_str_pairs.push(quote!(
                                                (#attribute_name, #value)
                                            ))
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {} // path, list etc.
                            }
                        }
                        _ => {
                            // Literal without a path & equal sign etc.
                        }
                    }
                }
            }
            _ => {}
        }
    }
    // Concatenate the extracted pairs.
    let res = quote!(#(#attribute_str_pairs),*);
    (res, map)
}

enum TraitToImplement {
    ToBytes,
    FromBytes,
    Inspectable,
    StructHelper,
}

/// The function that actually generates the code for this derived type.
fn impl_struct_helper_macro(
    input: proc_macro::TokenStream,
    trait_to_implement: TraitToImplement,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    // println!("input struct: {:#?}", input);

    // https://doc.rust-lang.org/reference/attributes.html
    // Seems it MUST be a literal expression; https://doc.rust-lang.org/reference/expressions/literal-expr.html
    // Lets, for sake of simplicity, just handle string inputs now.
    let (outer_attribute_tokens, _outer_map) = process_str_attributes(&input.attrs);

    let mut fields_static: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut inspectable_fields: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut fields_to_bytes: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut fields_from_bytes: Vec<proc_macro2::TokenStream> = Vec::new();
    let root_struct = &input.ident;

    match &input.data {
        syn::Data::Struct(data_struct) => {
            // println!("Data struct: {:#?}", data_struct);
            match &data_struct.fields {
                syn::Fields::Named(z) => {
                    for inner_field in &z.named {
                        let (inner_attribute_tokens, inner_map) =
                            process_str_attributes(&inner_field.attrs);

                        let name: String;
                        let inner_field_ident: &syn::Ident;
                        match &inner_field.ident {
                            Some(ident) => {
                                inner_field_ident = ident;
                                name = ident.to_string();
                            }
                            _ => {
                                panic!("No identifier for this field?");
                            }
                        }
                        if name.starts_with("_") {
                            continue;
                        }
                        match inner_map.get("ignore") {
                            Some(v) => {
                                if v == "true" {
                                    continue;
                                }
                            }
                            None => {}
                        }

                        match &inner_field.ty {
                            syn::Type::Array(arr) => {
                                // Element type
                                let type_ident = &arr.elem;
                                let arr_len = &arr.len;

                                let info = quote!(
                                    struct_helper::Info{
                                        start: offset_of!(#root_struct, #inner_field_ident),
                                        length: std::mem::size_of::<#type_ident>() *#arr_len ,
                                        type_name: stringify!(#type_ident),
                                        type_id: std::any::TypeId::of::<#type_ident>(),
                                        name: Some((#name)),
                                        element_type: struct_helper::ElementType::Array,
                                        attrs: [#inner_attribute_tokens].iter().cloned().collect(),
                                    }
                                );

                                fields_static.push(proc_macro2::TokenStream::from(quote!(
                                        struct_helper::Field{
                                            info: #info,
                                            children: (0..#arr_len).map(|i|
                                                {
                                                    let mut fields = <#type_ident as StructHelper>::fields();
                                                    fields.info.start = i * std::mem::size_of::<#type_ident>();
                                                    fields
                                                }).collect::<Vec<struct_helper::Field>>(),
                                        }
                                    )
                                ));

                                inspectable_fields.push(proc_macro2::TokenStream::from(quote!(
                                    Box::new(struct_helper::SimpleInspectable{
                                        start: offset_of!(#root_struct, #inner_field_ident),
                                        length: std::mem::size_of::<#type_ident>() *#arr_len ,
                                        type_name:  stringify!(#type_ident),
                                        name: Some((#name).to_string()),
                                        element_type: struct_helper::ElementType::Array,
                                        attrs: [#inner_attribute_tokens].iter().cloned().collect(),
                                        elements: (0..#arr_len).map(|i|
                                                {
                                                    let mut entry = <#type_ident as Inspectable>::inspect();
                                                    entry.set_start(i * std::mem::size_of::<#type_ident>());
                                                    entry
                                                }).collect::<Vec<Box<dyn Inspectable>>>(),
                                        ..Default::default()
                                    })
                                )));

                                fields_to_bytes.push(proc_macro2::TokenStream::from(quote!(
                                    for i in 0..#arr_len
                                    {
                                        let s = offset_of!(#root_struct, #inner_field_ident) + i * std::mem::size_of::<#type_ident>();
                                        // let e = std::mem::size_of::<#type_ident>() + s;
                                        // Copy against reference from packed struct.
                                        let tmp = self.#inner_field_ident[i];
                                        let buff = ToBytes::to_bytes(&tmp, endianness)?;
                                        for i in 0..buff.len()
                                        {
                                            // dest[s..e] = [..];
                                            dest[s + i] = buff[i];
                                        }
                                    }
                                )));
                                fields_from_bytes.push(proc_macro2::TokenStream::from(quote!(
                                    for i in 0..#arr_len
                                    {

                                        let s = offset_of!(#root_struct, #inner_field_ident) + i * std::mem::size_of::<#type_ident>();
                                        let e = std::mem::size_of::<#type_ident>() + s;
                                        x.#inner_field_ident[i]  = < #type_ident as FromBytes >::from_bytes(&src[s..e], endianness).expect("yes");
                                    }
                                )));
                            }
                            syn::Type::Verbatim(v) => {
                                panic!("Its an verbatim!? {:?}", v); // Shouldn't really happen in a struct derive
                            }
                            syn::Type::Path(type_path) => {
                                // Path, a name to another type, or a primitive.
                                // println!("Its a type_path {:#?}", type_path);
                                let type_ident = &type_path.path.segments[0].ident;
                                let n = type_ident.to_string();

                                let info = quote!(
                                struct_helper::Info{
                                    start: offset_of!(#root_struct, #inner_field_ident),
                                    length: std::mem::size_of::<#type_ident>(),
                                    type_name: #n,
                                    type_id: std::any::TypeId::of::<#type_ident>(),
                                    name: Some((#name)),
                                    element_type: struct_helper::ElementType::Path,
                                    attrs: [#inner_attribute_tokens].iter().cloned().collect(),
                                });

                                fields_static.push(proc_macro2::TokenStream::from(quote!(
                                    struct_helper::Field{
                                        info: #info,
                                        children: vec!(<#type_ident as StructHelper>::fields())}
                                )));

                                inspectable_fields.push(proc_macro2::TokenStream::from(quote!(
                                    Box::new(struct_helper::SimpleInspectable{
                                        start: offset_of!(#root_struct, #inner_field_ident),
                                        length: std::mem::size_of::<#type_ident>(),
                                        type_name: #n,
                                        name: Some((#name).to_string()),
                                        element_type: struct_helper::ElementType::Path,
                                        attrs: [#inner_attribute_tokens].iter().cloned().collect(),
                                        elements: <#type_ident as Inspectable>::fields(),
                                        ..Default::default()
                                    })
                                )));

                                fields_to_bytes.push(proc_macro2::TokenStream::from(quote!(
                                    {
                                        let s = offset_of!(#root_struct, #inner_field_ident);
                                        let e = std::mem::size_of::<#type_ident>() + s;
                                        // Copy against reference from packed struct.
                                        let tmp = self.#inner_field_ident;
                                        let buff = ToBytes::to_bytes(&tmp, endianness)?;
                                        // dest[s..e] = buff[..];
                                        for i in 0..buff.len()
                                        {
                                            dest[s + i] = buff[i];
                                        }
                                    }
                                )));
                                fields_from_bytes.push(proc_macro2::TokenStream::from(quote!(
                                    {
                                        let s = offset_of!(#root_struct, #inner_field_ident);
                                        let e = std::mem::size_of::<#type_ident>() + s;
                                        x.#inner_field_ident  = < #type_ident as FromBytes >::from_bytes(&src[s..e], endianness).expect("yes");
                                    }
                                )));
                            }
                            _ => {
                                println!("Its somethign else : {:?}", &inner_field.ty);
                            }
                        }
                    }
                }
                syn::Fields::Unnamed(z) => {
                    panic!("Field is Unnamed in struct?: {:?}", z);
                }
                syn::Fields::Unit => {
                    panic!("Field is Unit");
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            panic!("Enums aren't supported. {:?}", data_enum);
        }
        syn::Data::Union(data_union) => {
            panic!("Unions aren't supported. {:?}", data_union);
        }
    }
    let info = quote!(
        struct_helper::Info{
             start: 0,
             length: std::mem::size_of::<#name>(),
             type_name: stringify!(#name),
             type_id: std::any::TypeId::of::<#name>(),
             name: Some(stringify!(#name)),
             element_type: struct_helper::ElementType::Other,
             attrs: [#outer_attribute_tokens].iter().cloned().collect(),
        }
    );
    let trait_struct_helper = quote! {
        impl struct_helper::StructHelper for #name {
            fn fields() -> struct_helper::Field {
                struct_helper::Field {
                     info: #info,
                     children: vec!(#(#fields_static),*)
                }
            }
        };
    };

    let trait_inspectable = quote! {
        impl struct_helper::Inspectable for #name
        {
            fn start(&self) -> usize
            {
                0
            }
            fn length(&self) -> usize
            {
                std::mem::size_of::<#name>()
            }
            fn type_name(&self) -> &'static str
            {
                stringify!(#name)
            }
            fn name(&self) -> Option<String>
            {
                Some(stringify!(#name).to_string())
            }
            fn element_type(&self) -> struct_helper::ElementType
            {
                struct_helper::ElementType::Other
            }
            fn fields() -> Vec<Box<dyn Inspectable>>
            {
                vec!(#(#inspectable_fields),*)
            }
            fn elements(&self) -> Vec<Box<dyn Inspectable>>
            {
                <#name>::fields()
            }
            fn clone_box(&self) -> Box<dyn struct_helper::Inspectable>
            {
                Box::new(self.clone())
            }

            fn attrs(&self) -> std::collections::HashMap<&'static str, &'static str> 
            {
                [#outer_attribute_tokens].iter().cloned().collect()
            }

            fn inspect() -> Box<dyn struct_helper::Inspectable>
            {
                Box::new(struct_helper::SimpleInspectable{
                    start: 0,
                    length: std::mem::size_of::<#name>(),
                    type_name: stringify!(#name),
                    name: Some(stringify!(#name).to_string()),
                    element_type: struct_helper::ElementType::Path,
                    attrs: [#outer_attribute_tokens].iter().cloned().collect(),
                    elements: #name::fields(),
                    ..Default::default()
                })
            }

        };
    };

    let trait_to_bytes = quote! {
    impl struct_helper::ToBytes for #name
    {
        fn to_bytes(&self, endianness: Endianness) -> Result<Vec<u8>, String>
        {
            let mut dest : [u8; std::mem::size_of::<#name>()] = [0; std::mem::size_of::<#name>()];
            // Todo; restore checking here...
            // if (#name::fields()).info.length > dest.len()
            // {
                // return Err(format!("Type is {} long, doesn't fit into {} provided.", (#name::fields()).info.length, dest.len()));
            // }
            #(#fields_to_bytes);*
            Ok(dest.to_vec())
        }
    }};

    let trait_from_bytes = quote! {

        impl struct_helper::FromBytes for #name
        {
            fn from_bytes(src: &[u8], endianness: Endianness) -> Result<Self, String> where Self: Sized + Default
            {
                let mut x: #name = Default::default();
                // if (#name::fields()).info.length > src.len()
                // {
                    // return Err(format!("Type is {} long, only {} provided.", (#name::fields()).info.length, src.len()));
                // }
                #(#fields_from_bytes);*
                Ok(x)
            }
        }
    };
    let res: proc_macro::TokenStream = match trait_to_implement {
        TraitToImplement::ToBytes => trait_to_bytes.into(),
        TraitToImplement::FromBytes => trait_from_bytes.into(),
        TraitToImplement::StructHelper => trait_struct_helper.into(),
        TraitToImplement::Inspectable => trait_inspectable.into(),
    };
    // println!("Output: {:}", res.to_string());
    res
}

#[doc = "This implements the derive macro for the struct helper.

Any fields that start with an undercore (`_`) are ignored and not traversed into.

Attributes can be added with `#[struct_helper(my_key = \"Pi!\")]`, the keys (like `my_key`) can be
anything, the value must always be string at the moment.

The following pre-defined keys exist:

- `ignore`, if the value for this is `\"true\"`, this field is ignored as if it started with an
underscore.

So `#[struct_helper(foo = \"alpha\", bar = \"bravo\")]` will result in an `attrs` HashMap of 
`{\"foo\": \"alpha\", \"bar\": \"bravo\"}`.
"]
// #[proc_macro_derive(StructHelper)]
// pub fn struct_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // impl_struct_helper_macro(input, TraitToImplement::StructHelper)
// }

#[proc_macro_derive(FromBytes)]
pub fn from_bytes_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_struct_helper_macro(input, TraitToImplement::FromBytes)
}
#[proc_macro_derive(ToBytes)]
pub fn to_bytes_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_struct_helper_macro(input, TraitToImplement::ToBytes)
}

#[proc_macro_derive(Inspectable, attributes(inspect))]
pub fn inspectable_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_struct_helper_macro(input, TraitToImplement::Inspectable)
}
