#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01-valid-example.rs");
    t.pass("tests/ui/02-valid-example-mult-args.rs");
    t.compile_fail("tests/ui/03-no-strategy.rs");
    t.compile_fail("tests/ui/04-multiple-strategy.rs");
    t.compile_fail("tests/ui/05-unknown-argument.rs");
    t.compile_fail("tests/ui/06-unknown-argument-2.rs");
    t.compile_fail("tests/ui/07-invalid-strategy-string.rs");
    t.compile_fail("tests/ui/08-strategy-wrong-literal.rs");
    t.compile_fail("tests/ui/09-strategy-invalid-expression.rs");
    t.pass("tests/ui/10-valid-example-mut-pattern.rs");
}
