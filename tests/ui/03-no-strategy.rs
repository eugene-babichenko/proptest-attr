use proptest_attr::proptest;

fn main() {
    basic_test();
}

#[proptest]
fn basic_test(_value: u8) -> Result<(), TestCaseError> {
    Ok(())
}
