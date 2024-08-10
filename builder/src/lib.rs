use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _input = syn::parse_macro_input!(input as syn::DeriveInput);
    // generate the builder code
    let expanded = quote! {
        // the generated code will go here
    };
    TokenStream::from(expanded)
}
