use const_random::const_random;

#[test]
fn works() {
    const value1: u32 = const_random!(u32);
    const value2: u32 = const_random!(u32);
    assert_ne!(0, value1);
    assert_ne!(0, value2);
    assert_ne!(value1, value2);
}