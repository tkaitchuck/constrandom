#![feature(proc_macro_hygiene)]
#[macro_use]
use const_random::const_random;

#[test]
fn works() {
    const value: u32 = const_random!(u32);
    assert_ne!(0, value);
}