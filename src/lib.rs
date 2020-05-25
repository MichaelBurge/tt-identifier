extern crate proc_macro;

use std::hash::{Hash, Hasher};

use proc_macro::TokenStream;
use proc_macro2::Span;
use std::iter::FromIterator;
use syn::parse_quote;

#[proc_macro]
pub fn identify(input: TokenStream) -> TokenStream {
    //! Turns the input into something usable as a Rust identifier.
    let ident = syn::Ident::new(&mk_identifier(input), Span::call_site());
    let sym: proc_macro2::TokenStream = parse_quote!(#ident);
    TokenStream::from(sym)
}

#[proc_macro]
pub fn identify_string(input: TokenStream) -> TokenStream {
    //! Same as identify, but outputs a string instead of identifier.
    let s = mk_identifier(input);
    let lit = proc_macro::Literal::string(&*s);
    proc_macro::TokenTree::Literal(lit).into()
}

fn mk_identifier(input: TokenStream) -> String {
    let s = input.to_string();
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    let id = h.finish();
    format!("x_{}", id)
}
