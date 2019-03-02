#![feature(proc_macro_hygiene)]
use const_random_macro::const_random_u32;

#[test]
fn works() {
    const value: u32 = const_random_u32!();
    assert_ne!(0, value);
}