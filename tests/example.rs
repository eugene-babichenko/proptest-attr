use proptest::prelude::*;
use proptest_attr::proptest;

#[proptest(strategy = "0..10u8")]
fn basic_test(_value: u8) -> Result<(), TestCaseError> {
    Ok(())
}
