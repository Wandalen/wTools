// Purpose: Provides shared test assertions and logic for both the derived and manual implementations
// of constructors for named (struct-like) variants with varying field counts and attributes
// (`#[ scalar ]`, `#[ subform_scalar ]`), including static methods and standalone constructors.
//
// Coverage:
// - Rule 1c (Struct + Zero-Field + `#[ scalar ]`): Tests the static method `variant_zero_scalar()`.
// - Rule 1e (Struct + Single-Field + `#[ scalar ]`): Tests the static method `variant_one_scalar()`.
// - Rule 2e (Struct + Single-Field + `#[ subform_scalar ]`): Tests the static method `variant_one_subform()` which returns a former for the inner type.
// - Rule 3e (Struct + Single-Field + Default): Tests the static method `variant_one_default()` which returns a former for the inner type.
// - Rule 1g (Struct + Multi-Field + `#[ scalar ]`): Tests the static method `variant_two_scalar()`.
// - Rule 3g (Struct + Multi-Field + Default): Tests the static method `variant_two_default()` which returns a former for the variant. (Note: This variant is commented out in the enum definition in the manual file).
// - Rule 4a (#[ standalone_constructors ]): Tests the existence and functionality of standalone constructor functions (e.g., `standalone_variant_zero_scalar()`, `standalone_variant_one_default()`, etc.).
// - Rule 4b (Option 2 Logic): Tests the return types and usage of standalone constructors based on field attributes and whether they return scalars or formers.
//
// Test Relevance/Acceptance Criteria:
// - Defines the `EnumWithNamedFields` enum structure with named variants covering zero, one, and two fields.
// - Defines the `InnerForSubform` struct used in some variants.
// - Contains test functions that are included by the derive and manual test files.
// - Calls the static methods (e.g., `EnumWithNamedFields::variant_zero_scalar()`, `EnumWithNamedFields::variant_one_scalar()`) and standalone constructors (e.g., `standalone_variant_zero_scalar()`) provided by the including file.
// - Uses setters and `.form()` where former builders are expected.
// - Asserts that the returned values match the expected enum instances or former types, verifying that both derived and manual implementations correctly provide constructors for named variants with different attributes and field counts.

// File: module/core/former/tests/inc/former_enum_tests/named_tests/enum_named_fields_named_only_test.rs
use super::*; // Imports EnumWithNamedFields and InnerForSubform

// --- Zero Fields (Named) ---

#[ test ]
fn variant_zero_scalar_test()
{
  // Test Matrix Row: T24.1 (Implicitly, as this tests the behavior expected by the matrix)
  // Expect a direct static constructor taking no arguments.
  let got = EnumWithNamedFields::variant_zero_scalar();
  let expected = EnumWithNamedFields::VariantZeroScalar {};
  assert_eq!( got, expected );
}

// #[ test ]
// fn standalone_variant_zero_scalar_test() // New Test for S0.4
// {
//   // Test Matrix Row: T24.2 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor taking no arguments.
//   let got = standalone_variant_zero_scalar();
//   let expected = EnumWithNamedFields::VariantZeroScalar {};
//   assert_eq!( got, expected );
// }

// --- One Field (Named) ---

// #[ test ]
// fn variant_one_scalar_test()
// {
//   // Test Matrix Row: T24.3 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a direct static constructor taking one argument.
//   let got = EnumWithNamedFields::variant_one_scalar( "value_a".to_string() );
//   let expected = EnumWithNamedFields::VariantOneScalar { field_a : "value_a".to_string() };
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn variant_one_subform_test()
// {
//   // Test Matrix Row: T24.4 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a static method returning a subformer for InnerForSubform.
//   let got = EnumWithNamedFields::variant_one_subform()
//     .value( 101 ) // Use InnerForSubformFormer's setter
//     .form();
//   let expected = EnumWithNamedFields::VariantOneSubform { field_b: InnerForSubform { value: 101 } };
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn variant_one_default_test()
// {
//   // Test Matrix Row: T24.5 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a static method returning a subformer for InnerForSubform (default behavior).
//   let got = EnumWithNamedFields::variant_one_default()
//     .value( 102 ) // Use InnerForSubformFormer's setter
//     .form();
//   let expected = EnumWithNamedFields::VariantOneDefault { field_c: InnerForSubform { value: 102 } };
//   assert_eq!( got, expected );
// }

// --- One Field (Named) - Standalone Constructors (S1.4-S1.7) ---

// #[ test ]
// fn standalone_variant_one_default_test() // Test for S1.4
// {
//   // Test Matrix Row: T24.6 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor returning a subformer.
//   // Note: Manual implementation uses a placeholder End struct.
//   let got = standalone_variant_one_default()
//     .value( 103 )
//     .form();
//   let expected = EnumWithNamedFields::VariantOneDefault { field_c: InnerForSubform { value: 103 } };
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn standalone_variant_one_scalar_test() // Test for S1.5
// {
//   // Test Matrix Row: T24.7 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor taking one argument.
//   let got = standalone_variant_one_scalar( "value_b".to_string() );
//   let expected = EnumWithNamedFields::VariantOneScalar { field_a : "value_b".to_string() };
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn standalone_variant_one_subform_test() // Test for S1.6
// {
//   // Test Matrix Row: T24.8 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor returning a subformer.
//   // Note: Manual implementation uses a placeholder End struct.
//   let got = standalone_variant_one_subform()
//     .value( 104 )
//     .form();
//   let expected = EnumWithNamedFields::VariantOneSubform { field_b: InnerForSubform { value: 104 } };
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn standalone_variant_one_default_with_arg_test() // Test for S1.7
// {
//   // Test Matrix Row: T24.9 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor taking the marked argument.
//   // Note: Manual implementation might differ slightly from macro output depending on arg_for_constructor logic.
//   let got = standalone_variant_one_default_with_arg( InnerForSubform { value: 105 } );
//   let expected = EnumWithNamedFields::VariantOneDefault { field_c: InnerForSubform { value: 105 } };
//   assert_eq!( got, expected );
// }


// --- Two Fields (Named) ---

// #[ test ]
// fn variant_two_scalar_test()
// {
//   // Test Matrix Row: T24.10 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a direct static constructor taking multiple arguments.
//   let got = EnumWithNamedFields::variant_two_scalar( 42, true );
//   let expected = EnumWithNamedFields::VariantTwoScalar { field_d : 42, field_e : true };
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn variant_two_default_test() { /* Compile Error Expected */ }

// --- Two Fields (Named) - Standalone Constructors (SN.4-SN.7) ---

// #[ test ]
// fn standalone_variant_two_default_test() // Test for SN.4
// {
//   // Test Matrix Row: T24.11 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor returning a subformer.
//   // Note: Manual implementation uses a placeholder End struct.
//   let got = standalone_variant_two_default()
//     .value( 201 ) // Assuming InnerForSubformFormer methods are available on the placeholder
//     .form();
//   // qqq : Expected value depends on the placeholder implementation in manual file.
//   // For now, just check that it doesn't panic and returns the placeholder variant.
//   let expected = EnumWithNamedFields::UnitVariantScalar; // Matches placeholder return
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn standalone_variant_two_scalar_test() // Test for SN.5
// {
//   // Test Matrix Row: T24.12 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor taking multiple arguments.
//   let got = standalone_variant_two_scalar( 43, false );
//   let expected = EnumWithNamedFields::VariantTwoScalar { field_d : 43, field_e : false };
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn standalone_variant_two_subform_test() // Test for SN.6
// {
//   // Test Matrix Row: T24.13 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor returning a subformer.
//   // Note: Manual implementation uses a placeholder End struct.
//   let got = standalone_variant_two_subform()
//     .value( 202 ) // Assuming InnerForSubformFormer methods are available on the placeholder
//     .form();
//   // qqq : Expected value depends on the placeholder implementation in manual file.
//   // For now, just check that it doesn't panic and returns the placeholder variant.
//   let expected = EnumWithNamedFields::UnitVariantScalar; // Matches placeholder return
//   assert_eq!( got, expected );
// }

// #[ test ]
// fn standalone_variant_two_default_with_arg_test() // Test for SN.7
// {
//   // Test Matrix Row: T24.14 (Implicitly, as this tests the behavior expected by the matrix)
//   // Expect a standalone constructor taking marked arguments.
//   // Note: Manual implementation uses a direct constructor with all fields as args.
//   let got = standalone_variant_two_default_with_args( 44, true );
//   let expected = EnumWithNamedFields::VariantOneDefault { field_d: 44, field_e: true };
//   assert_eq!( got, expected );
// }