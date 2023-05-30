extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;



#[proc_macro_derive(New, attributes(default))]
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


    let mut args = Vec::new();

    let mut field_instance = Vec::new();
    let mut args_with_default = Vec::new();
    let mut field_instance_with_default_value = Vec::new();
    let mut methods = Vec::new();

    for field in fields.named.clone() {
        let default_value = field.attrs.into_iter()
            .filter(|attr| {
                attr.meta
                    .path()
                    .is_ident("default")
            }).map(|attr| {
                let value = attr.meta
                    .require_name_value()
                    .expect("Expected a name value pair");

                value.clone().value

            }).nth(0);

        let type_ = field.ty;
        let name = field.ident.unwrap();

        match default_value {
            Some(default) => {
                args_with_default.push(quote! {
                    #name: #type_
                });
                field_instance_with_default_value.push(quote! {
                    #name
                });
                field_instance.push(quote! {
                    #name: #default
                });

                methods.push(quote! {
                    pub fn with_#name(&self, value: #ty) -> Self {
                        Self {
                            #name: value,
                            ..self
                        }
                    }
                });
            },
            None => {
                args.push(quote! {
                    #name: #type_
                });

                field_instance.push(quote! {
                    #name
                });


            }
        };

    }

    args_with_default.extend(args.clone());
    field_instance_with_default_value.extend(field_instance.clone());

    let body = quote! {
        impl #name {
            pub fn new(#(#args),*) -> Self {
                Self {
                    #(#field_instance),*
                }
            }

            pub fn new_with_default(#(#args_with_default),*) -> Self {
                Self {
                    #(#field_instance_with_default_value),*
                }
            }

            #(#methods)*

        }
    };

    body.into()
}


