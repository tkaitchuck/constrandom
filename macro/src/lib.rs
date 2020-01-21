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

fn update_prng(state: &mut u128, newdata: usize) {
    *state = state.wrapping_mul(47_026_247_687_942_121_848_144_207_491_837_523_525u128).wrapping_add(newdata as u128);
}

fn finalize(state: &u128) -> u64 {
    let high = (*state >> 64) as u64;
    let low = *state as u64;
    let rot: u32 = (high >> 58) as u32;
    return (high ^ low).rotate_right(rot);
}

#[proc_macro_hack]
pub fn const_val(input: TokenStream) -> TokenStream {
    use std::ffi::OsStr;

    let mut state : u128 = 123456789;
    for x in input.into_iter() {
        let span = x.span();
        if let Some(name) = span.source_file().path().file_name().to_str() {
            for char in name {
                update_prng(&mut state, char);
            }
        }
        update_prng(&mut state, span.start().line);
        update_prng(&mut state, span.start().column);
    }
    match &input.to_string()[..] {
        "u8" => format!("0x{:x}", finalize(&state) as u8).parse().unwrap(),
        "u16" => format!("0x{:x}", finalize(&state) as u16).parse().unwrap(),
        "u32" => format!("0x{:x}", finalize(&state) as u32).parse().unwrap(),
        "u64" => format!("0x{:x}", finalize(&state) as u64).parse().unwrap(),
        "u128" => format!("0x{:x}", state).parse().unwrap(),
        "i8" => format!("0x{:x}", finalize(&state) as i8).parse().unwrap(),
        "i16" => format!("0x{:x}", finalize(&state) as i16).parse().unwrap(),
        "i32" => format!("0x{:x}", finalize(&state) as i32).parse().unwrap(),
        "i64" => format!("0x{:x}", finalize(&state) as i64).parse().unwrap(),
        "i128" => format!("0x{:x}", state as i128).parse().unwrap(),
        _ => panic!("Invalid integer type"),
    }
}