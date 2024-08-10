use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // get the name of the input struct
    let name = &input.ident;
    let new_name = Ident::new(&format!("{}Builder", name), Span::call_site());
    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };
    let fields = match data.fields {
        syn::Fields::Named(fields) => fields.named,
        _ => panic!("Only named fields are supported"),
    };
    // add option to each field
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let fields_option_define = quote! {
        #(
            #idents: Option<#types>,
        )*
    };
    let fields_option_init = quote! {
        #(
            #idents: None,
        )*
    };
    // generate the builder code
    let expanded = quote! {
        impl #name {
            pub fn builder() -> #new_name {
                #new_name {
                    #fields_option_init
                }
            }
        }
        pub struct #new_name {
            #fields_option_define
        }

    };
    TokenStream::from(expanded)
}
