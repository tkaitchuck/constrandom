extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use rand::rngs::OsRng;
use rand::Rng;

#[proc_macro_hack]
pub fn const_random(input: TokenStream) -> TokenStream {
    match &input.to_string()[..] {
        "u8" => format!("0x{:x}", OsRng.gen::<u8>()).parse().unwrap(),
        "u16" => format!("0x{:x}", OsRng.gen::<u16>()).parse().unwrap(),
        "u32" => format!("0x{:x}", OsRng.gen::<u32>()).parse().unwrap(),
        "u64" => format!("0x{:x}", OsRng.gen::<u64>()).parse().unwrap(),
        "u128" => format!("0x{:x}", OsRng.gen::<u128>()).parse().unwrap(),
        "i8" => format!("0x{:x}", OsRng.gen::<i8>()).parse().unwrap(),
        "i16" => format!("0x{:x}", OsRng.gen::<i16>()).parse().unwrap(),
        "i32" => format!("0x{:x}", OsRng.gen::<i32>()).parse().unwrap(),
        "i64" => format!("0x{:x}", OsRng.gen::<i64>()).parse().unwrap(),
        "i128" => format!("0x{:x}", OsRng.gen::<i128>()).parse().unwrap(),
        _ => panic!("Invalid integer type"),
    }

}
