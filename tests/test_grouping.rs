extern crate proc_macro2;
extern crate syn;

mod features;

#[macro_use]
mod macros;

use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use syn::Expr;

use std::iter::FromIterator;

#[test]
fn test_grouping() {
    let tokens: TokenStream = TokenStream::from_iter(vec![
        TokenTree::Literal(Literal::i32_suffixed(1)),
        TokenTree::Punct(Punct::new('+', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::None,
            TokenStream::from_iter(vec![
                TokenTree::Literal(Literal::i32_suffixed(2)),
                TokenTree::Punct(Punct::new('+', Spacing::Alone)),
                TokenTree::Literal(Literal::i32_suffixed(3)),
            ]),
        )),
        TokenTree::Punct(Punct::new('*', Spacing::Alone)),
        TokenTree::Literal(Literal::i32_suffixed(4)),
    ]);

    assert_eq!(tokens.to_string(), "1i32 +  2i32 + 3i32  * 4i32");

    snapshot!(tokens as Expr);
}
