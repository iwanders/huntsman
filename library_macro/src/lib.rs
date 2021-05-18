
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;



fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    println!("Full: {:?}", ast);
    match &ast.data
    {
        syn::Data::Struct(data_struct) => {
            match &data_struct.fields
            {
                syn::Fields::Named(z) =>
                    {
                        //~ println!("Field is named: {:?}", z);
                        for inner_field in &z.named
                        {
                            println!("heh; {:?}", inner_field.ident);
                            match &inner_field.ty
                            {
                                syn::Type::Array(arr) => {println!("Its an array");},
                                syn::Type::Verbatim(v) => {println!("Its an v");},
                                syn::Type::Path(type_path) => {
                                    println!("Its a type_path {:?}",type_path);
                                    println!("Its a {:?}", type_path.path.segments[0].ident);
                                    println!("Its a stringified {:?}", type_path.path.segments[0].ident.to_string());
                                },
                                _ => {println!("Its somethign else : {:?}", &inner_field.ty);},
                            }
                        }
                    },
                syn::Fields::Unnamed(z) =>
                    {
                        //~ println!("Field is Unnamed: {:?}", z);
                    }
                syn::Fields::Unit =>
                    {
                        //~ println!("Field is Unnamed: {:?}", z);
                    }
                
            }
        },
        syn::Data::Enum(data_enum) => {
            println!("Its a Enum");
        },
        syn::Data::Union(data_union) => {
            println!("Its a Union");
        },
    }
    println!("During compile :O {}", name);
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}


#[proc_macro_derive(HelloMacro, attributes(hello))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}



#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    return item;
}
