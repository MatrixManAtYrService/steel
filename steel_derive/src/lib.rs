extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(Steel)]
pub fn derive_steel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    if let Data::Struct(_) = &input.data {
        let gen = quote! {
            impl steel::rvals::Custom for #name {}
        };

        return gen.into();
    };

    let output = quote! { #input };
    output.into()
}
