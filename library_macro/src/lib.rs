// Sort of similar:
// https://github.com/dtolnay/syn/tree/master/examples/heapsize

extern crate proc_macro;

use quote::quote;
extern crate proc_macro2;
use syn;

#[macro_use]
extern crate memoffset;

fn impl_hello_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    println!("Full: {:#?}", input);

    let mut fields: Vec<proc_macro2::TokenStream> = Vec::new();
    let root_struct = &input.ident;

    match &input.data {
        syn::Data::Struct(data_struct) => {
            // println!("Data struct: {:?}", data_struct);
            match &data_struct.fields {
                syn::Fields::Named(z) => {
                    // println!("named field: {:?}", z);
                    for inner_field in &z.named {
                        let mut attributes_addition : String = String::new();
                        if inner_field.attrs.len() != 0
                        {
                            attributes_addition = ":attribute_from:".to_string() + &inner_field.attrs[0].path.segments[0].ident.to_string();
                            println!("attributes: {:?}", inner_field.attrs[0]);
                        }
                        //~ println!("heh; {:?}", inner_field.ident);
                        //~ let name = inner_field.ident.unwrap();
                        println!("name; {:?}", inner_field.ident);
                        let mut name: String = Default::default();
                        let inner_field_ident: &syn::Ident;
                        match &inner_field.ident {
                            Some(ident) => {
                                println!("{:?}", ident.to_string());
                                inner_field_ident = ident;
                                name = ident.to_string();
                            }
                            _ => {
                                panic!("");
                            }
                        }
                        match &inner_field.ty {
                            syn::Type::Array(arr) => {
                                println!("Its an array {:?}", arr);
                                let type_ident = &arr.elem;
                                println!("type ident: {:?}", type_ident);
                                println!("arrlen: {:?}", &arr.len);
                                let count: usize;
                                match &arr.len
                                {
                                    syn::Expr::Lit(lit) => match &lit.lit
                                        {
                                            syn::Lit::Int(intlit) => count = intlit.base10_parse::<usize>().unwrap(),
                                            _ => panic!("Should be an integer"),
                                        },
                                    _ => panic!("expected an integer for array.")
                                }
                                // let count = &arr.len.lit.token;
                                // let count = 1 as usize;
                                //split_at_mut() need to be used here, but it fits the architecture poorly atm.
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
                                // for i in 0..count
                                // {
                                    // fields.push(proc_macro2::TokenStream::from(quote!(
                                        // HelloField{
                                            // value: library::PrimitiveBind::None,
                                            // start: std::mem::size_of::<#type_ident>() * #i,
                                            // length: std::mem::size_of::<#type_ident>(),
                                            // type_name: (stringify!(#type_ident)).to_string(),
                                            // type_id: std::any::TypeId::of::<#type_ident>(),
                                            // name: Some((stringify!(#type_ident)).to_string() + #attributes_addition),
                                            // children: vec!(self.#inner_field_ident[#i].fields())}
                                    // )));
                                // }
                            }
                            syn::Type::Verbatim(v) => {
                                println!("Its an verbatim!? {:?}", v);
                            }
                            syn::Type::Path(type_path) => {
                                println!("Its a type_path {:?}", type_path);
                                println!("Its a {:?}", type_path.path.segments[0].ident);
                                println!(
                                    "Its a stringified {:?}",
                                    type_path.path.segments[0].ident.to_string()
                                );
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
                    println!("Field is Unnamed: {:?}", z);
                }
                syn::Fields::Unit => {
                    println!("Field is Unit");
                }
                SomethingElse => {
                    println!("Else: {:?}", SomethingElse);
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            println!("Its a Enum");
        }
        syn::Data::Union(data_union) => {
            println!("Its a Union");
        }
    }
    println!("During compile :O {}", name);
    //~ let mut fields : String = Default::default();
    let mut gen = quote! {
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
    println!("Output: {:}", gen.to_string());
    gen.into()
}

#[proc_macro_derive(HelloMacro, attributes(hello))]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    //~ let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(input)
}

#[proc_macro_attribute]
pub fn route(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return item;
}
