use self::hash::Hasher;
use proc_macro::Span;
use std::env;
use std::hash::{Hash, Hasher as _};

#[cfg(feature = "sha2")]
mod hash {
    use sha2::Digest;

    pub struct Hasher(sha2::Sha256);

    impl Hasher {
        pub fn new() -> Self {
            Hasher(Digest::new())
        }

        fn finalize(&self) -> [u8; 32] {
            self.0.clone().finalize().into()
        }

        pub fn finish_wide(self) -> u128 {
            let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, ..] = self.finalize();
            u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
        }
    }

    impl std::hash::Hasher for Hasher {
        fn finish(&self) -> u64 {
            let [a, b, c, d, e, f, g, h, ..] = self.finalize();
            u64::from_ne_bytes([a, b, c, d, e, f, g, h])
        }

        fn write(&mut self, bytes: &[u8]) {
            self.0.update(bytes);
        }
    }
}

#[cfg(not(feature = "sha2"))]
mod hash {
    pub use std::collections::hash_map::DefaultHasher as Hasher;
}

fn seed() -> impl Hash {
    env::var_os("CONST_RANDOM_SEED")
}

pub(crate) fn gen_random<T: Random>() -> T {
    Random::random()
}

pub(crate) trait Random {
    fn random() -> Self;
}

fn hash_stuff() -> Hasher {
    let span = Span::call_site();
    let mut hasher = Hasher::new();
    seed().hash(&mut hasher);
    span.source_file().path().hash(&mut hasher);
    span.start().line.hash(&mut hasher);
    span.start().column.hash(&mut hasher);
    hasher
}

impl Random for u64 {
    fn random() -> Self {
        hash_stuff().finish()
    }
}

impl Random for u128 {
    #[cfg(feature = "sha2")]
    fn random() -> Self {
        hash_stuff().finish_wide()
    }

    #[cfg(not(feature = "sha2"))]
    fn random() -> Self {
        let [a, b, c, d, e, f, g, h] = u64::random().to_ne_bytes();

        // Hash the same stuff in a different order.
        let span = Span::call_site();
        let mut hasher = Hasher::new();
        span.start().column.hash(&mut hasher);
        span.start().line.hash(&mut hasher);
        span.source_file().path().hash(&mut hasher);
        seed().hash(&mut hasher);
        let [i, j, k, l, m, n, o, p] = hasher.finish().to_ne_bytes();

        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }
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
