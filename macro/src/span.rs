use proc_macro::Span;
use std::{collections::HashSet, option_env, sync::Mutex};

use lazy_static::lazy_static;
use tiny_keccak::{Hasher, Sha3};

lazy_static! {
    static ref SEED: Vec<u8> = {
        if let Some(value) = option_env!("CONST_RANDOM_SEED") {
            value.as_bytes().to_vec()
        } else {
            let mut value = [0u8; 32];
            getrandom::getrandom(&mut value).unwrap();
            value.to_vec()
        }
    };
    static ref EXISTINGS_U8: Mutex<HashSet<u8>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_U16: Mutex<HashSet<u16>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_U32: Mutex<HashSet<u32>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_U64: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_U128: Mutex<HashSet<u128>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_I8: Mutex<HashSet<i8>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_I16: Mutex<HashSet<i16>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_I32: Mutex<HashSet<i32>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_I64: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
    static ref EXISTINGS_I128: Mutex<HashSet<i128>> = Mutex::new(HashSet::new());
}

pub(crate) fn gen_random<T: Random>() -> T {
    Random::random()
}

pub(crate) fn gen_random_unique<T: Random>() -> T {
    Random::random_unique()
}

pub(crate) trait Random {
    fn random() -> Self;
    fn random_unique() -> Self;
}

fn hash_stuff() -> impl Hasher {
    let span = Span::call_site();
    let mut hasher = Sha3::v256();
    hasher.update(&*SEED);
    hasher.update(&format!("{:?}", span).as_bytes());
    hasher
}

impl Random for u64 {
    fn random() -> Self {
        let mut output = [0; 8];
        hash_stuff().finalize(&mut output);
        Self::from_ne_bytes(output)
    }

    fn random_unique() -> Self {
        let mut output = [0; 8];
        let mut existings = EXISTINGS_U64.lock().unwrap();
        loop {
            hash_stuff().finalize(&mut output);
            let result = Self::from_ne_bytes(output);
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for u128 {
    fn random() -> Self {
        let mut output = [0; 16];
        hash_stuff().finalize(&mut output);
        Self::from_ne_bytes(output)
    }

    fn random_unique() -> Self {
        let mut output = [0; 16];
        let mut existings = EXISTINGS_U128.lock().unwrap();
        loop {
            hash_stuff().finalize(&mut output);
            let result = Self::from_ne_bytes(output);
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for u8 {
    fn random() -> Self {
        u64::random() as u8
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_U8.lock().unwrap();
        loop {
            let result = u64::random() as u8;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for u16 {
    fn random() -> Self {
        u64::random() as u16
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_U16.lock().unwrap();
        loop {
            let result = u64::random() as u16;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for u32 {
    fn random() -> Self {
        u64::random() as u32
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_U32.lock().unwrap();
        loop {
            let result = u64::random() as u32;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for i8 {
    fn random() -> Self {
        i64::random() as i8
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_I8.lock().unwrap();
        loop {
            let result = i64::random() as i8;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for i16 {
    fn random() -> Self {
        i64::random() as i16
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_I16.lock().unwrap();
        loop {
            let result = i64::random() as i16;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for i32 {
    fn random() -> Self {
        i64::random() as i32
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_I32.lock().unwrap();
        loop {
            let result = i64::random() as i32;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for i64 {
    fn random() -> Self {
        u64::random() as i64
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_I64.lock().unwrap();
        loop {
            let result = u64::random() as i64;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}

impl Random for i128 {
    fn random() -> Self {
        u128::random() as i128
    }

    fn random_unique() -> Self {
        let mut existings = EXISTINGS_I128.lock().unwrap();
        loop {
            let result = u128::random() as i128;
            if existings.insert(result) {
                return result;
            } else {
                continue;
            }
        }
    }
}
