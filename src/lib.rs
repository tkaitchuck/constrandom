#![no_std]
use proc_macro_hack::proc_macro_hack;

/// # Random constants
/// Allows you to insert random constants into your code that will be auto-generated at compile time.
/// A new value will be generated every time the relevent file is re-built.
/// # Example
/// ```
/// use const_random::const_random  ;
/// const MY_RANDOM_NUMBER: u32 = const_random!(u32);
/// ```
///
/// The following types are supported u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, and isize. 
///
#[proc_macro_hack(fake_call_site, only_hack_old_rustc)]
pub use const_random_macro::const_random;
