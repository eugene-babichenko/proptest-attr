//! An attribute-style replacement for [`proptest! {}`](proptest-link) macro.
//!
//! The goal for this macro is to provide `proptest` users with a simple way of writing tests that
//! are valid Rust code (unlike using `proptest! { ... }`). This allows to easily use tools like
//! `rustfmt`, `cargo-clippy`, and so on on your property tests. This would be impossible with the
//! default macro.
//!
//! ## Usage
//!
//! ```rust
//! use proptest::prelude::*;
//! use proptest_attr::proptest;
//!
//! #[proptest(strategy = "0..=10u8")]
//! #[test]
//! fn example_test(value: u8) -> prop::test_runner::TestCaseResult {
//!     // do your tests...
//!     Ok(())
//! }
//! ```
//!
//! Note a few things here:
//!
//! * You still need to import the `proptest` prelude.
//! * A `Strategy` is provided as an attribute argument. It should be a valid Rust expression
//!   enclosed in quotes.
//! * The test function takes the value type produced by `Strategy` and returns `TestCaseResult`.
//!
//! ## `no_std` support
//!
//! Aside from `proptest` this macro only uses the `core` library. When `proptest` is configured
//! correctly the generated code is `no_std`-compatible.
//!
//! [proptest-link]: https://altsysrq.github.io/rustdoc/proptest/latest/proptest/macro.proptest.html

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, punctuated::Punctuated, AttributeArgs, Error, Expr, ItemFn, Lit, Meta,
    MetaNameValue, NestedMeta, ReturnType, Signature,
};

#[proc_macro_attribute]
pub fn proptest(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse attribute arguments and search for `strategy = "..."`
    let args = parse_macro_input!(args as AttributeArgs);

    let mut maybe_strategy = None;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = &arg {
            if path.is_ident("strategy") && maybe_strategy.is_none() {
                if let Lit::Str(strategy_str) = lit {
                    maybe_strategy = match strategy_str.parse::<Expr>() {
                        Ok(strategy) => Some((strategy, strategy_str.span())),
                        Err(err) => {
                            return Error::new_spanned(
                                strategy_str,
                                format!("strategy is not a valid Rust expression: {}", err),
                            )
                            .into_compile_error()
                            .into()
                        }
                    };
                } else {
                    return Error::new_spanned(lit, "invalid strategy: must be a string literal")
                        .into_compile_error()
                        .into();
                }
            } else if maybe_strategy.is_some() {
                return Error::new_spanned(arg, "multiple strategies are not allowed")
                    .into_compile_error()
                    .into();
            } else {
                return Error::new_spanned(arg, "unknown argument")
                    .into_compile_error()
                    .into();
            }
        } else {
            return Error::new_spanned(arg, "unknown argument")
                .into_compile_error()
                .into();
        }
    }

    let strategy = match maybe_strategy {
        Some((strategy, strategy_span)) => quote_spanned!(strategy_span=> #strategy),
        None => {
            let output = quote! { compile_error!("no strategy specified for this proptest"); };
            return output.into();
        }
    };

    let input = parse_macro_input!(input as ItemFn);

    let attrs = input.attrs;
    let vis = input.vis;

    // Make a signature for the test function
    let test_function_signature = Signature {
        // No inputs or outputs in test functions
        inputs: Punctuated::new(),
        output: ReturnType::Default,
        ..input.sig
    };

    let inner_inputs = input.sig.inputs;
    let inner_output = input.sig.output;
    let inner_stmts = input.block.stmts;

    let output = quote! {
        #(#attrs)*
        #vis #test_function_signature {
            let strategy = #strategy;
            let runner_settings = ::core::default::Default::default();
            let mut runner = ::proptest::test_runner::TestRunner::new(runner_settings);
            let result = runner.run(&strategy, |#inner_inputs| #inner_output {
                #(#inner_stmts)*
            });
            result.unwrap();
        }
    };

    output.into()
}
