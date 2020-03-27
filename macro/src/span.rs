use proc_macro::Span;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(crate) fn gen_random<T: Random>() -> T {
    Random::random()
}

pub(crate) trait Random {
    fn random() -> Self;
}

impl Random for u8 {
    fn random() -> Self {
        u64::random() as u8
    }
}

impl Random for u16 {
    fn random() -> Self {
        u64::random() as u16
    }
}

impl Random for u32 {
    fn random() -> Self {
        u64::random() as u32
    }
}

impl Random for u64 {
    fn random() -> Self {
        let span = Span::call_site();
        let mut hasher = DefaultHasher::new();
        span.source_file().path().hash(&mut hasher);
        span.start().line.hash(&mut hasher);
        span.start().column.hash(&mut hasher);
        hasher.finish()
    }
}

impl Random for u128 {
    fn random() -> Self {
        let lo = u64::random();

        // Hash the same stuff in a different order.
        let span = Span::call_site();
        let mut hasher = DefaultHasher::new();
        span.start().column.hash(&mut hasher);
        span.start().line.hash(&mut hasher);
        span.source_file().path().hash(&mut hasher);
        let hi = hasher.finish();

        ((hi as u128) << 8) + lo as u128
    }
}

impl Random for i8 {
    fn random() -> Self {
        i64::random() as i8
    }
}

impl Random for i16 {
    fn random() -> Self {
        i64::random() as i16
    }
}

impl Random for i32 {
    fn random() -> Self {
        i64::random() as i32
    }
}

impl Random for i64 {
    fn random() -> Self {
        u64::random() as i64
    }
}

impl Random for i128 {
    fn random() -> Self {
        u128::random() as i128
    }
}
