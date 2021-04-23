use proptest::prelude::*;
use proptest_attr::proptest;

fn main() {
    basic_test();
}

#[proptest(strategy = 42)]
fn basic_test(_value: u8) -> Result<(), TestCaseError> {
    Ok(())
}
