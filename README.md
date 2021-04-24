# `proptest-attr`

<!-- cargo-sync-readme start -->

An attribute-style replacement for [`proptest! {}`](proptest-link) macro.

The goal for this macro is to provide `proptest` users with a simple way of writing tests that
are valid Rust code (unlike using `proptest! { ... }`). This allows to easily use tools like
`rustfmt`, `cargo-clippy`, and so on on your property tests. This would be impossible with the
default macro.

## Usage

```rust
use proptest::prelude::*;
use proptest_attr::proptest;

#[proptest(strategy = "0..=10u8")]
#[test]
fn example_test(value: u8) -> prop::test_runner::TestCaseResult {
    // do your tests...
    Ok(())
}
```

Note a few things here:

* You still need to import the `proptest` prelude.
* A `Strategy` is provided as an attribute argument. It should be a valid Rust expression
  enclosed in quotes.
* The test function takes the value type produced by `Strategy` and returns `TestCaseResult`.

Compare this to the default `proptest! {}`:

```rust
use proptest::prelude::*;

proptest! {
    fn example_test(value in 0..=10u8) {
        // do your tests...
    }
}
```

While the latter is less verbose, it is not valid Rust code, which complicates integration with
the development tools or makes it impossible at all depending on a tool.

### Multiple values

The following `proptest! {}` invocaton:

```rust
use proptest::prelude::*;

proptest! {
    fn example_test(a in 0..=10u8, b in 10..100u32) {
        // do your tests...
    }
}
```

Will be converted to the following code:

```rust
use proptest::prelude::*;
use proptest_attr::proptest;

#[proptest(strategy = "(0..=10u8, 10..100u32")]
#[test]
fn example_test(a: u8, b: u32) -> prop::test_runner::TestCaseResult {
    // do your tests...
    Ok(())
}
```

Note that while you are able to specify multiple arguments to your test function, you are
required to define strategy as a tuple of respective arguments for such case.

## `no_std` support

Aside from `proptest` this macro only uses the `core` library. When `proptest` is configured
correctly the generated code is `no_std`-compatible.

[proptest-link]: https://altsysrq.github.io/rustdoc/proptest/latest/proptest/macro.proptest.html

<!-- cargo-sync-readme end -->
