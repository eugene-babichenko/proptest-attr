use proptest::prelude::*;
use proptest_attr::proptest;

fn main() {
    basic_test();
}

#[proptest(helloworld = "abc")]
fn basic_test(_value: u8) -> Result<(), TestCaseError> {
    Ok(())
}
