

// Sort of similar:
// https://github.com/dtolnay/syn/tree/master/examples/heapsize 

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
extern crate proc_macro2;
use syn;



fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    println!("Full: {:?}", ast);

    //~ let mut fields : TokenStream = Default::default();
    //~ fields += "";
    //~ let mut stream = proc_macro::TokenStream::new();
    let mut fields: Vec<proc_macro2::TokenStream> = Vec::new();
    //~ stream.extend((0..input.field_count).fold(vec![], |mut state:Vec<proc_macro::TokenStream>, i| {
        //~ let field_name_str = format!("{}_{}", input.field_name, i);
        //~ let field_name = Ident::new(&field_name_str, Span::call_site());
        //~ let field_type = input.field_type.clone();
        //~ state.push(quote!(pub #field_name: #field_type,
        //~ ).into());
        //~ state
        //~ state
    //~ }).into_iter());
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
                            //~ println!("heh; {:?}", inner_field.ident);
                            //~ let name = inner_field.ident.unwrap();
                            println!("name; {:?}", inner_field.ident);
                            let mut name: String = Default::default();
                            match &inner_field.ident
                            {
                                Some(ident) => {println!("{:?}", ident.to_string());
                                name = ident.to_string();},
                                _ => {},
                            }
                            match &inner_field.ty
                            {
                                syn::Type::Array(arr) => {println!("Its an array");},
                                syn::Type::Verbatim(v) => {println!("Its an v");},
                                syn::Type::Path(type_path) => {
                                    println!("Its a type_path {:?}",type_path);
                                    println!("Its a {:?}", type_path.path.segments[0].ident);
                                    println!("Its a stringified {:?}", type_path.path.segments[0].ident.to_string());
                                    let n = type_path.path.segments[0].ident.to_string();
                                    fields.push(proc_macro2::TokenStream::from(quote!(
                                        HelloField{start: 0, length: 0, unit: (#n).to_string(), name: (#name).to_string()}
                                    )));
                                    //~ fields += ", ";
                                },
                                _ => {println!("Its somethign else : {:?}", &inner_field.ty);},
                            }
                        }
                    },
                syn::Fields::Unnamed(z) =>
                    {
                        println!("Field is Unnamed: {:?}", z);
                    }
                syn::Fields::Unit =>
                    {
                        println!("Field is Unit");
                    }
                SomethingElse =>
                {
                        println!("Else: {:?}", SomethingElse);
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
    //~ let mut fields : String = Default::default();
    let mut gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
            fn fields() -> Vec<HelloField>
            {
                return vec!(#(#fields),*);
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
