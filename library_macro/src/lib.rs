// Sort of similar:
// https://github.com/dtolnay/syn/tree/master/examples/heapsize

extern crate proc_macro;

use quote::quote;
extern crate proc_macro2;
use syn;

extern crate memoffset;

fn impl_hello_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let mut fields: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut immutable_fields: Vec<proc_macro2::TokenStream> = Vec::new();
    let root_struct = &input.ident;

    match &input.data {
        syn::Data::Struct(data_struct) => {
            // println!("Data struct: {:#?}", data_struct);
            match &data_struct.fields {
                syn::Fields::Named(z) => {
                    for inner_field in &z.named {
                        let mut attributes_addition: String = String::new();
                        if inner_field.attrs.len() != 0 {
                            attributes_addition = ":attribute_from:".to_string()
                                + &inner_field.attrs[0].path.segments[0].ident.to_string();
                            println!("attributes: {:?}", inner_field.attrs[0]);
                        }

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
                                // Create the fields for this array, unwrapping the internals.
                                fields.push(proc_macro2::TokenStream::from(quote!(
                                        library::MutableField{
                                            value: library::MutRef::None,
                                            start: offset_of!(#root_struct, #inner_field_ident),
                                            length: std::mem::size_of::<#type_ident>() *#arr_len ,
                                            type_name: stringify!(#type_ident),
                                            type_id: std::any::TypeId::of::<#type_ident>(),
                                            name: Some((#name).to_string() + #attributes_addition),
                                            children: self.#inner_field_ident.iter_mut().enumerate().map(|(i, mut x)|
                                                {
                                                    let mut fields = x.fields_as_mut();
                                                    fields.start = i * std::mem::size_of::<#type_ident>();
                                                    fields
                                                }).collect::<Vec<library::MutableField>>(),
                                        }
                                    )
                                ));
                                immutable_fields.push(proc_macro2::TokenStream::from(quote!(
                                        library::Field{
                                            start: offset_of!(#root_struct, #inner_field_ident),
                                            length: std::mem::size_of::<#type_ident>() *#arr_len ,
                                            type_name: stringify!(#type_ident),
                                            type_id: std::any::TypeId::of::<#type_ident>(),
                                            name: Some((#name).to_string() + #attributes_addition),
                                            children: (0..#arr_len).map(|i|
                                                {
                                                    let mut fields = <#type_ident as Inspectable>::fields();
                                                    fields.start = i * std::mem::size_of::<#type_ident>();
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

                                fields.push(proc_macro2::TokenStream::from(quote!(
                                    library::MutableField{
                                        value: library::MutRef::None,
                                        start: offset_of!(#root_struct, #inner_field_ident),
                                        length: std::mem::size_of::<#type_ident>(),
                                        type_name: #n,
                                        type_id: std::any::TypeId::of::<#type_ident>(),
                                        name: Some((#name).to_string() + #attributes_addition),
                                        children: vec!(self.#inner_field_ident.fields_as_mut())}
                                )));
                                immutable_fields.push(proc_macro2::TokenStream::from(quote!(
                                    library::Field{
                                        start: offset_of!(#root_struct, #inner_field_ident),
                                        length: std::mem::size_of::<#type_ident>(),
                                        type_name: #n,
                                        type_id: std::any::TypeId::of::<#type_ident>(),
                                        name: Some((#name).to_string() + #attributes_addition),
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
    let gen = quote! {
        impl library::Inspectable for #name {
            fn fields_as_mut<'a>(&'a mut self) -> library::MutableField
            {
                return library::MutableField{start: 0,
                             value: library::MutRef::None,
                             length: std::mem::size_of::<#name>(),
                             type_name: stringify!(#name),
                             type_id: std::any::TypeId::of::<#name>(),
                             name: Some(stringify!(#name).to_string()),
                             children: vec!(#(#fields),*)};
            }

            fn fields() -> library::Field {
                library::Field {
                     start: 0,
                     length: std::mem::size_of::<#name>(),
                     type_name: stringify!(#name),
                     type_id: std::any::TypeId::of::<#name>(),
                     name: Some(stringify!(#name).to_string()),
                     children: vec!(#(#immutable_fields),*)
                }
            }
        }
    };
    println!("Output: {:}", gen.to_string());
    gen.into()
}

#[proc_macro_derive(Inspectable, attributes(hello))]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_hello_macro(input)
}
