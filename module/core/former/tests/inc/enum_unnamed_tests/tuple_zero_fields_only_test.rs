// Purpose: Provides shared test assertions for zero-field tuple variants.
// Assumes the including file defines:
// 1. `ZeroTuple` enum with `Variant()` and its static/standalone constructors.
// 2. `ZeroTupleScalar` enum with `Variant()` and its static/standalone constructors.

#[test]
fn test_zero_tuple_default_static() {
    // use super::*; // Items should be in scope from the including file
    let got = ZeroTuple::variant();
    let expected = ZeroTuple::Variant();
    assert_eq!(got, expected);
}

#[test]
fn test_zero_tuple_default_standalone() {
    // use super::*; // Items should be in scope from the including file
    let got = zero_tuple_variant();
    let expected = ZeroTuple::Variant();
    assert_eq!(got, expected);
}

#[test]
fn test_zero_tuple_scalar_static() {
    // use super::*; // Items should be in scope from the including file
    let got = ZeroTupleScalar::variant();
    let expected = ZeroTupleScalar::Variant();
    assert_eq!(got, expected);
}

#[test]
fn test_zero_tuple_scalar_standalone() {
    // use super::*; // Items should be in scope from the including file
    let got = zero_tuple_scalar_variant();
    let expected = ZeroTupleScalar::Variant();
    assert_eq!(got, expected);
}