use proptest::prelude::*;
use proptest_attr::proptest;

fn main() {
    basic_test();
}

#[proptest(strategy = "(0..10u8, 10..100u32)")]
fn basic_test(a: u8, mut _b: u32) -> Result<(), TestCaseError> {
    _b += a as u32;
    Ok(())
}
