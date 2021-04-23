use proptest::prelude::*;
use proptest_attr::proptest;

fn main() {
    basic_test();
}

#[proptest(strategy = "(0..10u8, 10..100u32)")]
fn basic_test((a, b): (u8, u32)) -> Result<(), TestCaseError> {
    let _c = a as u32 + b;
    Ok(())
}
