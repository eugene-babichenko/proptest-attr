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
//! Compare this to the default `proptest! {}`:
//!
//! ```rust
//! use proptest::prelude::*;
//!
//! proptest! {
//!     fn example_test(value in 0..=10u8) {
//!         // do your tests...
//!     }
//! }
//! ```
//!
//! While the latter is less verbose, it is not valid Rust code, which complicates integration with
//! the development tools or makes it impossible at all depending on a tool.
//!
//! ### Multiple values
//!
//! The following `proptest! {}` invocaton:
//!
//! ```rust
//! use proptest::prelude::*;
//!
//! proptest! {
//!     fn example_test(a in 0..=10u8, b in 10..100u32) {
//!         // do your tests...
//!     }
//! }
//! ```
//!
//! Will be converted to the following code:
//!
//! ```rust
//! use proptest::prelude::*;
//! use proptest_attr::proptest;
//!
//! #[proptest(strategy = "(0..=10u8, 10..100u32)")]
//! #[test]
//! fn example_test(a: u8, b: u32) -> prop::test_runner::TestCaseResult {
//!     // do your tests...
//!     Ok(())
//! }
//! ```
//!
//! Note that while you are able to specify multiple arguments to your test function, you are
//! required to define strategy as a tuple of respective arguments for such case.
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
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, AttributeArgs, Error, Expr, FnArg,
    ItemFn, Lit, Meta, MetaNameValue, NestedMeta, PatType, ReturnType, Signature,
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

    // Convert multiple inputs to a tuple for use in the test runner
    let mut inner_inputs_pats = Vec::new();
    let mut inner_inputs_types = Vec::new();
    for arg in input.sig.inputs.into_iter() {
        let arg_span = arg.span();
        if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = arg {
            // We need to collect arguments into a tuple pattern, and patterns do not allow to use
            // attributes.
            if !attrs.is_empty() {
                return Error::new_spanned(
                    &attrs[0],
                    "proptest-attr does not allow to have attributes for function arguments",
                )
                .into_compile_error()
                .into();
            }

            inner_inputs_pats.push(pat);
            inner_inputs_types.push(ty);
        } else {
            return Error::new(
                arg_span,
                "receiver arguments are invalid in the testing context",
            )
            .into_compile_error()
            .into();
        }
    }

    let inner_inputs = if inner_inputs_pats.is_empty() {
        quote! {}
    } else if inner_inputs_pats.len() == 1 {
        let pat = &inner_inputs_pats[0];
        let ty = &inner_inputs_types[0];
        quote! { #pat: #ty }
    } else {
        quote! { ( #(#inner_inputs_pats),* ): ( #(#inner_inputs_types),* ) }
    };

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
