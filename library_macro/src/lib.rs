// Sort of similar:
// https://github.com/dtolnay/syn/tree/master/examples/heapsize

extern crate proc_macro;

use quote::quote;
extern crate proc_macro2;
use syn;

// https://github.com/rust-lang/rust/issues/48956
extern crate memoffset;

// Enums treated as unions... would be cool if we could handle that elegantly...
// perhaps with a 'denoted by' field specifying which of the enums is active?
// https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html

// Outputs a tokenstream in the shape of [(&'static str, &'static str)]
fn process_str_attributes(list: &Vec<syn::Attribute>) -> proc_macro2::TokenStream
{
    let mut attribute_str_pairs: Vec<proc_macro2::TokenStream> = Vec::new();

    for option in list.into_iter() {
        let option = option.parse_meta().expect("The attribute list optional must be populated");
        match option {
            syn::Meta::List(z) =>
            {
                for x in z.nested
                {
                    match x
                    {
                        syn::NestedMeta::Meta(meta_thing) =>
                        {
                            match meta_thing
                            {
                                syn::Meta::NameValue(meta_name_value) => {
                                    // We have something shapend like `foo = 3` or `foo = "bar"` 
                                    // Check if we have a string entry.
                                    match meta_name_value.lit
                                    {
                                        syn::Lit::Str(str_lit) =>
                                        {
                                            // Yes... cool, we can get the name of the path, and extract this literal.
                                            let attribute_name = meta_name_value.path.segments[0].ident.to_string();
                                            let value = str_lit.value();
                                            attribute_str_pairs.push(quote!(
                                                (#attribute_name, #value)
                                            ))
                                        },
                                        _ => {},
                                    }
                                
                                },
                                _ => {}, // path, list etc.
                                
                            }
                        }
                        _ => 
                        {
                            // Literal without a path & equal sign etc.
                        }
                    }
                }
            }
            _=> {},
        }
    }
    // Concatenate the extracted pairs.
    let res = quote!(#(#attribute_str_pairs),*);
    res
}

fn impl_inspectable_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    // println!("input struct: {:#?}", input);

    // https://doc.rust-lang.org/reference/attributes.html
    // Seems it MUST be a literal expression; https://doc.rust-lang.org/reference/expressions/literal-expr.html
    // Lets, for sake of simplicity, just handle string inputs now.
    let outer_attribute_tokens = process_str_attributes(&input.attrs);


    let mut fields_for_mut: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut fields_for_ref: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut fields_static: Vec<proc_macro2::TokenStream> = Vec::new();
    let root_struct = &input.ident;

    match &input.data {
        syn::Data::Struct(data_struct) => {
            // println!("Data struct: {:#?}", data_struct);
            match &data_struct.fields {
                syn::Fields::Named(z) => {
                    for inner_field in &z.named {
                        let inner_attribute_tokens = process_str_attributes(&inner_field.attrs);

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
                        match &inner_field.ty {
                            syn::Type::Array(arr) => {
                                // Element type
                                let type_ident = &arr.elem;
                                let arr_len = &arr.len;

                                let info = quote!(
                                    library::Info{
                                        start: offset_of!(#root_struct, #inner_field_ident),
                                        length: std::mem::size_of::<#type_ident>() *#arr_len ,
                                        type_name: stringify!(#type_ident),
                                        type_id: std::any::TypeId::of::<#type_ident>(),
                                        name: Some((#name)),
                                        attrs: [#inner_attribute_tokens].iter().cloned().collect(),
                                    }
                                );
                                // Create the fields for this array, unwrapping the internals.
                                fields_for_mut.push(proc_macro2::TokenStream::from(quote!(
                                        library::FieldMut{
                                            value: library::MutRef::None,
                                            info: #info,
                                            children: self.#inner_field_ident.iter_mut().enumerate().map(|(i, mut x)|
                                                {
                                                    let mut fields = x.fields_as_mut();
                                                    fields.info.start = i * std::mem::size_of::<#type_ident>();
                                                    fields
                                                }).collect::<Vec<library::FieldMut>>(),
                                        }
                                    )
                                ));
                                fields_for_ref.push(proc_macro2::TokenStream::from(quote!(
                                        library::FieldRef{
                                            value: library::Ref::None,
                                            info: #info,
                                            children: self.#inner_field_ident.iter().enumerate().map(|(i, mut x)|
                                                {
                                                    let mut fields = x.fields_as_ref();
                                                    fields.info.start = i * std::mem::size_of::<#type_ident>();
                                                    fields
                                                }).collect::<Vec<library::FieldRef>>(),
                                        }
                                    )
                                ));
                                fields_static.push(proc_macro2::TokenStream::from(quote!(
                                        library::Field{
                                            info: #info,
                                            children: (0..#arr_len).map(|i|
                                                {
                                                    let mut fields = <#type_ident as Inspectable>::fields();
                                                    fields.info.start = i * std::mem::size_of::<#type_ident>();
                                                    fields
                                                }).collect::<Vec<library::Field>>(),
                                        }
                                    )
                                ));
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
                                        library::Info{
                                            start: offset_of!(#root_struct, #inner_field_ident),
                                            length: std::mem::size_of::<#type_ident>(),
                                            type_name: #n,
                                            type_id: std::any::TypeId::of::<#type_ident>(),
                                            name: Some((#name)),
                                            attrs: [#inner_attribute_tokens].iter().cloned().collect(),
                                        });

                                fields_for_mut.push(proc_macro2::TokenStream::from(quote!(
                                    library::FieldMut{
                                        value: library::MutRef::None,
                                        info: #info,
                                        children: vec!(self.#inner_field_ident.fields_as_mut())}
                                )));
                                fields_for_ref.push(proc_macro2::TokenStream::from(quote!(
                                    library::FieldRef{
                                        value: library::Ref::None,
                                        info: #info,
                                        children: vec!(self.#inner_field_ident.fields_as_ref())}
                                )));
                                fields_static.push(proc_macro2::TokenStream::from(quote!(
                                    library::Field{
                                        info: #info,
                                        children: vec!(<#type_ident as Inspectable>::fields())}
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
        library::Info{
             start: 0,
             length: std::mem::size_of::<#name>(),
             type_name: stringify!(#name),
             type_id: std::any::TypeId::of::<#name>(),
             name: Some(stringify!(#name)),
             attrs: [#outer_attribute_tokens].iter().cloned().collect(),
        }
    );
    let gen = quote! {
        impl library::Inspectable for #name {
            fn fields_as_mut<'a>(&'a mut self) -> library::FieldMut
            {
                return library::FieldMut{
                             value: library::MutRef::None,
                             info: #info,
                             children: vec!(#(#fields_for_mut),*)};
            }

            fn fields_as_ref<'a>(&'a self) -> library::FieldRef
            {
                return library::FieldRef{
                             value: library::Ref::None,
                             info: #info,
                             children: vec!(#(#fields_for_ref),*)};
            }

            fn fields() -> library::Field {
                library::Field {
                     info: #info,
                     children: vec!(#(#fields_static),*)
                }
            }
        }
    };
    // println!("Output: {:}", gen.to_string());
    gen.into()
}

#[proc_macro_derive(Inspectable, attributes(hello))]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_inspectable_macro(input)
}
