#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Provides shared test assertions for zero-field tuple variants.
// Assumes the including file defines:
// 1. `EnumWithZeroFieldTuple` enum with `VariantZeroDefault` and `VariantZeroScalar`.
// 2. Static methods `variant_zero_default()` and `variant_zero_scalar()` on `EnumWithZeroFieldTuple`.
// 3. Standalone functions `standalone_variant_zero_default()` and `standalone_variant_zero_scalar()`.

#[ test ]
fn test_zero_field_default_static_constructor() {
    let got = EnumWithZeroFieldTuple::variant_zero_default();
    let expected = EnumWithZeroFieldTuple::VariantZeroDefault();
    assert_eq!(got, expected);
}

#[ test ]
fn test_zero_field_scalar_static_constructor() {
    let got = EnumWithZeroFieldTuple::variant_zero_scalar();
    let expected = EnumWithZeroFieldTuple::VariantZeroScalar();
    assert_eq!(got, expected);
}

// #[ test ]
// fn test_zero_field_default_standalone_constructor() {
//     let got = variant_zero_default(); // Name matches derive output
//     let expected = EnumWithZeroFieldTuple::VariantZeroDefault();
//     assert_eq!(got, expected);
// }

// #[ test ]
// fn test_zero_field_scalar_standalone_constructor() {
//     let got = variant_zero_scalar(); // Name matches derive output
//     let expected = EnumWithZeroFieldTuple::VariantZeroScalar();
//     assert_eq!(got, expected);
// }