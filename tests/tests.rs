use const_random::const_random;

#[test]
fn u32() {
    const VALUE1: u32 = const_random!(u32);
    const VALUE2: u32 = const_random!(u32);
    assert_ne!(0, VALUE1);
    assert_ne!(0, VALUE2);
    assert_ne!(VALUE1, VALUE2);
}

#[test]
fn i64() {
    const VALUE1: i64 = const_random!(i64);
    const VALUE2: i64 = const_random!(i64);
    assert_ne!(0, VALUE1);
    assert_ne!(0, VALUE2);
    assert_ne!(VALUE1, VALUE2);
}


#[test]
fn u128() {
    const VALUE1: u128 = const_random!(u128);
    const VALUE2: u128 = const_random!(u128);
    assert_ne!(0, VALUE1);
    assert_ne!(0, VALUE2);
    assert_ne!(VALUE1, VALUE2);
}
