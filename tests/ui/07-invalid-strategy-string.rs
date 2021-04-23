use proptest::prelude::*;
use proptest_attr::proptest;

fn main() {
    basic_test();
}

#[proptest(strategy = "123\\sdfq;;@")]
fn basic_test(_value: u8) -> Result<(), TestCaseError> {
    Ok(())
}
