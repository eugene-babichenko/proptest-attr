use proptest::prelude::*;
use proptest_attr::proptest;

fn main() {
    basic_test();
}

#[proptest(strategy = "0..10u8")]
fn basic_test(_value: u32) -> Result<(), TestCaseError> {
    Ok(())
}