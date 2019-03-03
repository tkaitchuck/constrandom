extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn const_random_u8(_input: TokenStream) -> TokenStream {
    format!("0x{:x}", rand::random::<u8>()).parse().unwrap()
}

#[proc_macro]
pub fn const_random_u16(_input: TokenStream) -> TokenStream {
    format!("0x{:x}", rand::random::<u16>()).parse().unwrap()
}

#[proc_macro]
pub fn const_random_u32(_input: TokenStream) -> TokenStream {
    format!("0x{:x}", rand::random::<u32>()).parse().unwrap()
}

#[proc_macro]
pub fn const_random_u64(_input: TokenStream) -> TokenStream {
    format!("0x{:x}", rand::random::<u64>()).parse().unwrap()
}

#[proc_macro]
pub fn const_random_u128(_input: TokenStream) -> TokenStream {
    format!("0x{:x}", rand::random::<u128>()).parse().unwrap()
}
