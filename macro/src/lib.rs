extern crate proc_macro;

use proc_macro::*;
use std::iter::once;
mod span;
use crate::span::{gen_random, gen_random_bytes};

/// Create a TokenStream of an identifier out of a string
fn ident(ident: &str) -> TokenStream {
    TokenTree::from(Ident::new(ident, Span::call_site())).into()
}

#[proc_macro]
pub fn const_random(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let Some(tt) = iter.next() else {
        panic!("missing type arg");
    };

    let result = match &tt {
        TokenTree::Ident(id) => {
            let s = id.to_string();
            match s.as_str() {
                "u8" => TokenTree::from(Literal::u8_suffixed(gen_random())).into(),
                "u16" => TokenTree::from(Literal::u16_suffixed(gen_random())).into(),
                "u32" => TokenTree::from(Literal::u32_suffixed(gen_random())).into(),
                "u64" => TokenTree::from(Literal::u64_suffixed(gen_random())).into(),
                "u128" => TokenTree::from(Literal::u128_suffixed(gen_random())).into(),
                "i8" => TokenTree::from(Literal::i8_suffixed(gen_random())).into(),
                "i16" => TokenTree::from(Literal::i16_suffixed(gen_random())).into(),
                "i32" => TokenTree::from(Literal::i32_suffixed(gen_random())).into(),
                "i64" => TokenTree::from(Literal::i64_suffixed(gen_random())).into(),
                "i128" => TokenTree::from(Literal::i128_suffixed(gen_random())).into(),
                "usize" => {
                    // Note: usize does not implement `Random` and follow the pattern above. If it
                    // did, when cross-compiling from a 32-bit host to a 64-bit target,
                    // `usize::random()` would produce a 32-bit random usize which would then be
                    // turned into a suffixed literal (e.g. `0x1234_5678usize`). On the 64-bit
                    // target that literal would always have the upper 32 bits as zero, which would
                    // be bad. Instead we produce code that will generate a 128-bit integer literal
                    // (on the host) and then truncate it to usize (on the target).
                    let value: TokenStream =
                        TokenTree::from(Literal::u128_suffixed(gen_random())).into();
                    let type_cast: TokenStream = [value, ident("as"), ident("usize")]
                        .iter()
                        .cloned()
                        .collect();
                    TokenTree::from(Group::new(Delimiter::Parenthesis, type_cast)).into()
                }
                "isize" => {
                    // The same reasoning as `usize` applies for `isize`.
                    let value: TokenStream =
                        TokenTree::from(Literal::i128_suffixed(gen_random())).into();
                    let type_cast: TokenStream = [value, ident("as"), ident("isize")]
                        .iter()
                        .cloned()
                        .collect();
                    TokenTree::from(Group::new(Delimiter::Parenthesis, type_cast)).into()
                }
                _ => panic!("invalid integer type arg: `{}`", s),
            }
        }
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            let mut iter = group.stream().into_iter();
            match (&iter.next(), &iter.next(), &iter.next(), &iter.next()) {
                (
                    Some(TokenTree::Ident(ident)),
                    Some(TokenTree::Punct(punct)),
                    Some(TokenTree::Literal(literal)),
                    None,
                ) if ident.to_string().as_str() == "u8" && punct.as_char() == ';' => {
                    let Ok(len) = literal.to_string().parse() else {
                        panic!("invalid array length: `{}`", literal);
                    };
                    let mut random_bytes = vec![0; len];
                    gen_random_bytes(&mut random_bytes);
                    let array_parts: TokenStream = random_bytes
                        .into_iter()
                        .flat_map(|byte| {
                            let val = TokenTree::from(Literal::u8_suffixed(byte));
                            let comma = TokenTree::from(Punct::new(',', Spacing::Alone));
                            once(val).chain(once(comma))
                        })
                        .collect();
                    TokenTree::from(Group::new(Delimiter::Bracket, array_parts)).into()
                }
                _ => panic!("invalid array type arg: `{}`", tt),
            }
        }
        _ => {
            panic!("invalid type arg: `{}`", tt);
        }
    };

    if let Some(tt) = iter.next() {
        panic!("invalid trailing token tree: `{}`", tt);
    };

    result
}
