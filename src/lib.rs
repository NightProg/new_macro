

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;


#[proc_macro_derive(New)]
/// This macro will generate a new function for the struct that will allow you to create a new instance of the struct.
/// 
/// # Example
/// ```rust
/// use new_macro::New;
/// 
/// #[derive(New)]
/// struct MyStruct {
///    a: u32,
///    b: bool,
///    c: i32
///   // Other attributes...
/// }
/// 
/// let instance = MyStruct::new(/* a */ 12, /* b */ true, /* c */ 32);
/// ```
pub fn new_derive(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as DeriveInput);

    let name = &tokens.ident;

    let data = match &tokens.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match &data.fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("Only named fields are supported"),
    };


    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut field_values = Vec::new();

    for field in fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let field_value = quote! { #field_name };

        field_names.push(field_name);
        field_types.push(field_type);
        field_values.push(field_value);
    }


    let expanded = quote! {
        impl #name {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names: #field_values),*
                }
            }
        }
    };

    expanded.into()
}


