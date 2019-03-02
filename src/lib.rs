#![feature(proc_macro_hygiene)]
use const_random_macro;

/// # Random constants
/// Allows you to insert random constants into your code that will be auto-generated at compile time.
/// A new value will be generated every time the relevent file is re-built.
/// # Example
/// ```
/// #![feature(proc_macro_hygiene)]
/// use const_random::const_random  ;
/// const MY_RANDOM_NUMBER: u32 = const_random!(u32);
/// ```
/// Note that the `#![feature(proc_macro_hygiene)]` must be enabled on your crate for the macro to work.
///
/// The following types are supported u8, u16, u32, u64, and u128
///
#[macro_export]
macro_rules! const_random {
    (u8) => {{ const_random_macro::const_random_u8!() }};
    (u16) => {{ const_random_macro::const_random_u16!() }};
    (u32) => {{ const_random_macro::const_random_u32!() }};
    (u64) => {{ const_random_macro::const_random_u64!() }};
    (u128) => {{ const_random_macro::const_random_u128!() }};
}
