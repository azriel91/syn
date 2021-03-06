extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod features;

#[macro_use]
mod macros;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Lit;

#[test]
fn test_struct() {
    let code = "
        #[derive(Debug, Clone)]
        pub struct Item {
            pub ident: Ident,
            pub attrs: Vec<Attribute>,
        }
    ";

    snapshot!(code as TokenStream);
}

#[test]
fn test_literal_mangling() {
    let code = "0_4";
    let parsed: Lit = syn::parse_str(code).unwrap();
    assert_eq!(code, quote!(#parsed).to_string());
}
