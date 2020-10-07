use getrandom;
use proc_macro::*;
use proc_macro_hack::proc_macro_hack;

// Ideally we would use the proper interface for this through the rand crate,
// but due to https://github.com/rust-lang/cargo/issues/5730 this leads to
// issues for no_std crates that try to use rand themselves. So instead we skip
// rand and generate random bytes straight from the OS.
fn gen_random<T>() -> T {
    let mut out = [0u8; 16];
    getrandom::getrandom(&mut out).unwrap();
    unsafe { std::mem::transmute_copy(&out) }
}

/// Create a TokenStream of an identifier out of a string
fn ident(ident: &str) -> TokenStream {
    TokenTree::from(Ident::new(ident, Span::call_site())).into()
}

#[proc_macro_hack]
pub fn const_random(input: TokenStream) -> TokenStream {
    match &input.to_string()[..] {
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
            let value: TokenStream = TokenTree::from(Literal::u128_suffixed(gen_random())).into();
            let type_cast: TokenStream = [value, ident("as"), ident("usize")]
                .iter()
                .cloned()
                .collect();
            TokenTree::from(Group::new(Delimiter::Parenthesis, type_cast)).into()
        }
        "isize" => {
            let value: TokenStream = TokenTree::from(Literal::i128_suffixed(gen_random())).into();
            let type_cast: TokenStream = [value, ident("as"), ident("isize")]
                .iter()
                .cloned()
                .collect();
            TokenTree::from(Group::new(Delimiter::Parenthesis, type_cast)).into()
        }
        _ => panic!("Invalid integer type"),
    }
}
