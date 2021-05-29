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
    // println!("Full: {:#?}", input);

    let mut fields: Vec<proc_macro2::TokenStream> = Vec::new();
    let root_struct = &input.ident;

    match &input.data {
        syn::Data::Struct(data_struct) => {
            // println!("Data struct: {:#?}", data_struct);
            match &data_struct.fields {
                syn::Fields::Named(z) => {
                    for inner_field in &z.named {
                        let mut attributes_addition : String = String::new();
                        if inner_field.attrs.len() != 0
                        {
                            attributes_addition = ":attribute_from:".to_string() + &inner_field.attrs[0].path.segments[0].ident.to_string();
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

                                // Create the fields for this array, unwrapping the internals.
                                fields.push(proc_macro2::TokenStream::from(quote!(
                                        HelloField{
                                            value: library::PrimitiveBind::None,
                                            start: 0,
                                            length: std::mem::size_of::<#type_ident>(),
                                            type_name: (stringify!(#type_ident)).to_string(),
                                            type_id: std::any::TypeId::of::<#type_ident>(),
                                            name: Some((#name).to_string() + #attributes_addition),
                                            children: self.#inner_field_ident.iter_mut().map(|mut x| 
                                                    x.fields()).collect::<Vec<HelloField>>(),
                                        }
                                    )
                                ));
                            }
                            syn::Type::Verbatim(v) => {
                                panic!("Its an verbatim!? {:?}", v);  // Shouldn't really happen in a struct derive
                            }
                            syn::Type::Path(type_path) => {
                                // println!("Its a type_path {:#?}", type_path);
                                // println!("Its a {:#?}", type_path.path.segments[0].ident);
                                // println!(
                                    // "Its a stringified {:?}",
                                    // type_path.path.segments[0].ident.to_string()
                                // );
                                let type_ident = &type_path.path.segments[0].ident;
                                let n = type_ident.to_string();

                                fields.push(proc_macro2::TokenStream::from(quote!(
                                        HelloField{
                                            value: library::PrimitiveBind::None,
                                            start: offset_of!(#root_struct, #inner_field_ident),
                                            length: std::mem::size_of::<#type_ident>(),
                                            type_name: (#n).to_string(),
                                            type_id: std::any::TypeId::of::<#type_ident>(),
                                            name: Some((#name).to_string() + #attributes_addition),
                                            children: vec!(self.#inner_field_ident.fields())}
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
        impl library::HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
            fn fields<'a>(&'a mut self) -> HelloField
            {
                return HelloField{start: 0,
                                  value: library::PrimitiveBind::None,
                                  length: std::mem::size_of::<#name>(),
                                  type_name: stringify!(#name).to_string(),
                                  type_id: std::any::TypeId::of::<#name>(),
                                  name: Some(stringify!(#name).to_string()),
                                  children: vec!(#(#fields),*)};
            }
        }
    };
    // println!("Output: {:}", gen.to_string());
    gen.into()
}

#[proc_macro_derive(HelloMacro, attributes(hello))]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_hello_macro(input)
}
