#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn const_random_u8(_input: TokenStream) -> TokenStream {
    let value: u8 = rand::random();
    let out_str = format!("0x{:x}",value);
    out_str.parse().unwrap()
}

#[proc_macro]
pub fn const_random_u16(_input: TokenStream) -> TokenStream {
    let value: u16 = rand::random();
    let out_str = format!("0x{:x}",value);
    out_str.parse().unwrap()
}

#[proc_macro]
pub fn const_random_u32(_input: TokenStream) -> TokenStream {
    let value: u32 = rand::random();
    let out_str = format!("0x{:x}",value);
    out_str.parse().unwrap()
}

#[proc_macro]
pub fn const_random_u64(_input: TokenStream) -> TokenStream {
    let value: u64 = rand::random();
    let out_str = format!("0x{:x}",value);
    out_str.parse().unwrap()
}

#[proc_macro]
pub fn const_random_u128(_input: TokenStream) -> TokenStream {
    let value: u128 = rand::random();
    let out_str = format!("0x{:x}",value);
    out_str.parse().unwrap()
}